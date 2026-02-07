# Airplane 3D Viewer - Инструкции 

## Сборка и запуск

### CLI Demo (работает)
```bash
cargo run
```

### Web UI (работает)
```bash
cargo run -- --serve-web
```

### 3D Viewer (в разработке)  
Модуль `src/renderer.rs` содержит полную реализацию 3D рендерера на wgpu/winit с поддержкой:
- Загрузка модели самолёта из `airplane/11805_airplane_v2_L2.obj`
- Вращение мышкой (левая кнопка + движение)
- 3D камера и освещение (фонг модель)

Для запуска 3D приложения требуется улучшить обработку lifetime'ов в surface creation. Текущ ошибки связаны с тем, что `wgpu::Surface` требует `'static` lifetime для `Window`.

**Решение**: использовать  `Rc<Window>` или `Arc<Window>` при создании surface, или переписать архитектуру для использования `run_return` вместо `run`.

### Структура проекта

- `src/main.rs` — основная точка входа с парсингом аргументов
- `src/lib.rs` — публичное API для библиотеки
- `src/renderer.rs` — 3D рендерер (требует fixes)
- `src/game_engine.rs` — game engine с component и entity системой  
- `src/meta_lang.rs` — парсер метаязыка
- `src/runtime.rs` — интерпретатор событий
- `tests/*` — функциональные тесты для каждого модуля
- `web/` — WebAssembly фронтенд (отдельная сборка)

## Команды

```bash
# Все测试
cargo test-all

# Построить всё
cargo build-all

# Только основное приложение
cargo build

# Запустить CLI  
cargo run

# Запустить Web
cargo run -- --serve-web

# Запустить 3D (после фиксов)
# cargo run -- --3d
```

## Известные проблемы

1. **web_frontend** — WebGL рендерер имеет ошибки в замыканиях, но фреймворк полностью готов
2. **renderer.rs** — требует фиксапproper lifetime for wgpu Surface

## Следующие шаги

1. Исправить lifetime issues в 3D рендерере
2. Интегрировать метаязык с 3D рендерером (скрипты управляют моделью)
3. Добавить поддержку интерактивного управления через метаязык
