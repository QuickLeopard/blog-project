# Blog Project

Полнофункциональная блог-платформа на Rust: бэкенд на Actix-Web/gRPC, фронтенд на Leptos WASM и CLI-клиент.

## Архитектура

```
blog-project/
├── blog-server/      REST API на Actix-Web + gRPC-сервер на Tonic
├── blog-frontend/    SPA на Leptos 0.7 (компилируется в WASM, раздаётся через Trunk)
├── blog-client/      Общая клиентская библиотека (транспорты HTTP + gRPC)
└── blog-cli/         Консольный клиент на основе blog-client
```

**Стек технологий:** Rust 2024 edition, Actix-Web 4, Tonic (gRPC), Leptos 0.7 (CSR), SQLx + PostgreSQL, хэширование паролей Argon2, JWT-аутентификация, UI на Bootstrap 5.

## Требования

- **Rust** (stable, 1.85+) с таргетом `wasm32-unknown-unknown`
- **Docker** и **Docker Compose**
- **Trunk** (`cargo install trunk`) — для локальной разработки фронтенда
- **Компилятор Protobuf** (`protoc`) — для генерации кода gRPC

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Настройка окружения

Скопируйте пример файла переменных окружения и отредактируйте по необходимости:

```bash
cp .env.example .env
```

Обязательные переменные:

| Переменная | Описание | По умолчанию |
|------------|----------|--------------|
| `POSTGRES_USER` | Пользователь базы данных | `blog_user` |
| `POSTGRES_PASSWORD` | Пароль базы данных | `blog_password` |
| `POSTGRES_DB` | Имя базы данных | `blog` |
| `DATABASE_URL` | Полная строка подключения к Postgres | собирается из вышеуказанных |
| `SECRET_TOKEN` | Секрет для подписи JWT (обязательно) | нет |
| `RUST_LOG` | Уровень логирования | `info` |

---

## Запуск проекта

### Вариант 1: Всё в Docker

Самый простой способ — локальный Rust не нужен.

```bash
docker compose up --build
```

Запускает:
- **PostgreSQL** на порту `5432`
- **blog-server** на порту `3000` (HTTP) и `50051` (gRPC)
- **blog-frontend** на порту `8080` (Nginx раздаёт WASM и проксирует `/api/` на blog-server)

Откройте http://localhost:8080 в браузере.

### Вариант 2: Бэкенд в Docker, фронтенд локально (рекомендуется для разработки)

Оптимально для разработки фронтенда — Trunk обеспечивает горячую перезагрузку при изменениях.

**1. Запустите Postgres и бэкенд:**

```bash
docker compose up postgres blog-server
```

**2. Запустите фронтенд через Trunk:**

```bash
cd blog-frontend
trunk serve
```

Trunk запускается на http://localhost:8080 и проксирует запросы `/api/` на `localhost:3000` (настраивается в `Trunk.toml`).

### Вариант 3: Всё локально (без Docker)

**1. Запустите PostgreSQL локально** (например, через пакетный менеджер или отдельный контейнер):

```bash
# отдельный контейнер Postgres
docker run -d --name blog-pg -p 5432:5432 \
  -e POSTGRES_USER=blog_user \
  -e POSTGRES_PASSWORD=blog_password \
  -e POSTGRES_DB=blog \
  postgres:16-alpine
```

**2. Запустите бэкенд:**

```bash
DATABASE_URL=postgres://blog_user:blog_password@localhost:5432/blog \
SECRET_TOKEN=my-dev-secret \
cargo run -p blog-server
```

Сервер автоматически выполняет миграции при запуске. HTTP API — на `localhost:3000`, gRPC — на `localhost:50051`.

**3. Запустите фронтенд:**

```bash
cd blog-frontend
trunk serve
```

Откройте http://localhost:8080.

---

## Использование CLI (`blog-cli`)

CLI взаимодействует с бэкендом по HTTP или gRPC. Требует запущенный blog-server.

### Сборка

```bash
cargo build -p blog-cli
```

### Выбор протокола

Каждая команда требует флага `--http` или `--grpc`:

```bash
# HTTP (сервер по умолчанию: 127.0.0.1:3000)
cargo run -p blog-cli -- --http <команда>

# gRPC (сервер по умолчанию: 127.0.0.1:50051)
cargo run -p blog-cli -- --grpc <команда>

# Произвольный адрес сервера
cargo run -p blog-cli -- --http --server 192.168.1.10:3000 <команда>
```

### Команды

**Регистрация нового пользователя:**

```bash
cargo run -p blog-cli -- --http register \
  --username alice \
  --email alice@example.com \
  --password secret123
```

**Вход (возвращает JWT-токен):**

```bash
cargo run -p blog-cli -- --http login \
  --username alice \
  --password secret123
```

Сохраните токен из вывода для аутентифицированных команд.

**Создать пост:**

```bash
cargo run -p blog-cli -- --http create-post \
  --title "Мой первый пост" \
  --content "Привет, мир!" \
  --token "<ваш-jwt-токен>"
```

**Список постов:**

```bash
cargo run -p blog-cli -- --http list-posts --offset 0 --limit 10
```

**Получить один пост:**

```bash
cargo run -p blog-cli -- --http get-post 1
```

**Обновить пост:**

```bash
cargo run -p blog-cli -- --http update-post \
  --id 1 \
  --title "Обновлённый заголовок" \
  --content "Обновлённое содержимое" \
  --token "<ваш-jwt-токен>"
```

**Удалить пост:**

```bash
cargo run -p blog-cli -- --http delete-post \
  --id 1 \
  --token "<ваш-jwt-токен>"
```

---

## Эндпоинты API

| Метод | Путь | Аутентификация | Описание |
|-------|------|----------------|----------|
| GET | `/api/health` | Нет | Проверка работоспособности |
| POST | `/api/auth/register` | Нет | Регистрация пользователя |
| POST | `/api/auth/login` | Нет | Вход, возвращает JWT |
| GET | `/api/posts` | Нет | Список постов (с пагинацией) |
| GET | `/api/posts/:id` | Нет | Получить один пост |
| POST | `/api/posts` | JWT | Создать пост |
| PUT | `/api/posts/:id` | JWT | Обновить пост (только владелец) |
| DELETE | `/api/posts/:id` | JWT | Удалить пост (только владелец) |

---

## Тестирование

```bash
# Запустить все тесты
cargo test --workspace

# Только тесты сервера
cargo test -p blog-server

# Конкретный модуль тестов
cargo test -p blog-server -- hash::tests
```

---

## Тестирование через curl

Все примеры используют синтаксис PowerShell (Windows). Запускайте из PowerShell или Windows Terminal.
Замените `<TOKEN>` на значение JWT, полученное при входе или регистрации.

### Проверка работоспособности бэкенда

```powershell
curl http://localhost:3000/api/health
```

### Аутентификация

**Регистрация нового пользователя:**

```powershell
curl -X POST http://localhost:3000/api/auth/register `
  -H "Content-Type: application/json" `
  -d "{\"username\":\"alice\",\"email\":\"alice@example.com\",\"password\":\"secret123\"}"
```

**Вход (возвращает JWT-токен):**

```powershell
curl -X POST http://localhost:3000/api/auth/login `
  -H "Content-Type: application/json" `
  -d "{\"username\":\"alice\",\"password\":\"secret123\"}"
```

Скопируйте значение поля `token` из ответа и сохраните для последующих запросов:

```powershell
$TOKEN = "eyJ..."
```

### Посты

**Список постов (с пагинацией):**

```powershell
curl "http://localhost:3000/api/posts?offset=0&limit=10"
```

**Получить один пост:**

```powershell
curl http://localhost:3000/api/posts/1
```

**Создать пост (требует аутентификации):**

```powershell
curl -X POST http://localhost:3000/api/posts `
  -H "Content-Type: application/json" `
  -H "Authorization: Bearer $TOKEN" `
  -d "{\"title\":\"Мой первый пост\",\"content\":\"Привет из curl!\"}"
```

**Обновить пост (только владелец):**

```powershell
curl -X PUT http://localhost:3000/api/posts/1 `
  -H "Content-Type: application/json" `
  -H "Authorization: Bearer $TOKEN" `
  -d "{\"title\":\"Обновлённый заголовок\",\"content\":\"Обновлённое содержимое.\"}"
```

**Удалить пост (только владелец):**

```powershell
curl -X DELETE http://localhost:3000/api/posts/1 `
  -H "Authorization: Bearer $TOKEN"
```

### Фронтенд (через Nginx-прокси на порту 8080)

При запуске через Docker Compose Nginx проксирует `/api/` на бэкенд.
Используйте порт `8080` вместо `3000`:

```powershell
# Проверка работоспособности через Nginx-прокси
curl http://localhost:8080/api/health

# Вход через прокси
curl -X POST http://localhost:8080/api/auth/login `
  -H "Content-Type: application/json" `
  -d "{\"username\":\"alice\",\"password\":\"secret123\"}"

# Список постов через прокси
curl "http://localhost:8080/api/posts?offset=0&limit=10"
```

> **Примечание:** В PowerShell `curl` является псевдонимом для `Invoke-WebRequest`. Если в ответе приходит XML вместо JSON, используйте `curl.exe` явно (настоящий бинарник curl):
> ```powershell
> curl.exe -X POST http://localhost:3000/api/auth/login ...
> ```
> Добавьте флаг `-v` к любой команде для просмотра заголовков запроса/ответа при отладке аутентификации или проблем с CORS.

---

## Структура проекта

Подробности в отдельных README-файлах:

- [blog-server/README.md](blog-server/README.md) — Бэкенд API-сервер
- [blog-frontend/README.md](blog-frontend/README.md) — Фронтенд на Leptos WASM
- [blog-client/README.md](blog-client/README.md) — Общая клиентская библиотека
- [blog-cli/README.md](blog-cli/README.md) — Консольный клиент
