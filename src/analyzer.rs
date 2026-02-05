use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use crate::meta_lang::parse_entities;

pub fn analyze_corpus(path: &str) {
    let mut comp_count: HashMap<String, usize> = HashMap::new();
    let mut event_count: HashMap<String, usize> = HashMap::new();
    let mut func_count: HashMap<String, usize> = HashMap::new();
    let mut total_files = 0usize;

    if let Ok(entries) = read_dir(path) {
        for ent in entries.flatten() {
            let p = ent.path();
            if p.is_file() {
                if let Some(ext) = p.extension() {
                    if ext == "meta" {
                        total_files += 1;
                        if let Ok(s) = read_to_string(&p) {
                            let entities = parse_entities(&s);
                            for e in entities {
                                for c in e.components {
                                    *comp_count.entry(c).or_default() += 1;
                                }
                                for ev in e.events {
                                    *event_count.entry(ev.name.clone()).or_default() += 1;
                                    // scan body for function-like tokens (foo(...))
                                    let mut i = 0usize;
                                    let b = ev.body.as_bytes();
                                    while i + 1 < b.len() {
                                        // find identifier followed by '('
                                        if is_ident_start(b[i]) {
                                            let start = i;
                                            i += 1;
                                            while i < b.len() && is_ident_continue(b[i]) { i += 1; }
                                            // skip spaces
                                            while i < b.len() && b[i].is_ascii_whitespace() { i += 1; }
                                            if i < b.len() && b[i] == b'(' {
                                                let name = String::from_utf8_lossy(&b[start..i]).to_string();
                                                *func_count.entry(name).or_default() += 1;
                                            }
                                        } else {
                                            i += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Analyzed {} files", total_files);
    println!("Top components:");
    for (k,v) in top_n(&comp_count, 10) { println!("  {} => {}", k, v); }
    println!("Top events:");
    for (k,v) in top_n(&event_count, 10) { println!("  {} => {}", k, v); }
    println!("Top function calls in bodies:");
    for (k,v) in top_n(&func_count, 20) { println!("  {} => {}", k, v); }
}

fn is_ident_start(b: u8) -> bool { (b'A' <= b && b <= b'Z') || (b'a' <= b && b <= b'z') || b == b'_' }
fn is_ident_continue(b: u8) -> bool { is_ident_start(b) || (b'0' <= b && b <= b'9') }

fn top_n(map: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut v: Vec<_> = map.iter().map(|(k,&c)| (k.clone(), c)).collect();
    v.sort_by(|a,b| b.1.cmp(&a.1));
    v.into_iter().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        analyze_corpus("corpus");
    }
}
