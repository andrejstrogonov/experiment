/// Встроенные компоненты движка
pub trait IComponent: Send + Sync {
    fn name(&self) -> &str;
    fn update(&mut self, delta_time: f64);
}

/// Transform компонент - определяет позицию, ротацию, масштаб
#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
}

/// Velocity компонент - для движения
#[derive(Debug, Clone)]
pub struct Velocity {
    pub value: Vector3,
}

/// Health компонент - для здоровья/урона
#[derive(Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

/// Sprite компонент - для отрисовки
#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture_path: String,
    pub visible: bool,
}

/// Physics компонент - для физики
#[derive(Debug, Clone)]
pub struct Physics {
    pub mass: f32,
    pub gravity: f32,
    pub velocity: Vector3,
}

/// Input компонент - для обработки ввода
#[derive(Debug, Clone)]
pub struct Input {
    pub keys_pressed: Vec<String>,
}

/// Вектор 3D для позиции и направления
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            value: Vector3::zero(),
        }
    }
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            texture_path: String::new(),
            visible: true,
        }
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            mass: 1.0,
            gravity: 9.8,
            velocity: Vector3::zero(),
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keys_pressed: Vec::new(),
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
}

impl Physics {
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
            gravity: 9.8,
            velocity: Vector3::zero(),
        }
    }
}

impl Input {
    pub fn is_key_pressed(&self, key: &str) -> bool {
        self.keys_pressed.contains(&key.to_string())
    }

    pub fn press_key(&mut self, key: String) {
        if !self.keys_pressed.contains(&key) {
            self.keys_pressed.push(key);
        }
    }

    pub fn release_key(&mut self, key: &str) {
        self.keys_pressed.retain(|k| k != key);
    }
}
