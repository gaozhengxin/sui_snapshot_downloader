代码不重要，把下面信息提供给AI，就可以了。

### 查看可用 epoch
```
https://db-snapshot.mainnet.sui.io/MANIFEST
```

result
```
{"available_epochs":[815,816,817,818,819,820,821,822,823,824,825,826,827,828,829,830,831,832,833,834,835,836,837,838,839,840,841,842,843,844,845,846,847,848,849,850,851,852,853,854,855,856,857,858,859,860,861,862,863,864]}
```

### 查看 epoch 中的文件列表，里面是 plain text
```
https://db-snapshot.mainnet.sui.io/epoch_xxx/MANIFEST
```

result
```
checkpoints/000031.sst
checkpoints/000055.sst
checkpoints/000294.sst
checkpoints/000385.sst
......
checkpoints/CURRENT
checkpoints/IDENTITY
checkpoints/LOG
checkpoints/MANIFEST-792689
checkpoints/OPTIONS-791235
checkpoints/OPTIONS-792691
epochs/000010.sst
epochs/000033.sst
epochs/000036.sst
epochs/000045.sst
epochs/000054.sst
epochs/000057.sst
epochs/000060.sst
......
epochs/CURRENT
epochs/MANIFEST-005286
epochs/OPTIONS-005288
indexes/1040395.sst
indexes/1042731.sst
indexes/1049350.sst
indexes/1050721.sst
indexes/1050729.sst
......
store/perpetual/000078.sst
store/perpetual/000807.sst
store/perpetual/000957.sst
store/perpetual/001114.sst
store/perpetual/001260.sst
store/perpetual/001386.sst
store/perpetual/001605.sst
store/perpetual/001780.sst
store/perpetual/001974.sst
......
store/perpetual/CURRENT
store/perpetual/IDENTITY
store/perpetual/LOG.old.1756071409440871
store/perpetual/LOG
store/perpetual/MANIFEST-5967351
store/perpetual/OPTIONS-5967346
store/perpetual/OPTIONS-5967353
```

### 下载里面的文件
```
wget -O epoch_853/store/perpetual/5918611.sst https://db-snapshot.mainnet.sui.io/epoch_853/store/perpetual/5918611.sst
```

---

rpc 节点只需要三个文件夹就够了 checkpoints/ epochs/ store/per/
当前大概 220G
最大文件不超过 200M

