use std::collections::HashMap;
use crate::meta_lang::{Entity, parse_entities};
use crate::runtime::{EntityInstance, Value};

/// Основной игровой движок
pub struct GameEngine {
    pub scenes: HashMap<String, Scene>,
    pub current_scene: Option<String>,
    pub time: GameTime,
    pub running: bool,
}

/// Сцена содержит ноды и их иерархию
#[derive(Debug, Clone)]
pub struct Scene {
    pub name: String,
    pub root: Node,
    pub entities_meta: Vec<Entity>,
}

/// Нода в сцене (как в Godot)
#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub components: HashMap<String, Component>,
    pub children: Vec<Node>,
    pub active: bool,
    pub instance: Option<EntityInstance>,
}

/// Компонент - поведение ноды
#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub properties: HashMap<String, Value>,
}

/// Время в игре
#[derive(Debug, Clone, Copy)]
pub struct GameTime {
    pub delta_time: f64,
    pub total_time: f64,
    pub frame_count: u32,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene: None,
            time: GameTime {
                delta_time: 0.016, // 60 FPS
                total_time: 0.0,
                frame_count: 0,
            },
            running: true,
        }
    }

    /// Создает сцену из определения метаязыка
    pub fn create_scene_from_meta(
        &mut self,
        scene_name: &str,
        meta_definition: &str,
    ) -> Result<(), String> {
        let entities = parse_entities(meta_definition);

        // Создаем корневой нод для сцены
        let root = Node {
            id: format!("root_{}", scene_name),
            name: "Root".to_string(),
            node_type: "Node".to_string(),
            components: HashMap::new(),
            children: Vec::new(),
            active: true,
            instance: None,
        };

        let scene = Scene {
            name: scene_name.to_string(),
            root,
            entities_meta: entities,
        };

        self.scenes.insert(scene_name.to_string(), scene);
        Ok(())
    }

    /// Загружает сцену в движок
    pub fn load_scene(&mut self, scene_name: &str) -> Result<(), String> {
        if !self.scenes.contains_key(scene_name) {
            return Err(format!("Scene '{}' not found", scene_name));
        }
        self.current_scene = Some(scene_name.to_string());
        Ok(())
    }

    /// Выполняет один кадр игры
    pub fn update(&mut self, delta_time: f64) {
        self.time.delta_time = delta_time;
        self.time.total_time += delta_time;
        self.time.frame_count += 1;

        if let Some(scene_name) = self.current_scene.clone() {
            let scene_opt = self.scenes.get_mut(&scene_name);
            if let Some(scene) = scene_opt {
                let root = scene.root.clone();
                let mut updated_root = root;
                Self::update_node_static(&mut updated_root, delta_time);
                scene.root = updated_root;
            }
        }
    }

    fn update_node_static(node: &mut Node, delta_time: f64) {
        if !node.active {
            return;
        }

        // Обновляем текущий нод
        if let Some(_instance) = &mut node.instance {
            let mut params = HashMap::new();
            params.insert("dt".to_string(), Value::Float(delta_time));

            // Находим соответствующую сущность в метаязыке
            // и выполняем события
        }

        // Рекурсивно обновляем детей
        for child in &mut node.children {
            Self::update_node_static(child, delta_time);
        }
    }

    /// Добавляет нод в сцену
    pub fn add_node(&mut self, scene_name: &str, node: Node) -> Result<(), String> {
        if let Some(scene) = self.scenes.get_mut(scene_name) {
            scene.root.children.push(node);
            Ok(())
        } else {
            Err(format!("Scene '{}' not found", scene_name))
        }
    }

    /// Обрабатывает событие для ноды
    pub fn emit_signal(
        &mut self,
        _scene_name: &str,
        _node_id: &str,
        _signal_name: &str,
        _params: HashMap<String, Value>,
    ) -> Result<(), String> {
        // Находим нод по ID и выполняем его событие
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    pub fn new(id: String, name: String, node_type: String) -> Self {
        Self {
            id,
            name,
            node_type,
            components: HashMap::new(),
            children: Vec::new(),
            active: true,
            instance: None,
        }
    }

    /// Добавляет компонент к ноде
    pub fn add_component(&mut self, component: Component) {
        self.components.insert(component.name.clone(), component);
    }

    /// Получает компонент по имени
    pub fn get_component(&self, name: &str) -> Option<&Component> {
        self.components.get(name)
    }

    /// Добавляет дочерний нод
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    /// Находит дочерний нод по имени
    pub fn find_child(&self, name: &str) -> Option<&Node> {
        self.children.iter().find(|n| n.name == name)
    }
}

impl Component {
    pub fn new(name: String) -> Self {
        Self {
            name,
            properties: HashMap::new(),
        }
    }

    pub fn set_property(&mut self, key: String, value: Value) {
        self.properties.insert(key, value);
    }

    pub fn get_property(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }
}
