# META GAME ENGINE - Примеры использования

## 1. Базовый пример: Создание простой игры

```rust
use game_engine::{GameEngine, Node, Component};
use runtime::{Value, EntityInstance};

fn main() {
    // Инициализация движка
    let mut engine = GameEngine::new();
    
    // Определение сущностей через метаязык
    let game_definition = r#"
    entity Player {
        components: [Transform, Sprite, Physics, Health];
        
        on Update(dt) {
            move(velocity * dt);
        }
        
        on TakeDamage(amount) {
            health -= amount;
        }
    }
    
    entity Enemy {
        components: [Transform, Sprite, Health, AI];
        
        on Update(dt) {
            chase_player();
        }
    }
    "#;
    
    // Создание сцены
    engine.create_scene_from_meta("MainScene", game_definition).unwrap();
    engine.load_scene("MainScene").unwrap();
    
    // Создание игровых объектов
    if let Some(scene) = engine.scenes.get_mut("MainScene") {
        // Player
        let mut player = Node::new(
            "player".to_string(),
            "Player".to_string(),
            "CharacterBody2D".to_string(),
        );
        
        let mut transform = Component::new("Transform".to_string());
        transform.set_property("position".to_string(), Value::Float(0.0));
        player.add_component(transform);
        
        player.instance = Some(EntityInstance::new("Player", "player"));
        
        // Enemy
        let mut enemy = Node::new(
            "enemy".to_string(),
            "Enemy".to_string(),
            "CharacterBody2D".to_string(),
        );
        
        let mut transform = Component::new("Transform".to_string());
        transform.set_property("position".to_string(), Value::Float(10.0));
        enemy.add_component(transform);
        
        enemy.instance = Some(EntityInstance::new("Enemy", "enemy"));
        
        // Добавление объектов в сцену
        scene.root.add_child(player);
        scene.root.add_child(enemy);
    }
    
    // Game loop
    for frame in 0..300 {
        engine.update(0.016); // 60 FPS
        
        if frame % 60 == 0 {
            println!("Frame {}: Time = {:.2}s", frame, engine.time.total_time);
        }
    }
}
```

---

## 2. Пример с компонентами

```rust
use components::{Transform, Vector3, Health, Sprite, Physics};

fn create_player_node() -> Node {
    let mut player = Node::new(
        "player".to_string(),
        "Player".to_string(),
        "Player".to_string(),
    );
    
    // Transform компонент
    let mut transform = Component::new("Transform".to_string());
    let pos = transform.set_property("position".to_string(), Value::Float(0.0));
    let rot = transform.set_property("rotation".to_string(), Value::Float(0.0));
    let scale = transform.set_property("scale".to_string(), Value::Float(1.0));
    player.add_component(transform);
    
    // Sprite компонент
    let mut sprite = Component::new("Sprite".to_string());
    sprite.set_property("texture".to_string(), Value::Str("player.png".to_string()));
    sprite.set_property("visible".to_string(), Value::Float(1.0));
    player.add_component(sprite);
    
    // Physics компонент
    let mut physics = Component::new("Physics".to_string());
    physics.set_property("velocity".to_string(), Value::Float(5.0));
    physics.set_property("mass".to_string(), Value::Float(1.0));
    player.add_component(physics);
    
    player
}
```

---

## 3. Пример с Event System

```rust
use systems::EventSystem;

fn setup_events() {
    let mut event_system = EventSystem::new();
    
    // Подписка на события
    event_system.subscribe(
        "collision".to_string(),
        EventListener {
            event_type: "collision".to_string(),
            handler: Box::new(|event| {
                println!("Collision detected: {:?}", event.source);
            }),
        },
    );
    
    // Отправка события
    let mut event_data = HashMap::new();
    event_data.insert("other".to_string(), Value::Str("Enemy".to_string()));
    
    let event = systems::Event {
        event_type: "collision".to_string(),
        source: "Player".to_string(),
        data: event_data,
    };
    
    event_system.emit(event);
}
```

---

## 4. Пример с State Machine

```rust
use systems::{StateMachine, EntityState};

fn create_character_state_machine() -> StateMachine {
    let mut state_machine = StateMachine::new();
    
    // Переходы между состояниями
    state_machine.add_transition(
        EntityState::Idle,
        "move".to_string(),
        EntityState::Moving,
    );
    
    state_machine.add_transition(
        EntityState::Moving,
        "stop".to_string(),
        EntityState::Idle,
    );
    
    state_machine.add_transition(
        EntityState::Idle,
        "attack".to_string(),
        EntityState::Attacking,
    );
    
    state_machine.add_transition(
        EntityState::Attacking,
        "finish".to_string(),
        EntityState::Idle,
    );
    
    state_machine.add_transition(
        EntityState::Idle,
        "hit".to_string(),
        EntityState::TakingDamage,
    );
    
    state_machine.add_transition(
        EntityState::TakingDamage,
        "recover".to_string(),
        EntityState::Idle,
    );
    
    state_machine
}

fn update_character(state_machine: &mut StateMachine, input: &str) {
    if state_machine.on_event(input) {
        println!("State changed to: {:?}", state_machine.current_state);
    }
}
```

---

## 5. Пример с Behavior Tree

```rust
use systems::{BehaviorTree, BehaviorNode};

fn create_enemy_ai() -> BehaviorTree {
    let mut tree = BehaviorTree::new("EnemyAI".to_string());
    
    // Построить дерево поведения
    let check_health = tree.add_node(BehaviorNode::Condition {
        check: "health > 0".to_string(),
        true_child: 1,
        false_child: 2,
    });
    
    let patrol = tree.add_node(BehaviorNode::Action {
        name: "patrol".to_string(),
    });
    
    let death = tree.add_node(BehaviorNode::Action {
        name: "die".to_string(),
    });
    
    tree.set_root(check_health);
    tree
}
```

---

## 6. Интеграция: Полная сцена с несколькими объектами

```rust
fn create_scene_with_multiple_objects(engine: &mut GameEngine) -> Result<(), String> {
    let scene_def = r#"
    entity Player {
        components: [Transform, Sprite, Physics, Input, Health];
        
        on Update(dt) {
            handle_input();
            move(velocity * dt);
        }
        
        on Collision(other) {
            if (other.tag == "Enemy") {
                takeDamage(10);
            }
        }
    }
    
    entity Enemy {
        components: [Transform, Sprite, Health, AI];
        
        on Update(dt) {
            ai_think();
            move(velocity * dt);
        }
        
        on Collision(other) {
            if (other.tag == "Player") {
                attack();
            }
        }
    }
    
    entity Item {
        components: [Transform, Sprite];
        
        on Collision(other) {
            if (other.tag == "Player") {
                pickup();
            }
        }
    }
    "#;
    
    engine.create_scene_from_meta("Level1", scene_def)?;
    engine.load_scene("Level1")?;
    
    if let Some(scene) = engine.scenes.get_mut("Level1") {
        // Создать игроков
        for i in 0..3 {
            let enemy = create_enemy_node(i);
            scene.root.add_child(enemy);
        }
        
        // Создать предметы
        for i in 0..5 {
            let item = create_item_node(i);
            scene.root.add_child(item);
        }
    }
    
    Ok(())
}
```

---

## 7. Работа с multiple сцен

```rust
fn handle_scene_transitions(engine: &mut GameEngine) {
    // Создать несколько сцен
    engine.create_scene_from_meta("MainMenu", r#"
        entity Button { components: [Transform, Sprite]; }
    "#).unwrap();
    
    engine.create_scene_from_meta("Level1", game_level_definition).unwrap();
    engine.create_scene_from_meta("GameOver", game_over_definition).unwrap();
    
    // Переключаться между сценами
    engine.load_scene("MainMenu").unwrap();
    
    // ... play game ...
    
    if game_won {
        engine.load_scene("GameOver").unwrap();
    }
}
```

---

## 8. Custom Components и Logic

```rust
// Расширение: создание собственного компонента
pub struct CustomAIComponent {
    pub patrol_points: Vec<(f32, f32)>,
    pub current_target: usize,
    pub speed: f32,
}

fn create_custom_enemy() -> Node {
    let mut enemy = Node::new(
        "enemy_custom".to_string(),
        "CustomEnemy".to_string(),
        "CharacterBody2D".to_string(),
    );
    
    let mut custom_ai = Component::new("CustomAI".to_string());
    custom_ai.set_property("speed".to_string(), Value::Float(3.5));
    custom_ai.set_property("patrol_range".to_string(), Value::Float(10.0));
    
    enemy.add_component(custom_ai);
    enemy
}
```

---

## 9. Отладка и インspection

```rust
fn inspect_scene(engine: &GameEngine, scene_name: &str) {
    if let Some(scene) = engine.scenes.get(scene_name) {
        println!("=== Scene: {} ===", scene_name);
        println!("Root: {}", scene.root.name);
        println!("Children: {}", scene.root.children.len());
        
        for child in &scene.root.children {
            println!("  - {}: {} components", child.name, child.components.len());
            for (comp_name, comp) in &child.components {
                println!("    - {}: {} properties", comp_name, comp.properties.len());
            }
        }
        
        println!("Meta entities: {}", scene.entities_meta.len());
        for entity in &scene.entities_meta {
            println!("  - {}: {} components, {} events", 
                entity.name, 
                entity.components.len(), 
                entity.events.len()
            );
        }
    }
}
```

---

## 10. Performance и Optimization

```rust
fn run_game_loop_optimized(engine: &mut GameEngine, max_frames: u32) {
    let target_fps = 60;
    let frame_time = 1.0 / target_fps as f64;
    
    let mut frames = 0;
    let start_time = std::time::Instant::now();
    
    while engine.is_running() && frames < max_frames {
        let frame_start = std::time::Instant::now();
        
        engine.update(frame_time);
        
        // Профилирование
        let frame_duration = frame_start.elapsed().as_secs_f64();
        if frame_duration > frame_time {
            println!("Frame {} took {:.4}ms (target: {:.4}ms)", 
                frames, 
                frame_duration * 1000.0,
                frame_time * 1000.0
            );
        }
        
        frames += 1;
    }
    
    let total_time = start_time.elapsed().as_secs_f64();
    println!("Total frames: {}", frames);
    println!("Total time: {:.2}s", total_time);
    println!("Average FPS: {:.1}", frames as f64 / total_time);
}
```

---

## Заключение

Эти примеры демонстрируют основные возможности META GAME ENGINE MVP:

✅ Парсинг метаязыка  
✅ Создание и управление сценами  
✅ Компоненты и ноды  
✅ События и системы  
✅ State Management  
✅ Поведение и AI  
✅ Отладка и анализ  

**Готово к расширению и улучшению!**
