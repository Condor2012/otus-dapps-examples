# Занятие 26. Смарт-контракт на Rust и деплой в NEAR, часть 1

Контракт «донаты» на NEAR SDK для Rust: пользователи донатят NEAR, бенефициар получает переводы.

## Что демонстрирует

- NEAR SDK: `#[near(contract_state)]`, `#[init]`, `#[private]`, `#[payable]`
- persistent storage: `UnorderedMap` из `near_sdk::store`
- учёт storage cost при первом донате (регистрация донора)
- перевод токенов через `Promise`
- unit-тесты с `VMContextBuilder` и интеграционный тест на `near-workspaces` (sandbox)
- пагинацию и view-методы

## Сборка и тесты

```bash
# unit-тесты
cargo test

# интеграционный тест в sandbox (требует near-workspaces)
cargo test --test workspaces
```

Сборка в WASM для деплоя:

```bash
cargo build --target wasm32-unknown-unknown --release
```
