# Занятие 24. Смарт-контракт на Rust и деплой в Solana, часть 1

Программа на Solana (без Anchor, на `solana-program` + borsh): счёт-фактура (invoice), которая хранится в PDA.

## Что демонстрирует

- entrypoint и диспетчеризацию инструкций (`CreateInvoice`, `PayInvoice`)
- PDA (Program Derived Address): создание аккаунта через `invoke_signed` с seeds
- CPI (cross-program invocation): перевод lamports через `system_instruction::transfer`
- проверки подписанта и доступа (только admin создаёт инвойсы)
- сериализацию данных аккаунта через borsh

## Сборка

```bash
cargo build-sbf
```

Для деплоя понадобится solana CLI и локальный валидатор (`solana-test-validator`). Клиент для вызова программы — в [`../25-invoice-client/`](../25-invoice-client/).
