---
title: 基于Hexo搭建个人博客
date: 2019-01-17 21:01:28
categories: 博客
tags: hexo
---
# 基于Hexo搭建个人博客

##### 安装

安装 Node.js,  Git

安装 Hexo：

```bash
npm install -g hexo-cli
```



##### 初始化

安装 Hexo 完成后，执行下列命令

```bash
mkdir blog
hexo init blog
cd blog
npm install
```



##### 配置参数

设置 _config.yml 文件，具体配置相见 https://hexo.io/zh-cn/docs/configuration



##### 配置主题

配置NexT主题 http://theme-next.iissnan.com/getting-started.html

安装该主题网页进行相关设置即可，对分类、标签的相关设置也在其中。



##### 发布文章

```
hexo new "My New Post"
```

然后在 source/_posts 文件夹中找到新创建的md文件，写下博客内容即可。



##### 部署到Github

安装 [hexo-deployer-git](https://github.com/hexojs/hexo-deployer-git)。

```bash
npm install hexo-deployer-git --save
```

修改 _config.yml 文件

```
deploy:
  	type: git  
	repo: git@github.com:ShuochengWang/ShuochengWang.github.io.git
	branch:  master        #[branch]
	message:               #[message]
```

部署命令：

```bash
hexo g -d
# hexo d -g
```

Github上需要设置如下：

1. 新建一个Repository，名为：yourname.github.io
2. 在该Repository的设置：Settings -> Options -> Github Pages -> Source，选择master branch，并保存



##### 使用自定义域名

首先要购买域名，然后设置其域名解析：

```
主机记录     记录类型           记录值
 @              A             192.30.252.153
 @              A             192.30.252.154
 www          CNAME           username.github.io.
```

然后在 source 文件夹中新建一个 CNAME 文件（无后缀），CNAME 中写入你的域名，例如：

```
example.com
```



##### 保存博客

部署到 github Repository的文件和本地文件不同，仅仅包括最后生成的网页代码。我们需要保存本地的Blog文件，以防文件丢失，那样就不能继续发布新博客了。为了方便，把我们的博客源文件保存到另一个分支blog中。

```
git init
git checkout -b blog
git remote add origin git@github.com:yourname/yourname.github.io.git
git add .
git commit -m "Update blog"
git push origin blog
```

为了避免误操作，将blog设置为默认分支，在Repository的 Settings -> Branchs -> Default branch，选择 blog 分支，然后 Update。



##### 常用命令：

```bash
hexo clean
hexo generate # hexo g
hexo server
hexo deploy # hexo d
```