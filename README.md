# otus-dapps-examples

Примеры смарт-контрактов к курсу Otus **«DApps-Development»**.

Автор занятий 24–27: написание и деплой смарт-контрактов на Rust в блокчейнах Solana и NEAR.

## Структура

| Занятие | Тема | Код |
|---|---|---|
| 24 | Смарт-контракт на Rust и деплой в Solana, часть 1 (PDA, CPI) | [`solana/24-invoice-program/`](solana/24-invoice-program/) |
| 25 | Смарт-контракт на Rust и деплой в Solana, часть 2 (клиент, транзакции) | [`solana/25-invoice-client/`](solana/25-invoice-client/) |
| 26 | Смарт-контракт на Rust и деплой в NEAR, часть 1 (NEAR SDK, persistent storage) | [`near/26-donation-contract/`](near/26-donation-contract/) |
| 27 | Смарт-контракт на Rust и деплой в NEAR, часть 2 (storage staking, callbacks) | — |

## Запуск

### Solana

```bash
cd solana/24-invoice-program
cargo build-sbf

cd ../25-invoice-client
export ADMIN_KEYPAIR="12,34,56,..."
cargo run   # требует запущенный solana-test-validator
```

### NEAR

```bash
cd near/26-donation-contract
cargo test                                # unit + интеграционные тесты в sandbox
cargo build --target wasm32-unknown-unknown --release
```

⚠️ Все примеры рассчитаны на **devnet/testnet/localnet**. Никогда не храните приватные ключи в репозитории.
