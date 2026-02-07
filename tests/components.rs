use experiment::components::Health;

#[test]
fn health_take_heal() {
    let mut h = Health::new(100.0);
    h.take_damage(30.0);
    assert_eq!(h.current, 70.0);
    h.heal(20.0);
    assert_eq!(h.current, 90.0);
}
