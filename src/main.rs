mod meta_lang;
mod runtime;
mod analyzer;
mod ast;
mod supercompiler;
mod aot_generator;
mod game_engine;
mod components;
mod scene;
mod systems;

use meta_lang::parse_entities;
use runtime::{EntityInstance, Value, execute_event};
use game_engine::{GameEngine, Node, Component};

fn main() {
    println!("=== META GAME ENGINE MVP ===\n");

    // ============= PART 1: meta_lang demonstration =============
    println!("--- Part 1: Meta Language Parsing ---");
    
    let sample = r#"
entity Player {
    components: [Transform, Sprite, Physics, Input];

    on Update(dt) {
        move(velocity * dt);
        collide();
    }

    on Collision(other) {
        if (other.tag == "Enemy") {
            takeDamage(10);
        }
    }
}
"#;

    let entities = parse_entities(sample);
    println!("Parsed {} entities:\n{:#?}\n", entities.len(), entities);

    // ============= PART 2: Runtime execution demonstration =============
    println!("--- Part 2: Runtime Execution ---");
    
    let mut player = EntityInstance::new("Player", "PlayerTag");
    player.velocity = 5.0;
    player.health = 100;

    let mut enemy = EntityInstance::new("Enemy", "Enemy");
    enemy.health = 50;

    // Execute Update(dt)
    if let Some(ent) = entities.iter().find(|e| e.name == "Player") {
        if let Some(evt) = ent.events.iter().find(|ev| ev.name == "Update") {
            let mut params = std::collections::HashMap::new();
            params.insert("dt".to_string(), Value::Float(0.16));
            execute_event(&mut player, evt, &params);
        }

        if let Some(evt) = ent.events.iter().find(|ev| ev.name == "Collision") {
            let mut params = std::collections::HashMap::new();
            params.insert("other".to_string(), Value::EntitySnapshot(enemy.snapshot()));
            execute_event(&mut player, evt, &params);
        }
    }

    println!("Player after events: {:?}\n", player);

    // ============= PART 3: Game Engine MVP =============
    println!("--- Part 3: Game Engine MVP ---");
    run_game_engine();

    // ============= PART 4: Corpus analysis (commented out for now) =============
    // println!("\n--- Part 4: Corpus Analysis ---");
    // analyze_corpus("corpus");
    // 
    // println!("\n--- Supercompiler: simplifying corpus ---");
    // let (rep, chosen) = supercompiler::simplify_corpus("corpus", "corpus_simplified");
    // println!("Supercompiler produced {} helpers", rep.helper_count);
    // println!("Chosen hot sequences: {:?}", chosen);
}

fn run_game_engine() {
    let mut engine = GameEngine::new();

    // Создаем сцену из метаязыка
    let game_meta = r#"
entity Player {
    components: [Transform, Sprite, Physics];
    
    on Update(dt) {
        move(velocity * dt);
    }
}

entity Enemy {
    components: [Transform, Sprite, Health];
    
    on Update(dt) {
        patrol();
    }
}
"#;

    if let Ok(()) = engine.create_scene_from_meta("GameScene", game_meta) {
        println!("✓ Scene created from meta language");
    }

    // Загружаем сцену
    if let Ok(()) = engine.load_scene("GameScene") {
        println!("✓ Scene loaded");
    }

    // Создаем игровые объекты (ноды)
    {
        if let Some(scene) = engine.scenes.get_mut("GameScene") {
            // Player объект
            let mut player_node = Node::new(
                "player_1".to_string(),
                "Player".to_string(),
                "CharacterBody2D".to_string(),
            );

            // Добавляем компоненты
            let mut transform = Component::new("Transform".to_string());
            transform.set_property("position".to_string(), Value::Float(0.0));
            player_node.add_component(transform);

            let mut sprite = Component::new("Sprite".to_string());
            sprite.set_property("texture".to_string(), Value::Str("player.png".to_string()));
            player_node.add_component(sprite);

            let mut physics = Component::new("Physics".to_string());
            physics.set_property("velocity".to_string(), Value::Float(5.0));
            player_node.add_component(physics);

            // Добавляем инстанс для выполнения событий метаязыка
            player_node.instance = Some(EntityInstance::new("Player", "player"));

            // Enemy объект
            let mut enemy_node = Node::new(
                "enemy_1".to_string(),
                "Enemy".to_string(),
                "CharacterBody2D".to_string(),
            );

            let mut enemy_transform = Component::new("Transform".to_string());
            enemy_transform.set_property("position".to_string(), Value::Float(10.0));
            enemy_node.add_component(enemy_transform);

            let mut enemy_health = Component::new("Health".to_string());
            enemy_health.set_property("hp".to_string(), Value::Int(50));
            enemy_node.add_component(enemy_health);

            enemy_node.instance = Some(EntityInstance::new("Enemy", "enemy"));

            // Добавляем ноды в сцену
            scene.root.add_child(player_node);
            scene.root.add_child(enemy_node);

            println!("✓ Game objects created: Player, Enemy");
        }
    }

    // Симуляция игровых кадров
    println!("✓ Starting game loop simulation (5 frames)...\n");
    for frame in 0..5 {
        engine.update(0.016); // 60 FPS = 16.6ms per frame
        
        if let Some(scene_name) = &engine.current_scene.clone() {
            if let Some(scene) = engine.scenes.get(scene_name) {
                print!("Frame {}: ", frame);
                print!("Time={:.3}s ", engine.time.total_time);
                print!("Root children={} ", scene.root.children.len());
                
                // Выводим информацию об объектах
                for (idx, child) in scene.root.children.iter().enumerate() {
                    if idx > 0 { print!(", "); }
                    print!("{}", child.name);
                }
                println!();
            }
        }
    }

    println!("\n✓ Game engine MVP demonstration complete");
    println!("  - Scene system: ✓");
    println!("  - Component system: ✓");
    println!("  - Meta language integration: ✓");
    println!("  - Game loop: ✓");
}

