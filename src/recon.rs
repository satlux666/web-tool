use colored::*;
use dns_lookup::lookup_host;
use rayon::prelude::*;
use std::sync::Mutex;
use indicatif::{ProgressBar, ProgressStyle};
use crate::utils::{domain_only, fix_url};
use crate::wordlists::{common_subs, common_paths};
use reqwest::blocking::Client;

pub fn subdomains(target: &str, limit: usize) -> Vec<String> {
    let base = domain_only(target);
    println!("{} subdomain enum", "[*]".cyan());
    let list = common_subs();
    let take = limit.min(list.len());
    let found = Mutex::new(Vec::new());
    let pb = ProgressBar::new(take as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{bar:35.cyan/blue}] {pos}/{len}").unwrap());

    list.into_iter().take(take).collect::<Vec<_>>().par_iter().for_each(|w| {
        let sub = format!("{}.{}", w, base);
        if lookup_host(&sub).is_ok() {
            found.lock().unwrap().push(sub);
        }
        pb.inc(1);
    });
    pb.finish_and_clear();

    let mut out = found.into_inner().unwrap();
    out.sort();
    out.dedup();
    println!("{} {} subs found", "[+]".green(), out.len());
    for s in &out {
        println!("    {}", s);
    }
    out
}

pub fn interesting_paths(target: &str, client: &Client) -> Vec<String> {
    let base = fix_url(target).trim_end_matches('/').to_string();
    println!("{} checking common paths", "[*]".cyan());
    let found = Mutex::new(Vec::new());
    let paths = common_paths();

    paths.par_iter().for_each(|p| {
        let full = format!("{}{}", base, p);
        if let Ok(r) = client.get(&full).send() {
            let code = r.status().as_u16();
            if code == 200 || code == 301 || code == 302 || code == 403 || code == 401 {
                found.lock().unwrap().push(format!("{} ({})", p, code));
            }
        }
    });

    let res = found.into_inner().unwrap();
    for x in &res {
        println!("    {}", x);
    }
    res
}

pub fn tech_detect(target: &str, client: &Client) -> Vec<String> {
    let url = fix_url(target);
    let mut techs = Vec::new();
    if let Ok(resp) = client.get(&url).send() {
        if let Some(s) = resp.headers().get("server") {
            techs.push(format!("Server: {}", s.to_str().unwrap_or("?")));
        }
        if let Some(p) = resp.headers().get("x-powered-by") {
            techs.push(format!("X-Powered-By: {}", p.to_str().unwrap_or("?")));
        }
        if let Ok(body) = resp.text() {
            let b = body.to_lowercase();
            if b.contains("wp-content") || b.contains("wordpress") { techs.push("WordPress".into()); }
            if b.contains("laravel") { techs.push("Laravel".into()); }
            if b.contains("react") { techs.push("React".into()); }
            if b.contains("vue.") || b.contains("vuejs") { techs.push("Vue".into()); }
            if b.contains("jquery") { techs.push("jQuery".into()); }
            if b.contains("bootstrap") { techs.push("Bootstrap".into()); }
            if b.contains("cloudflare") { techs.push("Cloudflare".into()); }
            if b.contains("nginx") { techs.push("Nginx".into()); }
            if b.contains("apache") { techs.push("Apache".into()); }
        }
    }
    println!("{} technologies:", "[+]".green());
    for t in &techs {
        println!("    {}", t);
    }
    techs
}

pub fn basic_osint(target: &str, client: &Client) {
    let d = domain_only(target);
    let url = fix_url(target);
    println!("{} basic info", "[*]".cyan());
    if let Ok(r) = client.get(&url).send() {
        println!("    status: {}", r.status());
        if let Some(s) = r.headers().get("server") {
            println!("    server: {}", s.to_str().unwrap_or("-"));
        }
    }
    if let Ok(ips) = lookup_host(&d) {
        println!("    dns:");
        for ip in ips {
            println!("      {}", ip);
        }
    }
}
