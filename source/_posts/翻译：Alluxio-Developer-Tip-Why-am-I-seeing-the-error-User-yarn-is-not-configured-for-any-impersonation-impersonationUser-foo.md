---
title: >-
  翻译：Alluxio Developer Tip: Why am I seeing the error 'User yarn is not
  configured for any impersonation. impersonationUser foo'
date: 2019-02-25 15:29:21
categories: 翻译
tags: Alluxio
---

# Alluxio使用贴士：impersonationUser相关配置问题分析与解决

Alluxio技术博客翻译系列——Alluxio Developer Tip: Why am I seeing the error “User yarn is not configured for any impersonation. impersonationUser: foo”? 

[原文地址](https://www.alluxio.com/blog/alluxio-developer-tip-why-am-i-seeing-the-error-user-yarn-is-not-configured-for-any-impersonation-impersonationuser-foo?tdsourcetag=s_pctim_aiomsg)

本文也会发布在微信公众号“**Alluxio**”上，欢迎关注该微信公众号。

## 什么是用户模拟（User Impersonation）

模拟（Impersonation）就是一个用户代表另一个用户行事的能力。例如，假设用户“yarn”具有连接到服务的证书，但用户“foo”没有。因此，用户“foo”再一般情况下将永远无法访问该服务。但是，用户“yarn”可以访问该服务并模拟用户“foo”（代表用户“foo”行事），使得用户“foo”获得访问该服务的能力 。因此，模拟能够使一个用户能够代表另一个用户访问服务。

 

## 谁是用户？

模拟功能定义了用户如何代表其他用户行事。因此，了解谁是用户是很重要的。Hadoop应用程序（例如Spark、Hive、Presto等）使用Hadoop客户端与HDFS（或其他HDFS兼容的文件系统，例如Alluxio）进行交互。将Alluxio与Hadoop应用程序一起使用时，将有**两个**客户端作为应用程序的一部分——Hadoop客户端和[Alluxio客户端](https://www.alluxio.com/blog/developer-tip-why-did-my-job-fail-with-error-message-class-alluxiohadoopfilesystem-not-found)。当应用程序使用Alluxio URI（例如alluxio://host:port/my/path/）时，它将首先直接与Hadoop客户端交互，而在内部，Hadoop客户端将与Alluxio客户端进行交互。最后，Alluxio客户端直接与Alluxio master和Alluxio worker通信。

可以为Hadoop客户端和Alluxio客户端指定用户或身份（identity）。由于Hadoop客户端和Alluxio客户端的用户可以独立指定，这意味着Hadoop客户端用户与Alluxio客户端用户可能不同。Hadoop客户端用户甚至可以与Alluxio客户端用户位于不同的命名空间中。例如，可能在单个应用程序中，Hadoop客户端被指定为用户“foo”，但Alluxio客户端用户被指定为“yarn”。

 

## 什么是Alluxio客户端侧Hadoop模拟（Alluxio Client-side Hadoop Impersonation）？

Alluxio客户端侧Hadoop模拟旨在解决当Hadoop客户端用户与Alluxio客户端用户不同时所出现的混淆问题。由于Hadoop客户端用户和Alluxio客户端用户在同一应用程序中可能不同，因此Alluxio客户端侧Hadoop模拟会检查Hadoop客户端用户，然后尝试模拟该Hadoop客户端用户。

例如，假设Hadoop应用程序正在运行，将Hadoop客户端用户指定为“foo”，将Alluxio客户端用户指定为“yarn”。如果没有客户端侧Hadoop模拟，Alluxio客户端将以用户“yarn”而不是“foo”连接到Alluxio服务器（master和worker）。这意味着任何数据交互行为都将归属于用户“yarn”。

但是，通过客户端侧模拟，Alluxio客户端将确定Hadoop客户端用户是“foo”，然后用户“yarn”模拟用户“foo”连接到Alluxio服务器。现在，所有数据交互都将归属于用户“foo”。启用此模拟后，Alluxio客户端可以代表相同的Hadoop客户端用户运行，从而实现与Alluxio的无缝透明交互。

 

## 为什么我会遇到这些错误？

使用Alluxio时，您可能会遇到类似“User yarn is not configured for any impersonation. impersonationUser: foo”的错误。这些是Alluxio服务器拒绝访问的错误信息，您可以在logs/master.log中或logs/worker.log中分别找到master或worker的报错信息，但这些错误有时也会传递给应用程序。

如果您看到此类错误，则表示未正确配置Alluxio服务器来启用客户端侧Hadoop模拟。此错误“User yarn is not configured for any impersonation. impersonationUser: foo”表示“在用户‘yarn’下运行的应用程序正在连接到Alluxio服务，并且正在尝试模拟用户‘foo’，但是Alluxio服务器未配置为允许用户‘yarn’这样做。”

 

## 为什么要使用模拟？

一个自然而然的问题是，究竟为什么在这种情况下要利用模拟？Alluxio客户端尝试模拟的原因是，当Alluxio客户端检测到Hadoop客户端用户与Alluxio客户端用户不同时，Alluxio客户端将尝试模拟Hadoop客户端用户。在运行示例中，Hadoop客户端用户是“foo”，因此如果应用程序直接与HDFS交互，它将以用户“foo”读写文件。然而，在该场景中使用Alluxio时，Alluxio客户端用户是“yarn”（与用户“foo”不同）。因此，如果没有使用模拟，之前以用户“foo”与HDFS交互的相同应用程序将以用户“yarn”与Alluxio进行交互。由于身份不同，Alluxio客户端将尝试模仿Hadoop客户端用户。在此示例中，Alluxio用户“yarn”将模拟用户“foo”。

 

## 如何解决这个错误？

解决此错误的最简单方法是[在Alluxio服务器上正确配置模拟](http://www.alluxio.org/docs/1.8/en/advanced/Security.html#impersonation)。这涉及将以下参数添加到Alluxio服务器配置文件并重新启动Alluxio服务：

`alluxio.master.security.impersonation.<USER>.users=*`

<USER\>是一个占位符，必须由需要模拟能力的实际用户名替换。对于本运行示例，它将是：

`alluxio.master.security.impersonation.yarn.users=*`

此配置意味着用户“yarn”能够模拟任何其他用户。因此，下次用户“yarn”想要模仿用户“foo”时，Alluxio服务器将会允许，应用程序可以无缝地继续。

避免错误的另一种方法是完全禁用客户端模拟。这需要客户端配置参数（不在服务器上），通过设置：

`alluxio.security.login.impersonation.username=_NONE_`

这会禁用客户端模拟功能，因此Alluxio客户端不会尝试模拟Hadoop客户端用户。在本运行示例中，这意味着当应用程序与Alluxio交互时，所有读取和写入将以用户“yarn”进行，而不是用户“foo”。

有关配置的更多详细信息和选项，请参阅[模拟文档](http://www.alluxio.org/docs/1.8/en/advanced/Security.html#impersonation)。