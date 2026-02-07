use std::collections::HashMap;
use crate::game_engine::Node;
use crate::meta_lang::Entity;

/// Система сцен для управления игровыми объектами
pub struct SceneManager {
    pub scenes: HashMap<String, GameScene>,
    pub active_scene: Option<String>,
}

/// Полное описание сцены
pub struct GameScene {
    pub name: String,
    pub root: Node,
    pub meta_entities: Vec<Entity>,
    pub physics_enabled: bool,
    pub render_enabled: bool,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            active_scene: None,
        }
    }

    pub fn add_scene(&mut self, scene: GameScene) {
        self.scenes.insert(scene.name.clone(), scene);
    }

    pub fn get_scene(&self, name: &str) -> Option<&GameScene> {
        self.scenes.get(name)
    }

    pub fn get_scene_mut(&mut self, name: &str) -> Option<&mut GameScene> {
        self.scenes.get_mut(name)
    }

    pub fn set_active_scene(&mut self, name: String) -> Result<(), String> {
        if self.scenes.contains_key(&name) {
            self.active_scene = Some(name);
            Ok(())
        } else {
            Err(format!("Scene '{}' not found", name))
        }
    }

    pub fn get_active_scene(&self) -> Option<&GameScene> {
        self.active_scene.as_ref().and_then(|name| self.get_scene(name))
    }

    pub fn get_active_scene_mut(&mut self) -> Option<&mut GameScene> {
        let name = self.active_scene.clone()?;
        self.get_scene_mut(&name)
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GameScene {
    pub fn new(name: String) -> Self {
        Self {
            name,
            root: Node::new("root".to_string(), "Root".to_string(), "Node".to_string()),
            meta_entities: Vec::new(),
            physics_enabled: true,
            render_enabled: true,
        }
    }

    pub fn with_physics(mut self, enabled: bool) -> Self {
        self.physics_enabled = enabled;
        self
    }

    pub fn with_render(mut self, enabled: bool) -> Self {
        self.render_enabled = enabled;
        self
    }
}
