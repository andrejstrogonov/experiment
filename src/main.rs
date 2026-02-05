mod meta_lang;
mod runtime;
mod analyzer;
mod ast;
mod supercompiler;
mod aot_generator;

use meta_lang::parse_entities;
use runtime::{EntityInstance, Value, execute_event};
use analyzer::analyze_corpus;

fn main() {
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
    println!("Parsed {} entities:\n{:#?}", entities.len(), entities);

    // Create runtime instances
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

    println!("Player after events: {:?}", player);

    // Analyze corpus folder for frequent constructs
    println!("\n--- Corpus analysis ---");
    analyze_corpus("corpus");

    // Run supercompiler simplification and analyze simplified corpus
    println!("\n--- Supercompiler: simplifying corpus -> corpus_simplified ---");
    let (rep, chosen) = supercompiler::simplify_corpus("corpus", "corpus_simplified");
    println!("Supercompiler produced {} helpers, total replacements {}", rep.helper_count, rep.total_replacements);
    println!("Chosen hot sequences: {:?}", chosen);
    println!("\n--- Analysis of simplified corpus ---");
    analyze_corpus("corpus_simplified");

    // If we have hot sequences, generate AOT benchmark
    if !chosen.is_empty() {
        aot_generator::generate_and_run_aot(&chosen);
    } else {
        println!("No hot sequences selected for AOT generation.");
    }
}
