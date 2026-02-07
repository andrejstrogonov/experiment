use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Event { name: String, params: Option<String>, body: String }

fn trim_semicolon(s: &str) -> &str { let s = s.trim(); if s.ends_with(';') { &s[..s.len()-1] } else { s } }

fn parse_entities(input: &str) -> Vec<(String, Vec<Event>)> {
    let mut res = Vec::new();
    let mut i = 0usize;
    let bytes = input.as_bytes();
    let n = bytes.len();
    while i < n {
        if input[i..].starts_with("entity") {
            i += "entity".len();
            while i < n && input.as_bytes()[i].is_ascii_whitespace() { i += 1; }
            let start = i;
            while i < n { let c = input.as_bytes()[i] as char; if c.is_whitespace() || c == '{' { break; } i += 1; }
            let name = input[start..i].trim().to_string();
            while i < n && input.as_bytes()[i] != b'{' { i += 1; }
            if i >= n { break; }
            i += 1; let mut brace_level = 1; let block_start = i;
            while i < n && brace_level > 0 { match input.as_bytes()[i] { b'{' => brace_level += 1, b'}' => brace_level -= 1, _=>{} } i += 1; }
            let block_end = i - 1; let block = &input[block_start..block_end];
            // parse events
            let mut events = Vec::new();
            let mut j = 0usize;
            while j < block.len() {
                if block[j..].starts_with("on ") {
                    j += 3; while j < block.len() && block.as_bytes()[j].is_ascii_whitespace() { j += 1; }
                    let en_start = j; while j < block.len() { let c = block.as_bytes()[j] as char; if c.is_whitespace() || c == '(' { break; } j += 1; }
                    let en_name = block[en_start..j].trim().to_string();
                    let mut params = None;
                    if j < block.len() && block.as_bytes()[j] == b'(' { j += 1; let pstart = j; while j < block.len() && block.as_bytes()[j] != b')' { j += 1; } params = Some(block[pstart..j].trim().to_string()); if j < block.len() { j += 1; } }
                    while j < block.len() && block.as_bytes()[j] != b'{' { j += 1; }
                    if j >= block.len() { break; }
                    j += 1; let mut lev = 1usize; let bstart = j; while j < block.len() && lev > 0 { match block.as_bytes()[j] { b'{' => lev += 1, b'}' => lev -= 1, _=>{} } j += 1; }
                    let bend = j - 1; let body = block[bstart..bend].trim().to_string();
                    events.push(Event { name: en_name, params, body });
                } else { j += 1; }
            }
            res.push((name, events));
        } else { i += 1; }
    }
    res
}

#[derive(Clone)]
struct EntityInstance { name: String, tag: String, health: i32, velocity: f64, position: f64 }

impl EntityInstance {
    fn new(name: &str, tag: &str) -> Self { Self { name: name.to_string(), tag: tag.to_string(), health: 100, velocity: 0.0, position: 0.0 } }
}

enum Value { Float(f64), Int(i64), Str(String), EntitySnapshot(String, String, i32) }

fn eval_condition(cond: &str, params: &HashMap<String, Value>) -> bool {
    if cond.contains("==") {
        let parts: Vec<&str> = cond.split("==").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let left = parts[0];
            let right = parts[1].trim_matches('"');
            if left.ends_with(".tag") {
                let var = left.trim_end_matches(".tag");
                if let Some(v) = params.get(var) {
                    if let Value::EntitySnapshot(_, tag, _) = v { return tag == right; }
                }
            }
        }
    }
    false
}

fn exec_statements(entity: &mut EntityInstance, body: &str, params: &HashMap<String, Value>) {
    for stmt in body.split(';') {
        let s = stmt.trim(); if s.is_empty() { continue; }
        if s.starts_with("move(") {
            if s.contains("velocity * dt") {
                if let Some(Value::Float(dt)) = params.get("dt") { let dist = entity.velocity * dt; entity.position += dist; web_sys::console::log_1(&format!("{} moves by {}", entity.name, dist).into()); }
            }
        } else if s.starts_with("takeDamage(") {
            if let Some(open) = s.find('(') { if let Some(close) = s.find(')') { let num = s[open+1..close].trim().trim_matches('"'); if let Ok(v) = num.parse::<i32>() { entity.health -= v; web_sys::console::log_1(&format!("{} takes {} damage", entity.name, v).into()); } } }
        } else if s.starts_with("collide(") || s == "collide()" {
            web_sys::console::log_1(&format!("{} collided", entity.name).into());
        } else {
            web_sys::console::log_1(&format!("Unrecognized stmt: {}", s).into());
        }
    }
}

fn execute_event(entity: &mut EntityInstance, event: &Event, params: &HashMap<String, Value>) {
    let body = event.body.trim();
    if body.starts_with("if ") {
        if let Some(cond_start) = body.find('(') { if let Some(cond_end) = body.find(')') { let cond = &body[cond_start+1..cond_end]; let inner_start = body.find('{').unwrap_or(body.len()); let inner_end = body.rfind('}').unwrap_or(body.len()); let inner = &body[inner_start+1..inner_end]; if eval_condition(cond, params) { exec_statements(entity, inner, params); } return; } }
    }
    exec_statements(entity, body, params);
}

use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
pub fn start_app(canvas_id: &str, script: &str) {
    let window = window().expect("no global window");
    let document = window.document().expect("no document");
    let canvas = document.get_element_by_id(canvas_id).expect("canvas not found");
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().expect("not a canvas");
    let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();

    // parse entities and find plane
    let entities = parse_entities(script);
    let mut plane = EntityInstance::new("Plane","Player");
    plane.position = 200.0; plane.velocity = 0.0; plane.health = 100;
    // find Tick event for the entity named "Plane" or the first entity
    let mut tick_event: Option<Event> = None;
    for (ename, evs) in entities.iter() {
        if ename == "Plane" || tick_event.is_none() {
            for e in evs.iter() { if e.name == "Tick" { tick_event = Some(e.clone()); } }
        }
    }

    let mut last = js_sys::Date::now();

    // animation loop closure
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let ctx = Rc::new(ctx);
    let plane = Rc::new(RefCell::new(plane));
    let tick_event = tick_event;

    // clone `f` to move into the closure for scheduling the next frame
    let f_inner = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let now = js_sys::Date::now();
        let dt = (now - last) / 1000.0;
        last = now;

        // execute script Tick
        if let Some(ev) = &tick_event {
            let mut params = HashMap::new();
            params.insert("dt".to_string(), Value::Float(dt));
            params.insert("other".to_string(), Value::EntitySnapshot("Enemy".to_string(), "EnemyTag".to_string(), 50));
            let mut p = plane.borrow_mut();
            execute_event(&mut p, ev, &params);
        }

        // render simple airplane
        let ctx = ctx.clone();
        let mut p = plane.borrow_mut();
        let w = canvas.width() as f64; let h = canvas.height() as f64;
        ctx.set_fill_style(&JsValue::from_str("#0b1220"));
        ctx.fill_rect(0.0, 0.0, w, h);
        ctx.set_fill_style(&JsValue::from_str("#f97316"));
        ctx.begin_path();
        let x = p.position; let y = h/2.0;
        ctx.move_to(x as f64, y as f64 - 10.0);
        ctx.line_to(x as f64 + 30.0, y as f64);
        ctx.line_to(x as f64, y as f64 + 10.0);
        ctx.close_path();
        ctx.fill();

        // draw HUD
        ctx.set_fill_style(&JsValue::from_str("white"));
        ctx.fill_text(&format!("pos: {:.2} vel: {:.2} hp: {}", p.position, p.velocity, p.health), 10.0, 20.0).ok();

        // request next frame
        let window = web_sys::window().unwrap();
        let cb_opt = f_inner.borrow();
        if let Some(cb_ref) = cb_opt.as_ref() {
            window.request_animation_frame(cb_ref.as_ref().unchecked_ref()).unwrap();
        }
    }) as Box<dyn FnMut()>));

    // start loop
    let cb_opt = f.borrow();
    if let Some(cb_ref) = cb_opt.as_ref() {
        window.request_animation_frame(cb_ref.as_ref().unchecked_ref()).unwrap();
    }
}
