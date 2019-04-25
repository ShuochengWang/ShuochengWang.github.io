---
title: 翻译：Store 1 Billion Files in Alluxio 2.0
date: 2019-04-25 13:28:57
categories: 翻译
tags: Alluxio
---

# 在Alluxio 2.0中存储10亿个文件

Alluxio技术博客翻译系列——Store 1 Billion Files in Alluxio 2.0

[原文地址](http://www.alluxio.com/blog/store-1-billion-files-in-alluxio-20)

本文也会发布在微信公众号“**Alluxio**”上，欢迎关注该微信公众号。

---



Alluxio 2.0旨在能够在单个文件系统命名空间中支持10亿个文件。命名空间扩展对Alluxio至关重要，原因如下：

1. Alluxio提供了可以挂载多个存储系统的单个命名空间。因此，Alluxio命名空间的大小是所有已挂载存储的大小的总和。
2. 对象存储越来越受欢迎，与HDFS等文件系统相比，对象存储通常会容纳更多的小文件。

在Alluxio 1.x中，Alluxio命名空间在实践中仅能容纳约2亿个文件。由于Alluxio master的JVM堆大小的限制，进一步扩展会导致垃圾回收问题。此外，存储2亿个文件需要占用JVM堆的大量内存（约200GB）。

为了在2.0中扩展Alluxio命名空间，我们新增了对在磁盘上的RocksDB中存储部分命名空间的支持。最近访问的数据存储在内存中，而较旧的数据最终存储在磁盘上。这降低了为Alluxio命名空间提供服务的内存需求，通过减少需要处理的对象的数量，减轻了Java垃圾回收器的压力。

## 设置堆外元数据

堆外元数据是Alluxio 2.0中的默认值，因此无需任何操作即可开始使用它。如果要将所有元数据存储在内存中，设置

**alluxio-site.properties**

```
alluxio.master.metastore=HEAP # default is ROCKS for off-heap
```

堆外元数据的默认位置是`${alluxio.work.dir}/metastore`，通常位于Alluxio主目录下。你可以配置该位置，设置

**alluxio-site.properties**

```
alluxio.master.metastore.dir=/path/to/metastore
```



## 内存中的元数据存储

虽然Alluxio能够将所有元数据存储在磁盘上，但由于磁盘相对于内存较慢，这会导致性能降低。为了缓解这种情况，Alluxio在内存中保留了大量文件，以便能够快速访问。当文件数量增长到接近最大缓存大小时，Alluxio会将最近使用较少的文件驱逐到磁盘上。用户可以自行权衡内存使用量和速度，设置

**alluxio-site.properties**

```
alluxio.master.metastore.inode.cache.max.size=10000000
```

缓存大小为1000万，需要约10GB的堆。

## 大小

### 内存

为获得最佳性能，master的内存应该能够容纳工作集的元数据。估计工作集中的文件数，并将每个文件乘以1KB。例如，对于500万个文件，估计5GB的master内存。要配置master内存，请在`conf/alluxio-env.sh`中添加jvm选项

**alluxio-env.sh**

```
ALLUXIO_MASTER_JAVA_OPTS+=" -Xmx5G"
```

如果你有足够多可用内存，我们建议在生产部署中为master提供31GB的堆。

**alluxio-env.sh**

```
ALLUXIO_MASTER_JAVA_OPTS+=" -Xmx31G"
```

更新缓存大小以匹配分配的内存量：

**alluxio-site.properties**

```
# 31 million

alluxio.master.metastore.inode.cache.max.size=31000000
```

这适用于大多数工作负载，并且避免了切换到64位指针（一旦堆达到32GB，就需要切换到64位指针）。

### 磁盘

Alluxio在磁盘上存储inode比在内存中存储更紧凑。每个文件不再需要1KB，而是需要约500字节。要支持在磁盘上存储10亿个文件，请提供具有500GB空间的HDD或SSD，并确保rocksdb metastore正在使用该磁盘。

**alluxio-site.properties**

```
alluxio.master.metastore.dir=/path/to/metastore
```



## 结论

Alluxio 2.0通过利用磁盘资源实现冷元数据，显著提高了元数据的可扩展性。这使得Alluxio能够处理更大的命名空间（随着人们将更多存储挂载到Alluxio）并更好地利用对象存储。请继续关注我们的后续博客，后续将会介绍利用RocksDB存储元数据的技术细节。