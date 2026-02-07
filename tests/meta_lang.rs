use experiment::meta_lang::{parse_entities, Entity};

#[test]
fn parse_sample_entity() {
    let s = r#"entity A { components: [X, Y]; on Tick() { do(); } }"#;
    let es: Vec<Entity> = parse_entities(s);
    assert_eq!(es.len(), 1);
    let e = &es[0];
    assert_eq!(e.name, "A");
    assert_eq!(e.events.len(), 1);
    assert_eq!(e.events[0].name, "Tick");
}
