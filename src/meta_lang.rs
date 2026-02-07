#[derive(Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub components: Vec<String>,
    pub events: Vec<Event>,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub params: Option<String>,
    pub body: String,
}

fn trim_semicolon(s: &str) -> &str {
    let s = s.trim();
    if s.ends_with(';') {
        &s[..s.len() - 1]
    } else {
        s
    }
}

pub fn parse_entities(input: &str) -> Vec<Entity> {
    let mut res = Vec::new();
    let mut i = 0usize;
    let bytes = input.as_bytes();
    let n = bytes.len();

    while i < n {
        // find "entity"
        if input[i..].starts_with("entity") {
            i += "entity".len();
            // skip whitespace
            while i < n && input.as_bytes()[i].is_ascii_whitespace() {
                i += 1;
            }
            // read name
            let start = i;
            while i < n {
                let c = input.as_bytes()[i] as char;
                if c.is_whitespace() || c == '{' {
                    break;
                }
                i += 1;
            }
            let name = input[start..i].trim().to_string();

            // find opening brace
            while i < n && input.as_bytes()[i] != b'{' {
                i += 1;
            }
            if i >= n { break; }
            i += 1; // skip '{'
            let mut brace_level = 1;
            let block_start = i;
            while i < n && brace_level > 0 {
                match input.as_bytes()[i] {
                    b'{' => brace_level += 1,
                    b'}' => brace_level -= 1,
                    _ => {}
                }
                i += 1;
            }
            let block_end = i - 1; // position of matching '}'
            let block = &input[block_start..block_end];

            // parse components
            let mut components = Vec::new();
            if let Some(idx) = block.find("components:") {
                if let Some(open) = block[idx..].find('[') {
                    let off = idx + open + 1;
                    if let Some(close) = block[off..].find(']') {
                        let list = &block[off..off + close];
                        components = list
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                }
            }

            // parse events: naive scan for "on <Name>(...){...}"
            let mut events = Vec::new();
            let mut j = 0usize;
            while j < block.len() {
                if block[j..].starts_with("on ") {
                    j += 3;
                    while j < block.len() && block.as_bytes()[j].is_ascii_whitespace() { j += 1; }
                    let en_start = j;
                    while j < block.len() {
                        let c = block.as_bytes()[j] as char;
                        if c.is_whitespace() || c == '(' { break; }
                        j += 1;
                    }
                    let en_name = block[en_start..j].trim().to_string();
                    // params
                    let mut params = None;
                    if j < block.len() && block.as_bytes()[j] == b'(' {
                        j += 1;
                        let pstart = j;
                        while j < block.len() && block.as_bytes()[j] != b')' { j += 1; }
                        params = Some(block[pstart..j].trim().to_string());
                        if j < block.len() { j += 1; }
                    }
                    // find body
                    while j < block.len() && block.as_bytes()[j] != b'{' { j += 1; }
                    if j >= block.len() { break; }
                    j += 1; // skip '{'
                    let mut lev = 1usize;
                    let bstart = j;
                    while j < block.len() && lev > 0 {
                        match block.as_bytes()[j] {
                            b'{' => lev += 1,
                            b'}' => lev -= 1,
                            _ => {}
                        }
                        j += 1;
                    }
                    let bend = j - 1;
                    let body = block[bstart..bend].trim().to_string();
                    events.push(Event { name: en_name, params, body });
                } else {
                    j += 1;
                }
            }

            res.push(Entity { name, components, events });
        } else {
            i += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sample_entity() {
        let s = r#"entity A { components: [X, Y]; on Tick() { do(); } }"#;
        let es = parse_entities(s);
        assert_eq!(es.len(), 1);
        let e = &es[0];
        assert_eq!(e.name, "A");
        assert_eq!(e.components, vec!["X", "Y"]);
        assert_eq!(e.events.len(), 1);
        assert_eq!(e.events[0].name, "Tick");
    }
}
