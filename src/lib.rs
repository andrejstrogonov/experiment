pub mod meta_lang;
pub mod runtime;
pub mod analyzer;
pub mod ast;
pub mod supercompiler;
pub mod aot_generator;
pub mod game_engine;
pub mod components;
pub mod scene;
pub mod systems;
pub mod renderer;

// Re-export commonly used types for tests and external use
pub use meta_lang::parse_entities;
pub use runtime::{EntityInstance, Value, execute_event};
pub use game_engine::{GameEngine, Node, Component};
