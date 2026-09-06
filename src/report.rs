use serde_json::json;
use crate::utils::{save_file, now, domain_only};

pub fn write_json(target: &str, ports: &[u16], subs: &[String], emails: &[String], phones: &[String], paths: &[String], techs: &[String], notes: &[String]) {
    let data = json!({
        "tool": "luxweb",
        "target": target,
        "time": now(),
        "open_ports": ports,
        "subdomains": subs,
        "emails": emails,
        "phones": phones,
        "interesting_paths": paths,
        "technologies": techs,
        "notes": notes
    });
    let name = format!("{}_{}.json", domain_only(target), now());
    save_file(&name, &serde_json::to_string_pretty(&data).unwrap());
}

pub fn write_txt(target: &str, ports: &[u16], subs: &[String], emails: &[String], phones: &[String], paths: &[String], techs: &[String], notes: &[String]) {
    let mut out = String::new();
    out.push_str(&format!("luxweb report - {}\n", target));
    out.push_str(&format!("time: {}\n\n", now()));
    out.push_str("OPEN PORTS\n");
    for p in ports { out.push_str(&format!("  {}\n", p)); }
    out.push_str("\nSUBDOMAINS\n");
    for s in subs { out.push_str(&format!("  {}\n", s)); }
    out.push_str("\nEMAILS\n");
    for e in emails { out.push_str(&format!("  {}\n", e)); }
    out.push_str("\nPHONES\n");
    for p in phones { out.push_str(&format!("  {}\n", p)); }
    out.push_str("\nPATHS\n");
    for p in paths { out.push_str(&format!("  {}\n", p)); }
    out.push_str("\nTECH\n");
    for t in techs { out.push_str(&format!("  {}\n", t)); }
    out.push_str("\nNOTES\n");
    for n in notes { out.push_str(&format!("  {}\n", n)); }
    let name = format!("{}_{}.txt", domain_only(target), now());
    save_file(&name, &out);
}
