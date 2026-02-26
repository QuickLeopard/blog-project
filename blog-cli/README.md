# Blog CLI — Утилита для работы с блогом

Подробная инструкция по сборке и использованию CLI-клиента проекта блога (blog-cli).

## Обзор
CLI позволяет взаимодействовать с сервером блога через HTTP или gRPC: регистрация/логин пользователя, создание/обновление/удаление постов, получение списка или отдельного поста и подсчёт постов.

## Требования
- Rust (rustc, cargo) — рекомендована последняя стабильная версия.
- Сервер блога (HTTP или gRPC) должен быть запущен и доступен по адресу/порту.

## Дефолтные адреса
Если не указать `--server`, используются:
- gRPC: `127.0.0.1:50051`
- HTTP: `127.0.0.1:3000`

## Сборка
Из корня рабочего пространства (workspace):

- Быстрая сборка и запуск:
```bash
cargo run -p blog-cli -- --help
```

- Сборка релизной версии (байнари в target/release):
```bash
cargo build -p blog-cli --release
```

Или из папки `blog-cli`:
```bash
cd blog-cli
cargo run -- --help
```

## Запуск — общая структура
Флаги протокола (обязательная группа — нужно выбрать один из них):
- `--grpc` — использовать gRPC
- `--http` — использовать HTTP

Опция `--server <ADDR>` — указать адрес сервера (перекрывает дефолт).

Синтаксис запуска через cargo:
```bash
cargo run -p blog-cli -- --http <subcommand> [options]
# или
cd blog-cli && cargo run -- --grpc <subcommand> [options]
```

Clap генерирует имена подкоманд в стиле kebab-case от enum-variants. Для `CreatePost` есть alias `create` (короткое имя).

## Подкоманды и примеры

1) Регистрация пользователя
```bash
cargo run -p blog-cli -- --http register --username alice --email alice@example.com --password secret
```

2) Логин
```bash
cargo run -p blog-cli -- --http login --username alice --password secret
```

3) Создать пост (используя alias `create` или `create-post`)
```bash
# с HTTP
cargo run -p blog-cli -- --http create --title "Мой заголовок" --content "Текст поста" --token "JWT_TOKEN"

# или полное имя команды
cargo run -p blog-cli -- --http create-post --title "Мой заголовок" --content "Текст поста" --token "JWT_TOKEN"
```
Обратите внимание: в текущей реализации поля используются как длинные флаги `--title`, `--content`, `--token`.

4) Удалить пост
```bash
cargo run -p blog-cli -- --http delete-post --id 42 --token "JWT_TOKEN"
# alias: delete
cargo run -p blog-cli -- --http delete --id 42 --token "JWT_TOKEN"
```

5) Обновить пост
```bash
cargo run -p blog-cli -- --http update-post --id 42 --title "Новый заголовок" --content "Новый текст" --token "JWT_TOKEN"
# alias: update
```

6) Список постов
```bash
cargo run -p blog-cli -- --http list-posts --offset 0 --limit 10
# alias: list
```

7) Получить пост по id
```bash
cargo run -p blog-cli -- --http get-post --id 42
# alias: get
```

8) Подсчитать посты
```bash
cargo run -p blog-cli -- --http count-posts
```

## gRPC примеры
Поменяйте флаг протокола:
```bash
cargo run -p blog-cli -- --grpc create --title "T" --content "C" --token "JWT_TOKEN"
```
или
```bash
cargo run -p blog-cli -- --grpc list-posts --offset 0 --limit 20
```

Если сервер gRPC слушает не по дефолту:
```bash
cargo run -p blog-cli -- --grpc --server 10.0.0.5:50051 list-posts
```

## Токен аутентификации
Команды, требующие авторизации (create, update, delete), ожидают JWT токен в параметре `--token`. Токен должен быть действительным и выдан сервером при логине/регистрации.

## Отладка и ошибки
- Если сервер возвращает HTTP ошибку (4xx/5xx) — CLI вернёт ошибку и её текст.
- Для gRPC ошибки отображаются как tonic/transport ошибки.
- Проверьте адрес сервера и протокол, используйте `--server` для переопределения.

## Частые проблемы
- "connection refused" — сервер не запущен или указан неверный хост/порт.
- Неправильный токен — сервер вернёт 401/permission denied.
- Неправильные флаги — используйте `--help` для просмотра доступных опций:
```bash
cargo run -p blog-cli -- --help
cargo run -p blog-cli -- --http --help
cargo run -p blog-cli -- --http create --help
```

## Советы
- Для production — собирайте в релизном режиме: `cargo build -p blog-cli --release`.
- Если хотите короткие флаги (`-t`, `-c`, `-k`) — добавьте их в атрибуты `#[arg(short, long)]` в `main.rs` (в кодовой базе пока используются только длинные флаги `--...`).
