use reqwest::blocking::Client;
use std::{
    fs,
    io,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use threadpool::ThreadPool;

/// 下载一个文件（到内存，下载完成后再写盘），支持自动重试
fn download_file(client: &Client, url: &str, out_path: &Path) -> io::Result<()> {
    for attempt in 1..=5 {
        match client.get(url).send() {
            Ok(resp) if resp.status().is_success() => {
                let bytes = resp.bytes().expect("read response bytes");
                fs::write(out_path, &bytes)?; // 一次性写入最终文件
                return Ok(());
            }
            Ok(resp) => {
                eprintln!(
                    "Failed (status {}) downloading {} (attempt {})",
                    resp.status(),
                    url,
                    attempt
                );
            }
            Err(err) => {
                eprintln!("Error downloading {}: {} (attempt {})", url, err, attempt);
            }
        }
        thread::sleep(Duration::from_secs(2 * attempt)); // 退避重试
    }
    Err(io::Error::new(io::ErrorKind::Other, "Failed after retries"))
}

/// 下载 snapshot 中的所有文件
fn download_epoch(base_url: &str, epoch: u64, out_dir: &Path, num_threads: usize) -> io::Result<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(300))
        .build()
        .unwrap();

    // 1. 下载 epoch manifest
    let manifest_url = format!("{}/epoch_{}/MANIFEST", base_url, epoch);
    let manifest_text = client
        .get(&manifest_url)
        .send()
        .expect("fetch manifest")
        .text()
        .expect("manifest text");

    // 2. 过滤文件，只保留需要的目录
    let files: Vec<String> = manifest_text
        .lines()
        .filter(|line| {
            line.starts_with("checkpoints/")
                || line.starts_with("epochs/")
                || line.starts_with("store/perpetual/")
        })
        .map(|s| s.to_string())
        .collect();

    println!("Epoch {}: {} files to download", epoch, files.len());

    // 3. 并发下载
    let pool = ThreadPool::new(num_threads);
    let total_files = files.len();
    let downloaded = Arc::new(AtomicUsize::new(0));
    let total_bytes = Arc::new(AtomicU64::new(0));

    for file in files {
        let client = client.clone();
        let base_url = base_url.to_string();
        let out_dir = out_dir.to_path_buf();
        let downloaded = Arc::clone(&downloaded);
        let total_bytes = Arc::clone(&total_bytes);

        pool.execute(move || {
            let url = format!("{}/epoch_{}/{}", base_url, epoch, file);
            let out_path = out_dir.join(&file);

            if out_path.exists() {
                downloaded.fetch_add(1, Ordering::Relaxed);
                return;
            }

            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }

            match client.get(&url).send() {
                Ok(resp) if resp.status().is_success() => {
                    let bytes = resp.bytes().expect("read response bytes");
                    let size = bytes.len() as u64;
                    fs::write(&out_path, &bytes).unwrap();

                    downloaded.fetch_add(1, Ordering::Relaxed);
                    total_bytes.fetch_add(size, Ordering::Relaxed);

                    let done = downloaded.load(Ordering::Relaxed);
                    let bytes = total_bytes.load(Ordering::Relaxed);
                    println!(
                        "[{}/{}] Downloaded {} ({} MB total)",
                        done,
                        total_files,
                        file,
                        bytes / (1024 * 1024)
                    );
                }
                Ok(resp) => {
                    eprintln!("Failed {} with status {}", file, resp.status());
                }
                Err(err) => {
                    eprintln!("Error downloading {}: {}", file, err);
                }
            }
        });
    }

    pool.join();
    println!("Epoch {} download complete.", epoch);
    Ok(())
}

fn main() -> io::Result<()> {
    let base_url = "https://db-snapshot.mainnet.sui.io";
    let epoch = 864; // TODO: 换成你需要的 epoch
    let out_dir = PathBuf::from("/var/sui/db/snapshot");
    let num_threads = 32;

    download_epoch(base_url, epoch, &out_dir, num_threads)?;

    Ok(())
}

