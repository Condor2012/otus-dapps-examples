# otus-dapps-examples

Примеры смарт-контрактов к курсу Otus **«DApps-Development»**.

Автор занятий 24–27: написание и деплой смарт-контрактов на Rust в блокчейнах Solana и NEAR.

## Структура

| Занятие | Тема | Код |
|---|---|---|
| 24 | Смарт-контракт на Rust и деплой в Solana, часть 1 (Anchor, PDA) | [smart-program-sol](https://github.com/Condor2012/smart-program-sol) |
| 25 | Смарт-контракт на Rust и деплой в Solana, часть 2 (CPI, SPL Token) | [smart-program-client-sol](https://github.com/Condor2012/smart-program-client-sol) |
| 26 | Смарт-контракт на Rust и деплой в NEAR, часть 1 (NEAR SDK, persistent storage) | — |
| 27 | Смарт-контракт на Rust и деплой в NEAR, часть 2 (storage staking, callbacks) | — |

## Запуск

### Solana (Anchor)

```bash
cd solana/<пример>
anchor build
anchor test
```

### NEAR

```bash

cd near/<пример>
cargo build --target wasm32-unknown-unknown --release
near deploy ...
```

⚠️ Все примеры рассчитаны на **devnet/testnet**. Никогда не храните приватные ключи в репозитории.
