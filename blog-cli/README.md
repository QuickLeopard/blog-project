# blog-cli

Консольный клиент для блог-платформы. Поддерживает протоколы HTTP и gRPC.

## Сборка

```bash
cargo build -p blog-cli
```

## Использование

Каждая команда требует флаг протокола (`--http` или `--grpc`):

```bash
blog-cli --http <команда>
blog-cli --grpc <команда>
```

Произвольный адрес сервера (необязательно):

```bash
blog-cli --http --server 192.168.1.10:3000 <команда>
blog-cli --grpc --server 192.168.1.10:50051 <команда>
```

Адреса по умолчанию: `127.0.0.1:3000` (HTTP), `127.0.0.1:50051` (gRPC).

## Команды

### Аутентификация

```bash
# Регистрация
blog-cli --http register --username alice --email alice@example.com --password secret123

# Вход (возвращает JWT-токен)
blog-cli --http login --username alice --password secret123
```

### Посты

```bash
# Список постов (с пагинацией)
blog-cli --http list-posts --offset 0 --limit 10

# Получить один пост
blog-cli --http get-post 1

# Создать пост (требуется токен)
blog-cli --http create-post --title "Привет" --content "Мир" --token "<jwt>"

# Обновить пост (только владелец)
blog-cli --http update-post --id 1 --title "Новый заголовок" --content "Новое содержимое" --token "<jwt>"

# Удалить пост (только владелец)
blog-cli --http delete-post --id 1 --token "<jwt>"
```

### Псевдонимы команд

| Полная команда | Псевдоним |
|---|---|
| `create-post` | `create` |
| `delete-post` | `delete` |
| `update-post` | `update` |
| `list-posts` | `list` |

## Пример рабочего процесса

```bash
# 1. Регистрация
blog-cli --http register --username bob --email bob@test.com --password pass123
# Вывод: Registered user: bob ... token: "eyJ..."

# 2. Вход
blog-cli --http login --username bob --password pass123
# Вывод: Logged in user: bob ... token: "eyJ..."

# 3. Создание поста (используйте токен из шага 2)
blog-cli --http create-post --title "Мой пост" --content "Содержимое" --token "eyJ..."

# 4. Список всех постов
blog-cli --http list-posts

# 5. Обновление поста
blog-cli --http update-post --id 1 --title "Обновлённый" --content "Новое содержимое" --token "eyJ..."

# 6. Удаление поста
blog-cli --http delete-post --id 1 --token "eyJ..."
```

## Зависимости

Использует библиотеку `blog-client` для абстракции транспорта и `clap` для разбора аргументов.
