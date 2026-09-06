use colored::*;
use regex::Regex;
use std::collections::HashSet;
use reqwest::blocking::Client;
use crate::utils::fix_url;

pub fn emails_and_phones(target: &str, client: &Client) -> (Vec<String>, Vec<String>) {
    let url = fix_url(target);
    println!("{} extracting contacts", "[*]".cyan());
    let mut emails = HashSet::new();
    let mut phones = HashSet::new();

    if let Ok(resp) = client.get(&url).send() {
        if let Ok(text) = resp.text() {
            let re_mail = Regex::new(r"[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,6}").unwrap();
            let re_phone = Regex::new(r"(?:\+?\d{1,3}[\s\-.]?)?\(?\d{2,4}\)?[\s\-.]?\d{3,4}[\s\-.]?\d{3,5}").unwrap();

            for c in re_mail.captures_iter(&text) {
                emails.insert(c[0].to_lowercase());
            }
            for c in re_phone.captures_iter(&text) {
                let p = c[0].to_string();
                let digits: String = p.chars().filter(|x| x.is_ascii_digit()).collect();
                if digits.len() >= 8 && digits.len() <= 15 {
                    phones.insert(p);
                }
            }
        }
    }

    let e: Vec<_> = emails.into_iter().collect();
    let p: Vec<_> = phones.into_iter().collect();
    println!("{} emails: {} | phones: {}", "[+]".green(), e.len(), p.len());
    for x in &e { println!("    {}", x); }
    for x in &p { println!("    {}", x); }
    (e, p)
}
