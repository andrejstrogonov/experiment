use experiment::game_engine::GameEngine;

#[test]
fn create_and_load_scene() {
    let mut engine = GameEngine::new();
    let meta = r#"entity Test { on Update(dt) { move(velocity * dt); } }"#;
    assert!(engine.create_scene_from_meta("S1", meta).is_ok());
    assert!(engine.load_scene("S1").is_ok());
    assert_eq!(engine.current_scene.as_deref(), Some("S1"));
    engine.update(0.016);
    assert!(engine.time.frame_count > 0);
}
