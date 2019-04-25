---
title: 翻译：Getting Started with Alluxio and Spark in 5 Minutes
date: 2019-04-25 13:20:48
categories: 翻译
tags: Alluxio
---

# 5分钟快速上手Alluxio和Spark

Alluxio技术博客翻译系列——Getting Started with Alluxio and Spark in 5 Minutes

[原文地址](http://www.alluxio.com/blog/getting-started-with-alluxio-and-spark-in-5-minutes)

本文也会发布在微信公众号“**Alluxio**”上，欢迎关注该微信公众号。

---

## 简介

Apache Spark给大数据计算带来了重大革新，将其与Alluxio配合使用时，其效果还将更加出色。[Alluxio](https://www.alluxio.org/)为Spark提供了可靠的数据共享层，通过Alluxio处理存储， Spark在执行应用程序逻辑时更加得心应手。Bazaarvoice使用Spark和Alluxio构建了实时大数据平台，该平台不仅能够在黑色星期五等高峰事件中处理15亿次页面浏览量，还能对这些数据进行实时分析（[更多内容](https://www.alluxio.com/blog/accelerate-spark-and-hive-jobs-on-aws-s3-by-10x-with-alluxio-tiered-storage)）。在这种规模下，速度的提升能够给新工作负载赋能。我们已经构建了一个简洁的方法来集成Alluxio和Spark。

本博客面向那些对如何利用Alluxio和Spark感兴趣的新人，所有示例都可以通过简单步骤在本地计算机上复现。

## 准备

要开始使用Alluxio和Spark，首先需要下载两个系统的发行版，安装Java 8并下载示例数据以完成练习。

- [Alluxio 1.8.1](https://www.alluxio.org/download)
- [Spark 2.4.0](https://spark.apache.org/downloads.html)
- [安装Java - JDK 8](https://www.oracle.com/technetwork/java/javase/downloads/jdk8-downloads-2133151.html)
- [示例数据文件](https://alluxio-public.s3.amazonaws.com/sample-2g.tar.gz)

将Alluxio和Spark下载到工作目录。样本数据文件也可以下载和解压缩到此处，或者`/tmp`目录。Spark必须引用样本文件的完整路径。

## 配置Alluxio

- 打开远程登录服务，以便能够成功`ssh localhost`。

![远程登录](setting-up-alluxio.png)

- 为避免重复输入密码，可以将主机的公共SSH密钥添加到`~/.ssh/authorized_keys`。有关详细信息，请参阅[本教程](http://www.linuxproblem.org/art_9.html)。

从[Alluxio网站](http://alluxio.org/download)下载、解压并启动Alluxio的预编译版本

```bash
$ tar -zxf alluxio-1.8.1-bin.tar.gz

$ cd alluxio-1.8.1

$ bin/alluxio bootstrap-conf localhost

$ bin/alluxio-start.sh local -f
```

访问[localhost:19999/home](http://localhost:19999/home)的Web UI，验证Alluxio系统是否正在运行。

## 配置Spark

解压缩Spark的预编译版本

```bash
$ tar -zxf spark-2.4.0-bin-hadoop2.7.tgz

$ cd spark-2.4.0-bin-hadoop2.7
```

通过在Spark项目目录下运行以下命令来启动`spark-shell`程序。在交互式shell中，你可以处理来自各种源的数据，在当前情况下，数据源是本地文件系统。

## 集成Alluxio和Spark

Spark需要使用Alluxio客户端jar包才能让Spark程序与Alluxio交互；Alluxio客户端jar包是两个系统的集成点。使用`--driver-class-path`参数后跟客户端jar包的路径指定它，该客户端jar包位于Alluxio包的client目录中。

```bash
$ cd spark-2.4.0-bin-hadoop2.7

$ bin/spark-shell --driver-class-path <PATH>/alluxio-1.8.1/client/alluxio-1.8.1-client.jar
```



## 运行一个简单的示例

作为第一个示例，我们将使用Alluxio和Spark从本地存储读取数据，借此来熟悉两个系统的集成。之前下载的示例数据文件包含了大小为2gb的文件，该文件的内容为从英语词典中随机生成的单词。

```bash
$ cd /tmp

$ tar -zxf sample-2g.tar.gz
```

在Spark中处理2gb样本文件并计算其中的行数。确保指定了正确的文件路径。

```bash
scala> val file = sc.textFile("/tmp/sample-2g")

scala> file.count()
```

Alluxio还可以用作数据的源或接收点。你可以将文件保存到Alluxio，并且类似地对Alluxio的数据执行相同的操作，就像对本地文件系统一样。

```bash
scala> file.saveAsTextFile("alluxio://localhost:19998/sample-2g")

scala> val alluxioFile = sc.textFile("alluxio://localhost:19998/sample-2g")

scala> alluxioFile.count()
```



## 结论

本文是使用Alluxio和Spark的简单介绍。我们的后续的博客将详细介绍利用Alluxio和Spark的优势：

- 多个作业之间的数据共享：只有一个作业需要从冷数据中慢速读取
- 作业失败易恢复：Alluxio在作业失败或重新启动时保留数据
- 托管存储：Alluxio优化了跨应用程序分配的存储的利用率