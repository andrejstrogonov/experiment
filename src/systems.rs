use std::collections::HashMap;
use crate::runtime::Value;

/// Система событий в движке
pub struct EventSystem {
    listeners: HashMap<String, Vec<EventListener>>,
}

/// Слушатель события
pub struct EventListener {
    pub event_type: String,
    pub handler: Box<dyn Fn(&Event) + Send + Sync>,
}

/// Данные события
pub struct Event {
    pub event_type: String,
    pub source: String,
    pub data: HashMap<String, Value>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event_type: String, listener: EventListener) {
        self.listeners.entry(event_type).or_insert_with(Vec::new).push(listener);
    }

    pub fn emit(&self, event: Event) {
        if let Some(listeners) = self.listeners.get(&event.event_type) {
            for listener in listeners {
                (listener.handler)(&event);
            }
        }
    }
}

impl Default for EventSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Система поведения для сущностей
pub struct BehaviorTree {
    pub name: String,
    pub nodes: Vec<BehaviorNode>,
    pub root_id: usize,
}

pub enum BehaviorNode {
    /// Успешно завершилась сразу
    Success,
    /// Неудача
    Failure,
    /// Выполняется действие с указанным именем
    Action { name: String },
    /// Выполнить все дети последовательно
    Sequence { children: Vec<usize> },
    /// Выполнить первого успешного ребенка
    Selector { children: Vec<usize> },
    /// Условный узел
    Condition { check: String, true_child: usize, false_child: usize },
}

impl BehaviorTree {
    pub fn new(name: String) -> Self {
        Self {
            name,
            nodes: Vec::new(),
            root_id: 0,
        }
    }

    pub fn add_node(&mut self, node: BehaviorNode) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    pub fn set_root(&mut self, id: usize) {
        self.root_id = id;
    }
}

/// Система состояний для персонажей
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityState {
    Idle,
    Moving,
    Attacking,
    TakingDamage,
    Dead,
    Custom(String),
}

pub struct StateMachine {
    pub current_state: EntityState,
    pub transitions: HashMap<EntityState, HashMap<String, EntityState>>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            current_state: EntityState::Idle,
            transitions: HashMap::new(),
        }
    }

    pub fn add_transition(&mut self, from: EntityState, trigger: String, to: EntityState) {
        self.transitions.entry(from).or_insert_with(HashMap::new).insert(trigger, to);
    }

    pub fn on_event(&mut self, trigger: &str) -> bool {
        if let Some(transitions) = self.transitions.get(&self.current_state) {
            if let Some(next_state) = transitions.get(trigger) {
                self.current_state = next_state.clone();
                return true;
            }
        }
        false
    }

    pub fn is_in_state(&self, state: &EntityState) -> bool {
        &self.current_state == state
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}
