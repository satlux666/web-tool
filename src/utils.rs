use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;
use url::Url;
use std::fs::{self, File};
use std::io::Write;
use chrono::Local;
use colored::*;

pub fn make_client(proxy: Option<String>) -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36"));
    let mut b = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .danger_accept_invalid_certs(true)
        .redirect(reqwest::redirect::Policy::limited(5));
    if let Some(px) = proxy {
        if let Ok(p) = reqwest::Proxy::all(&px) {
            b = b.proxy(p);
        }
    }
    b.build().expect("client failed")
}

pub fn fix_url(t: &str) -> String {
    if t.starts_with("http://") || t.starts_with("https://") {
        t.to_string()
    } else {
        format!("https://{}", t)
    }
}

pub fn domain_only(t: &str) -> String {
    match Url::parse(&fix_url(t)) {
        Ok(u) => u.host_str().unwrap_or(t).to_string(),
        Err(_) => t.to_string(),
    }
}

pub fn save_file(name: &str, content: &str) {
    let _ = fs::create_dir_all("results");
    let path = format!("results/{}", name);
    if let Ok(mut f) = File::create(&path) {
        let _ = f.write_all(content.as_bytes());
        println!("{} saved {}", "[+]".green(), path);
    }
}

pub fn now() -> String {
    Local::now().format("%Y%m%d-%H%M%S").to_string()
}
