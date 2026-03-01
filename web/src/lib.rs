use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d, Window};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;

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
struct EntityInstance {
    name: String,
    tag: String,
    health: i32,
    velocity: f64,
    position: f64,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
}

impl EntityInstance {
    fn new(name: &str, tag: &str) -> Self {
        Self {
            name: name.to_string(),
            tag: tag.to_string(),
            health: 100,
            velocity: 0.0,
            position: 0.0,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
        }
    }
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
                if let Some(Value::Float(dt)) = params.get("dt") {
                    let dist = entity.velocity * dt;
                    entity.position += dist;
                    web_sys::console::log_1(&format!("{} moves by {}", entity.name, dist).into());
                }
            }
        } else if s.starts_with("rotateX(") {
            if let Some(open) = s.find('(') {
                if let Some(close) = s.find(')') {
                    if let Ok(val) = s[open+1..close].trim().parse::<f64>() {
                        entity.rotation_x += val;
                    }
                }
            }
        } else if s.starts_with("rotateY(") {
            if let Some(open) = s.find('(') {
                if let Some(close) = s.find(')') {
                    if let Ok(val) = s[open+1..close].trim().parse::<f64>() {
                        entity.rotation_y += val;
                    }
                }
            }
        } else if s.starts_with("rotateZ(") {
            if let Some(open) = s.find('(') {
                if let Some(close) = s.find(')') {
                    if let Ok(val) = s[open+1..close].trim().parse::<f64>() {
                        entity.rotation_z += val;
                    }
                }
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

#[wasm_bindgen]
pub fn start_app(canvas_id: &str, script: &str) {
    let window = window().expect("no global window");
    let document = window.document().expect("no document");
    let canvas = document.get_element_by_id(canvas_id).expect("canvas not found");
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().expect("not a canvas");
    let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();

    // parse entities and instantiate first entity from script
    let entities = parse_entities(script);
    let mut ent = if !entities.is_empty() {
        EntityInstance::new(&entities[0].0, "Player")
    } else {
        EntityInstance::new("Plane","Player")
    };
    ent.position = 200.0;
    ent.velocity = 0.0;
    ent.health = 100;
    // find Tick event for the selected entity
    let mut tick_event: Option<Event> = None;
    for (ename, evs) in entities.iter() {
        if ename == &ent.name || tick_event.is_none() {
            for e in evs.iter() {
                if e.name == "Tick" {
                    tick_event = Some(e.clone());
                }
            }
        }
    }

    let mut last = js_sys::Date::now();

    // animation loop using recursion through request_animation_frame
    let ctx = Rc::new(ctx);
    let plane = Rc::new(RefCell::new(ent));
    let tick_event_ref = std::rc::Rc::new(tick_event);

    fn schedule_frame(
        window: &web_sys::Window,
        ctx: Rc<CanvasRenderingContext2d>,
        plane: Rc<RefCell<EntityInstance>>,
        tick_event: Rc<Option<Event>>,
        last_time: Rc<RefCell<f64>>,
        canvas: HtmlCanvasElement,
    ) {
        let ctx_clone = ctx.clone();
        let plane_clone = plane.clone();
        let tick_event_clone = tick_event.clone();
        let canvas_clone = canvas.clone();

        let closure = Closure::once(Box::new(move || {
            let now = js_sys::Date::now();
            let mut last_mut = last_time.borrow_mut();
            let dt = (now - *last_mut) / 1000.0;
            *last_mut = now;
            drop(last_mut); // release borrow

            // execute script Tick
            if let Some(ev) = tick_event_clone.as_ref() {
                let mut params = HashMap::new();
                params.insert("dt".to_string(), Value::Float(dt));
                params.insert("other".to_string(), Value::EntitySnapshot("Enemy".to_string(), "EnemyTag".to_string(), 50));
                {
                    let mut p = plane_clone.borrow_mut();
                    execute_event(&mut p, ev, &params);
                }
            }

            // render entity (plane or cube) and hud
            {
                let p = plane_clone.borrow();
                let w = canvas_clone.width() as f64;
                let h = canvas_clone.height() as f64;
                ctx_clone.set_fill_style(&JsValue::from_str("#0b1220"));
                ctx_clone.fill_rect(0.0, 0.0, w, h);
                if p.name == "Cube" {
                    // rotate and draw centered square with simple 3-axis effect
                    ctx_clone.save();
                    ctx_clone.translate(w/2.0, h/2.0).ok();
                    // apply z rotation
                    ctx_clone.rotate(p.rotation_z).ok();
                    // simulate x/y tilt by scaling
                    let sx = p.rotation_y.cos();
                    let sy = p.rotation_x.cos();
                    ctx_clone.scale(sx, sy).ok();
                    let size = 50.0;
                    ctx_clone.set_fill_style(&JsValue::from_str("#4ade80"));
                    ctx_clone.fill_rect(-size/2.0, -size/2.0, size, size);
                    ctx_clone.restore();
                } else {
                    ctx_clone.set_fill_style(&JsValue::from_str("#f97316"));
                    ctx_clone.begin_path();
                    let x = p.position;
                    let y = h / 2.0;
                    ctx_clone.move_to(x as f64, y as f64 - 10.0);
                    ctx_clone.line_to(x as f64 + 30.0, y as f64);
                    ctx_clone.line_to(x as f64, y as f64 + 10.0);
                    ctx_clone.close_path();
                    ctx_clone.fill();
                }

                // draw HUD
                ctx_clone.set_fill_style(&JsValue::from_str("white"));
                ctx_clone.fill_text(
                    &format!("pos: {:.2} vel: {:.2} hp: {} rotX:{:.2} rotY:{:.2} rotZ:{:.2}",
                             p.position, p.velocity, p.health, p.rotation_x, p.rotation_y, p.rotation_z),
                    10.0,
                    20.0,
                ).ok();
            }

            // schedule next frame
            let window = web_sys::window().unwrap();
            schedule_frame(&window, ctx_clone, plane_clone, tick_event_clone, last_time.clone(), canvas_clone);
        }));

        window.request_animation_frame(closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }

    schedule_frame(&window, ctx, plane, tick_event_ref, Rc::new(RefCell::new(last)), canvas);
}

// -- unit tests -------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn parse_simple_script() {
        let script = "entity Plane { on Tick(dt) { move(velocity * dt); } }";
        let entities = parse_entities(script);
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].0, "Plane");
        assert_eq!(entities[0].1.len(), 1);
        assert_eq!(entities[0].1[0].name, "Tick");
    }

    #[test]
    fn rotation_statements_affect_entity() {
        let mut entity = EntityInstance::new("Cube", "Tag");
        exec_statements(&mut entity, "rotateX(1.5); rotateY(2.5); rotateZ(3.5);", &HashMap::new());
        assert!((entity.rotation_x - 1.5).abs() < 1e-6);
        assert!((entity.rotation_y - 2.5).abs() < 1e-6);
        assert!((entity.rotation_z - 3.5).abs() < 1e-6);
    }
}
