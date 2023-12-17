---
layout: post
title: Auto log in websites by Rust reqwest
date: 2023-02-15 18:41:28
tags: 
- Rust
- Web programming
description: "Auto log in websites by Rust reqwest"
---

`reqwest` is a Rust crate for web programming. We can use it to auto log in some websites. Here I will give a simple example. If you want to automatically log in a website with strict man-machine verification, you may need to perfect more details.

<!-- more -->

#### Create a client with cookie store

First, we need to create a `reqwest` client. Since we want to log in websites and do something in log-in status, we need to use cookies to show the website that we are logged in.

`reqwest` support `"cookies"` feature, cookies received in responses will be preserved and included in additional requests. This is to say, the client will store and use cookies automatically when you **enable** this feature.

Now we can create a client with cookie store. Remember you should enable `"cookies"` feature. However, cookies can only be used in the current login process, and cannot be persisted locally for the next time log-in.

``` Rust
use reqwest::Client;

let client = Client::builder()
    .cookie_store(true)
    .build()?;
```

If we only want to log in once with the account and directly use the existing cookie instead of the account the next time we log in, we can use persistent cookie store (Crate `reqwest_cookie_store`) and save the cookie in a local file. Each time, we read the local file to get the cookie and initial cookie store before logging in. If the cookie is valid, we do not need to log in with that account. We just do something on the site directly. If the cookie is invalid or has expired, we will log in with that account and save the new cookie in a local file for next use.

``` Rust
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};

let cookie_store_path = String::from("cookies.json");
let cookie_store = {
    // read the local file if exists, or create a new file otherwise.
    let file = File::open(&cookie_store_path).unwrap_or_else(|_| {
        File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&cookie_store_path)
            .unwrap()
    });
    CookieStore::load_json(BufReader::new(file)).unwrap()
};
let cookie_store = Arc::new(CookieStoreMutex::new(cookie_store));
let client = Client::builder()
    .cookie_provider(cookie_store.clone())
    .build()?;
```

#### Log in with the account

If we use an existing cookie, we first have to check whether we are logged in. We can request a web page to check the response and determine if we are logged in.

``` rust
pub async fn is_login(&self) -> Result<bool> {
    let res = self.client.get("https://example.com/dashboard").send().await?;
    // check the response
    // ...
}
```

To log in, we first have to create a form using the corresbonding account (e.g., username and password). Then we create a post request with headers and the form. The details of the request is according to the websites (e.g., how to create the form, what headers are needed). We can learn about it by press `F12` in browser, and check the network monitor in browser dev-tools. Then we send the post request and check whether the login is successful.

``` Rust
pub async fn login(&self) -> Result<()> {
    // check if we need to login
    if self.is_login().await? {
        info!("already login");
        return Ok(());
    }

    // create a form using your account
    let form = ...;
    // send a post request to log in
    let res = self
        .client
        .post(LOGIN_POST_URL)
        .header("origin", "https://example.com")
        .header("referer", "https://example.com/auth/form/login")
        .form(&form)
        .send()
        .await?;

    // check whether the login is successful.
    if self.is_login().await? {
        info!("login success");
        Ok(())
    } else {
        info!("login failed. {:#?}", res);
        Err(anyhow!("login failed"))
    }
}
```

#### Do something in login status

After log in, you can do something you wanted, like, crawl information, sec-kill, etc. We can use Crate `scraper` to deal with the html document, like, parse it and apply some CSS selector.

``` rust
use scraper::{Html, Selector};

let res = self.client.get(URL).send().await?;
let text = res.text().await?;

// deal with the html
let document = Html::parse_document(&text);
let selector = Selector::parse(r#"tr[class~="myclass"]"#).unwrap();
let item = document.select(&selector).next().unwrap();
```

#### Save the cookie when exit

If we use cookie store in the local file, we should save the latest cookie in the local file when the program exits.

Luckily, the `Drop` can help us do it automatically in Rust. We can impl the `Drop` trait to save the cookie.

``` rust
impl Drop for MyClient {
    fn drop(&mut self) {
        let mut writer = File::create(self.cookie_store_path).map(BufWriter::new).unwrap();
        let store = self.cookie_store.lock().unwrap();
        store.save_json(&mut writer).unwrap();
    }
}
```
