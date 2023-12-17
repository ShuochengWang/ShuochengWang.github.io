---
layout: post
title: 'notify-me: A Rust library for sending notifications'
tags:
    - Rust
    - notify
description: "notify-me: A Rust library for sending notifications. Send notifications to email or communication software, such as WeChat. It is very suitable for developers to receive notifications of their software on mobile phones."
---

[notify-me](https://github.com/ShuochengWang/notify-me): Send notifications to email or communication software, such as WeChat. It is very suitable for developers to receive notifications of their software on mobile phones.

<!-- more -->

I developed [notify-me](https://github.com/ShuochengWang/notify-me) crate to send notifications to myself when my order grabbing program runs.I designed an order grabbing program.And when I grab an order, I hope I can receive the notification immediately, instead of watching my program on the computer all the time.With this crate, when I grab the order successfully, I will use it to send a notification to my WeChat or email, so that I can immediately receive the notification on my mobile phone for processing.

## Features

- Send notifications to your email
- Send notifications to your WeChat

## Examples

To use this library, add the following to your `Cargo.toml`:

```rust
[dependencies]
notify-me = "0.2"
```

### Send notifications to WeChat

Note that, this crate use [xtuis](https://xtuis.cn/) to implement WeChat notifications.
Hence you have to first follow the WeChat official account of [xtuis](https://xtuis.cn/) and get the `token`.

```rust
use notify_me::{Notify, WechatNotifier};

let notifier = WechatNotifier::new("your xtuis token").unwrap();
notifier.notify("notification title", "notification content").unwrap();
```

### Send notifications to email

```rust
use notify_me::{Notify, EmailNotifier};

let notifier = EmailNotifier::new("smtp_host", "smtp_username", "smtp_password", "recipient").unwrap();
notifier.notify("notification title", "notification content").unwrap();
```
