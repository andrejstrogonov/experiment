# META GAME ENGINE MVP

## Обзор

Построен MVP игрового движка на основе **метаязыка** (из предыдущего этапа), вдохновленный архитектурой **Godot Engine**. Проект интегрирует:

- ✅ **Meta Language Parser** - парсинг определения сущностей
- ✅ **Runtime Executor** - выполнение событий в реальном времени
- ✅ **Game Engine** - система сцен и нод
- ✅ **Component System** - переиспользуемые компоненты
- ✅ **Event System** - система событий (готова к использованию)
- ✅ **State Machine** - управление состояниями (готова к использованию)

---

## Архитектура

### 1. **Meta Language** (`meta_lang.rs`)
Парсит определения сущностей в специальном формате:
```rust
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
```

**Основные структуры:**
- `Entity` - определение сущности с компонентами и событиями
- `Event` - описание события с параметрами и телом

---

### 2. **Game Engine** (`game_engine.rs`)
Основное ядро движка со всеми основными компонентами.

#### `GameEngine`
Главный контроллер игры:
```rust
pub struct GameEngine {
    pub scenes: HashMap<String, Scene>,
    pub current_scene: Option<String>,
    pub time: GameTime,
    pub running: bool,
}
```

**Основные методы:**
- `create_scene_from_meta()` - создание сцены из метаязыка
- `load_scene()` - загрузка активной сцены
- `update()` - обновление кадра (60 FPS по умолчанию)

#### `Scene`
Представляет игровую сцену:
```rust
pub struct Scene {
    pub name: String,
    pub root: Node,
    pub entities_meta: Vec<Entity>,
}
```

#### `Node`
Базовый объект в сцене (как в Godot):
```rust
pub struct Node {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub components: HashMap<String, Component>,
    pub children: Vec<Node>,
    pub active: bool,
    pub instance: Option<EntityInstance>,
}
```

**Основные методы:**
- `add_component()` - добавить компонент
- `add_child()` - добавить дочерний нод
- `find_child()` - найти дочерний нод по имени

#### `Component`
Переиспользуемое поведение:
```rust
pub struct Component {
    pub name: String,
    pub properties: HashMap<String, Value>,
}
```

---

### 3. **Components** (`components.rs`)
Встроенные компоненты для объектов:

- **Transform** - позиция, ротация, масштаб
- **Velocity** - вектор скорости
- **Health** - система здоровья с уроном и лечением
- **Sprite** - графическое отображение
- **Physics** - параметры физики
- **Input** - обработка ввода

```rust
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn take_damage(&mut self, amount: f32) { ... }
    pub fn heal(&mut self, amount: f32) { ... }
    pub fn is_alive(&self) -> bool { ... }
}
```

---

### 4. **Runtime** (`runtime.rs`)
Выполнения событий и управление сущностями:

```rust
pub struct EntityInstance {
    pub name: String,
    pub tag: String,
    pub health: i32,
    pub velocity: f64,
    pub position: f64,
}

pub enum Value {
    Float(f64),
    Int(i64),
    Str(String),
    EntitySnapshot(SimpleEntity),
}

pub fn execute_event(entity: &mut EntityInstance, event: &Event, params: &HashMap<String, Value>)
```

---

### 5. **Systems** (`systems.rs`)
Готовые системы для расширения функциональности:

#### EventSystem
Система публикации-подписки для событий:
```rust
pub struct EventSystem {
    listeners: HashMap<String, Vec<EventListener>>,
}

pub fn emit(&self, event: Event) { ... }
pub fn subscribe(&mut self, event_type: String, listener: EventListener) { ... }
```

#### StateMachine
Управление состояниями объектов:
```rust
pub enum EntityState {
    Idle,
    Moving,
    Attacking,
    TakingDamage,
    Dead,
    Custom(String),
}

pub fn on_event(&mut self, trigger: &str) -> bool { ... }
pub fn is_in_state(&self, state: &EntityState) -> bool { ... }
```

#### BehaviorTree
Система деревьев поведения для AI:
```rust
pub struct BehaviorTree {
    pub name: String,
    pub nodes: Vec<BehaviorNode>,
    pub root_id: usize,
}
```

---

### 6. **Scene Manager** (`scene.rs`)
Управление несколькими сценами:
```rust
pub struct SceneManager {
    pub scenes: HashMap<String, GameScene>,
    pub active_scene: Option<String>,
}

pub fn set_active_scene(&mut self, name: String) -> Result<(), String> { ... }
get_active_scene_mut() -> Option<&mut GameScene> { ... }
```

---

## Пример использования

### Создание и запуск игры
```rust
let mut engine = GameEngine::new();

// 1. Создать сцену из метаязыка
engine.create_scene_from_meta("GameScene", game_meta)?;

// 2. Загрузить сцену
engine.load_scene("GameScene")?;

// 3. Добавить ноды
let player = Node::new("player_1".to_string(), "Player".to_string(), "CharacterBody2D".to_string());
engine.add_node("GameScene", player)?;

// 4. Game loop
while engine.is_running() {
    engine.update(0.016); // 60 FPS
}
```

---

## Интеграция с метаязыком

1. **Парсинг**: Метаязык парсит определения сущностей
2. **Хранение**: Определения хранятся в `Scene::entities_meta`
3. **Выполнение**: Eventos выполняются через `execute_event()` в runtime
4. **Состояние**: Текущее состояние хранится в `Node::instance` (EntityInstance)

---

## Структура файлов

```
src/
├── main.rs                 # Точка входа, демонстрация
├── meta_lang.rs           # Парсер метаязыка
├── runtime.rs             # Выполнение событий
├── game_engine.rs         # Основной игровой движок
├── components.rs          # Встроенные компоненты
├── scene.rs               # Система управления сценами
├── systems.rs             # Event System, State Machine, Behavior Tree
├── analyzer.rs            # Анализ corpus (из предыдущего этапа)
├── ast.rs                 # AST структуры
├── supercompiler.rs       # Оптимизация (из предыдущего этапа)
└── aot_generator.rs       # AOT генератор (из предыдущего этапа)
```

---

## Демонстрация

При запуске `cargo run` выводится:

1. **Part 1**: Парсинг метаязыка - структура сущности
2. **Part 2**: Выполнение событий - обновление позиции и здоровья
3. **Part 3**: Game Engine MVP
   - Создание сцены из метаязыка
   - Создание игровых объектов
   - Симуляция 5 кадров с отображением времени

---

## Возможности расширения

### Phase 2: Расширенные функции
- [ ] Система физики (collision detection)
- [ ] Система рендеринга (2D/3D)
- [ ] Input handling (клавиатура, мышь, геймпад)
- [ ] Sound & Music system
- [ ] Particle system
- [ ] Animation system

### Phase 3: Editor & Tools
- [ ] Visual Scene Editor (как в Godot)
- [ ] Inspector для компонентов
- [ ] Asset Browser
- [ ] Debug Console

### Phase 4: Advanced Features
- [ ] Networking (multiplayer support)
- [ ] Save/Load system
- [ ] Scripting API
- [ ] Performance profiling

---

## Вывод

✅ Успешно построен **MVP игрового движка** который:
- Интегрирует метаязык для описания сущностей
- Использует архитектуру похожую на Godot
- Имеет систему компонентов и нод
- Поддерживает события и состояния
- Готов к дальнейшему расширению

**Основа для полноценного game engine готова!**
