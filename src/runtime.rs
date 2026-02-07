use std::collections::HashMap;
use crate::meta_lang::Event;

#[derive(Debug, Clone)]
pub struct EntityInstance {
    pub name: String,
    pub tag: String,
    pub health: i32,
    pub velocity: f64,
    pub position: f64,
}

#[derive(Debug, Clone)]
pub enum Value {
    Float(f64),
    Int(i64),
    Str(String),
    EntitySnapshot(SimpleEntity),
}

#[derive(Debug, Clone)]
pub struct SimpleEntity {
    pub name: String,
    pub tag: String,
    pub health: i32,
}

impl EntityInstance {
    pub fn new(name: &str, tag: &str) -> Self {
        Self { name: name.to_string(), tag: tag.to_string(), health: 100, velocity: 0.0, position: 0.0 }
    }

    pub fn snapshot(&self) -> SimpleEntity {
        SimpleEntity { name: self.name.clone(), tag: self.tag.clone(), health: self.health }
    }
}

pub fn execute_event(entity: &mut EntityInstance, event: &Event, params: &HashMap<String, Value>) {
    // naive executor: handle simple statements and one-level if conditions
    let body = event.body.trim();
    if body.starts_with("if ") {
        // parse condition and inner body
        if let Some(cond_start) = body.find('(') {
            if let Some(cond_end) = body.find(')') {
                let cond = body[cond_start+1..cond_end].trim();
                let inner_start = body.find('{').unwrap_or(body.len());
                let inner_end = body.rfind('}').unwrap_or(body.len());
                let inner = &body[inner_start+1..inner_end];
                if eval_condition(cond, params) {
                    exec_statements(entity, inner, params);
                }
                return;
            }
        }
    }

    exec_statements(entity, body, params);
}

fn eval_condition(cond: &str, params: &HashMap<String, Value>) -> bool {
    // only supports patterns like: other.tag == "Enemy"
    if cond.contains("==") {
        let parts: Vec<&str> = cond.split("==").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let left = parts[0];
            let right = parts[1].trim_matches('"');
            if left.ends_with(".tag") {
                let var = left.trim_end_matches(".tag");
                if let Some(v) = params.get(var) {
                    if let Value::EntitySnapshot(se) = v {
                        return se.tag == right;
                    }
                }
            }
        }
    }
    false
}

fn exec_statements(entity: &mut EntityInstance, body: &str, params: &HashMap<String, Value>) {
    // split by semicolon; handle simple function calls and expressions
    for stmt in body.split(';') {
        let s = stmt.trim();
        if s.is_empty() { continue; }
        if s.starts_with("move(") {
            if s.contains("velocity * dt") {
                if let Some(Value::Float(dt)) = params.get("dt") {
                    let dist = entity.velocity * dt;
                    entity.position += dist;
                    println!("{} moves by {} (velocity {} * dt {})", entity.name, dist, entity.velocity, dt);
                }
            }
        } else if s.starts_with("collide(") || s == "collide()" {
            println!("{} collided (simulated)", entity.name);
        } else if s.starts_with("takeDamage(") {
            if let Some(open) = s.find('(') {
                if let Some(close) = s.find(')') {
                    let num = s[open+1..close].trim().trim_matches('"');
                    if let Ok(v) = num.parse::<i32>() {
                        entity.health -= v;
                        println!("{} takes {} damage, health -> {}", entity.name, v, entity.health);
                    }
                }
            }
        } else {
            println!("Unrecognized stmt: '{}'", s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta_lang::Event;

    #[test]
    fn test_take_damage() {
        let mut e = EntityInstance::new("P", "Ptag");
        let ev = Event { name: "Hit".to_string(), params: None, body: "takeDamage(5)".to_string() };
        execute_event(&mut e, &ev, &HashMap::new());
        assert_eq!(e.health, 95);
    }
}
