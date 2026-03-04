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

## Управление токеном

При успешном выполнении команд `register` или `login` JWT-токен **автоматически сохраняется** в файл `.blog_token` в текущей директории.

Все команды, требующие аутентификации (`create-post`, `update-post`, `delete-post`), **автоматически загружают токен** из `.blog_token`, если параметр `--token` не указан явно.

Приоритет:
1. Явный `--token <значение>` — используется в первую очередь
2. Файл `.blog_token` — загружается автоматически, если `--token` не передан
3. Если токен не найден ни там, ни там — выводится сообщение об ошибке

## Команды

### Аутентификация

```bash
# Регистрация (токен сохраняется в .blog_token)
blog-cli --http register --username alice --email alice@example.com --password secret123

# Вход (токен сохраняется в .blog_token)
blog-cli --http login --username alice --password secret123
```

### Посты

```bash
# Список постов (с пагинацией)
blog-cli --http list-posts --offset 0 --limit 10

# Получить один пост
blog-cli --http get-post 1

# Создать пост (токен читается из .blog_token)
blog-cli --http create-post --title "Привет" --content "Мир"

# Создать пост с явным токеном
blog-cli --http create-post --title "Привет" --content "Мир" --token "<jwt>"

# Обновить пост (только владелец)
blog-cli --http update-post --id 1 --title "Новый заголовок" --content "Новое содержимое"

# Удалить пост (только владелец)
blog-cli --http delete-post --id 1
```

### Псевдонимы команд

| Полная команда | Псевдоним |
|---|---|
| `create-post` | `create` |
| `delete-post` | `delete` |
| `update-post` | `update` |
| `list-posts` | `list` |
| `get-post` | `get` |

## Пример рабочего процесса

```bash
# 1. Регистрация — токен сохраняется автоматически
blog-cli --http register --username bob --email bob@test.com --password pass123
# ✓ Registered as bob
#   Token saved to .blog_token

# 2. Создание поста — токен подхватывается из .blog_token
blog-cli --http create-post --title "Мой пост" --content "Содержимое"
# ✓ Created post #1: "Мой пост"

# 3. Список всех постов
blog-cli --http list-posts
# Posts (offset=0, limit=10):
#   #1    Мой пост (by user #1)
# (1 posts shown)

# 4. Просмотр одного поста
blog-cli --http get-post 1

# 5. Обновление поста
blog-cli --http update-post --id 1 --title "Обновлённый" --content "Новое содержимое"
# ✓ Updated post #1: "Обновлённый"

# 6. Удаление поста
blog-cli --http delete-post --id 1
# ✓ Deleted post #1

# 7. То же самое через gRPC
blog-cli --grpc login --username bob --password pass123
blog-cli --grpc create-post --title "gRPC пост" --content "Через gRPC"
```

## Обработка ошибок

CLI выводит понятные сообщения об ошибках:

| Ситуация | Сообщение |
|---|---|
| Нет токена | `Error: Not authenticated. Run register or login first, or pass --token.` |
| Неверные данные для входа | `Error: Unauthorized: Invalid credentials` |
| Пост не найден | `Error: Resource not found.` |
| Дублирование пользователя | `Error: Conflict — User already exists` |

## Зависимости

Использует библиотеку `blog-client` для абстракции транспорта и `clap` для разбора аргументов.
