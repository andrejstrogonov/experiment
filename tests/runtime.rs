use experiment::runtime::{EntityInstance, execute_event, Value};
use experiment::meta_lang::Event;

#[test]
fn take_damage_exec() {
    let mut e = EntityInstance::new("P", "Ptag");
    let ev = Event { name: "Hit".to_string(), params: None, body: "takeDamage(5)".to_string() };
    execute_event(&mut e, &ev, &std::collections::HashMap::new());
    assert_eq!(e.health, 95);
}
