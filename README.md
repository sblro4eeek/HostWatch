# 🖥️ HostWatch

---

> ⚡ Лёгкий CLI и HTTP-сервис для получения «снимка» состояния хоста: RAM/Swap, диски, датчики (температуры) и системные детали. Подходит как утилита «по запросу» и как маленький агент c кэшированием метрик.

---

## 📑 Содержание
- [✨ Возможности](#возможности)
- [📋 Требования](#требования)
- [⚙️ Установка](#установка)
- [🚀 Быстрый старт](#быстрый-старт)
- [🌐 HTTP API](#http-api)
- [🪵 Логирование](#логирование)
- [📦 Зависимости](#-зависимости)
- [📜 Лицензия](#лицензия)

---

## ✨ Возможности

- 🌐 **HTTP API**: эндпоинт `/get_info` с кэшированным снапшотом (фоновое обновление).
- 🧠 **Кэш метрик**:: фоновая задача обновляет метрики раз в N секунд (по умолчанию 2s).
- 🧱 **Наблюдаемость**: логирование запросов (tower-http Trace), компрессия, таймауты, graceful shutdown (Ctrl+C, SIGTERM).
- 🧩 **Чистые типы домена** — единые структуры данных для CLI/HTTP.

---

## 📋 Требования
- 🦀 Rust **1.75+**

---

## ⚙️ Установка
```
git clone https://github.com/sblro4eeek/HostWatch.git
cd hostwatch

# сборка релизной версии
cargo build --release
# бинарь: target/release/hostwatch

```
### 📦 Как бинарь
После `cargo build --release` бинарь лежит в `target/release/hostwatch`.

---

## 🚀 Быстрый старт
### 🌐 HTTP режим
```
# запустить сервер на 8080
cargo run -- serve --port 8080

# получить метрики
curl -s http://127.0.0.1:8080/get_info | jq .
```

### 📊 Пример ответа:
```
{
  "system": {
    "name": "Linux",
    "kernel_version": "6.8.0-xyz",
    "os_version": "Ubuntu 24.04",
    "host_name": "my-host"
  },
  "memory": {
    "total_ram_gb": 31.25,
    "total_ram_mb": 32000.0,
    "used_ram_gb": 12.42,
    "used_ram_mb": 12715.5,
    "ram_percent": 39.8,
    "total_swap_gb": 2.0,
    "total_swap_mb": 2048.0,
    "used_swap_gb": 0.0,
    "used_swap_mb": 0.0,
    "swap_percent": 0.0
  },
  "disks": [
    {
      "name": "nvme0n1p2",
      "mount_point": "/",
      "available_space_gb": 123.45,
      "available_space_mb": 126412.8,
      "total_space_gb": 476.94,
      "total_space_mb": 488386.6
    }
  ],
  "components": [
    { "label": "CPU (Tctl)", "temperature": 54.0 }
  ]
}
```
> Значения примерные; набор полей может отличаться между платформами.

---

## 🌐 HTTP API
### `GET /get_info`
Возвращает текущий кэшированный `HostStats`. Кэш обновляется в фоне каждые 2 секунды.

📡 Коды ответов:
- ✅ `200 OK` — успешный снимок состояния

📦 Заголовки:

- `Content-Encoding: gzip/br/zstd `— если клиент поддерживает (включена компрессия)
- `Content-Type: application/json; charset=utf-8`
---

## 🪵 Логирование
Используется `tracing` + `tower_http::trace`
Примеры:
```
RUST_LOG=info cargo run --port 8080
RUST_LOG=tower_http=info,hostwatch=debug cargo run
```
> 🛠️ Средства логирования уже подключены в старте приложения; формат — компактный, пригодный для grep/журналов.

---

## 📦 Зависимости

| 📚 Библиотека             | 🔖 Версия  | 🛠️ Назначение                                    |
|-------------------------|---------|-----------------------------------------------|
| [axum](https://crates.io/crates/axum)               | `0.8.4`   | HTTP-стек (роутинг, сервер)                   |
| [tokio](https://crates.io/crates/tokio)             | `1.47.1`     | Async runtime                                 |
| [tokio-util](https://crates.io/crates/tokio-util)   | `0.7.16`   | Доп. утилиты для Tokio (CancellationToken и др.) |
| [tower](https://crates.io/crates/tower)             | `0.5.2`   | Сервисный слой (основа для middleware)        |
| [tower-http](https://crates.io/crates/tower-http)   | `0.6.6`   | Middleware (CORS, trace, compression, timeout)|
| [sysinfo](https://crates.io/crates/sysinfo)         | `0.37`  | Сбор системных метрик (CPU, RAM, диски, сенсоры) |
| [clap](https://crates.io/crates/clap)               | `4.5.48`     | CLI-парсер аргументов                         |
| [serde](https://crates.io/crates/serde)             | `1.0.226`     | Сериализация/десериализация                   |
| [serde_json](https://crates.io/crates/serde_json)   | `1.0.225`     | Работа с JSON                                 |
| [tracing](https://crates.io/crates/tracing)         | `0.1.41`   | Логирование и трассировка                     |
| [tracing-subscriber](https://crates.io/crates/tracing-subscriber) | `0.3.20` | Подписчики и форматирование логов             |
| [anyhow](https://crates.io/crates/anyhow)           | `1.0.100`     | Удобная работа с ошибками                     |

---

## 📜 License

[MIT](https://opensource.org/licenses/MIT)