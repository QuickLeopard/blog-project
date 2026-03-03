# blog-server

REST API на Actix-Web и gRPC-сервер на Tonic для блог-платформы.

## Обязанности

- Регистрация и вход пользователей с хэшированием паролей Argon2
- JWT-аутентификация через middleware
- CRUD-операции для постов блога (обновление/удаление только владельцем)
- Хранение данных в PostgreSQL через SQLx (автоматические миграции при запуске)
- gRPC-сервис (Tonic), дублирующий все HTTP-эндпоинты

## Архитектура

```
presentation/
├── http_public.rs      Публичные эндпоинты (вход, регистрация, получение постов)
├── http_protected.rs   Защищённые JWT-эндпоинты (создание, обновление, удаление постов)
├── middleware.rs        Экстрактор AuthenticatedUser (валидация JWT)
└── grpc_service.rs     Реализация gRPC BlogService

application/
├── auth_service.rs     Регистрация, вход, генерация токенов
└── blog_service.rs     CRUD-логика постов

data/
├── post_repository_trait.rs    Трейт PostRepository (async)
├── user_repository_trait.rs    Трейт UserRepository (async)
├── db_post_repository.rs       Реализация для PostgreSQL
├── db_user_repository.rs       Реализация для PostgreSQL
├── in_memory_post_repository.rs  In-memory реализация (для тестирования)
└── in_memory_user_repository.rs  In-memory реализация (для тестирования)

infrastructure/
├── database.rs         Пул соединений + запуск миграций
├── jwt.rs              JwtService (генерация/проверка токенов)
├── hash.rs             Argon2 hash_password / verify_password
├── app_state.rs        Общая структура AppState
└── logging.rs          Настройка трассировки (tracing)

domain/
├── post.rs             Структура Post, типы запросов/ответов
├── user.rs             Структура User, типы запросов/ответов аутентификации
└── error.rs            Перечисление DomainError с маппингом в HTTP-статусы
```

## Конфигурация

| Переменная окружения | Обязательна | По умолчанию | Описание |
|---|---|---|---|
| `DATABASE_URL` | Да | `postgres://localhost/blog` | Строка подключения к PostgreSQL |
| `SECRET_TOKEN` | Да | — | Секрет для подписи JWT |
| `RUST_LOG` | Нет | `info` | Уровень логирования |

## Запуск

```bash
DATABASE_URL=postgres://blog_user:blog_password@localhost:5432/blog \
SECRET_TOKEN=my-dev-secret \
cargo run -p blog-server
```

Запускает HTTP на `0.0.0.0:3000` и gRPC на `0.0.0.0:50051`. Миграции выполняются автоматически.

## База данных

Требуется PostgreSQL 16+. Миграции из `migrations/` создают две таблицы:

- **users** — `id`, `username` (уникальный), `email` (уникальный), `password_hash`, `created_at`
- **posts** — `id`, `title`, `content`, `author_id` (FK → users), `created_at`, `updated_at`

## Тестирование

```bash
cargo test -p blog-server
```

Есть unit-тесты для `hash_password`/`verify_password`. In-memory реализации репозиториев доступны для тестирования сервисов без базы данных.
