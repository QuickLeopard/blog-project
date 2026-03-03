# blog-cli

Command-line client for the blog platform. Supports both HTTP and gRPC protocols.

## Build

```bash
cargo build -p blog-cli
```

## Usage

Every command requires a protocol flag (`--http` or `--grpc`):

```bash
blog-cli --http <command>
blog-cli --grpc <command>
```

Custom server address (optional):

```bash
blog-cli --http --server 192.168.1.10:3000 <command>
blog-cli --grpc --server 192.168.1.10:50051 <command>
```

Default addresses: `127.0.0.1:3000` (HTTP), `127.0.0.1:50051` (gRPC).

## Commands

### Authentication

```bash
# Register
blog-cli --http register --username alice --email alice@example.com --password secret123

# Login (returns JWT token)
blog-cli --http login --username alice --password secret123
```

### Posts

```bash
# List posts (paginated)
blog-cli --http list-posts --offset 0 --limit 10

# Get a single post
blog-cli --http get-post 1

# Create a post (requires token)
blog-cli --http create-post --title "Hello" --content "World" --token "<jwt>"

# Update a post (owner only)
blog-cli --http update-post --id 1 --title "New Title" --content "New body" --token "<jwt>"

# Delete a post (owner only)
blog-cli --http delete-post --id 1 --token "<jwt>"
```

### Command Aliases

| Full command | Alias |
|---|---|
| `create-post` | `create` |
| `delete-post` | `delete` |
| `update-post` | `update` |
| `list-posts` | `list` |

## Example Workflow

```bash
# 1. Register
blog-cli --http register --username bob --email bob@test.com --password pass123
# Output: Registered user: bob ... token: "eyJ..."

# 2. Login
blog-cli --http login --username bob --password pass123
# Output: Logged in user: bob ... token: "eyJ..."

# 3. Create a post (use token from step 2)
blog-cli --http create-post --title "My Post" --content "Content here" --token "eyJ..."

# 4. List all posts
blog-cli --http list-posts

# 5. Update the post
blog-cli --http update-post --id 1 --title "Updated" --content "New content" --token "eyJ..."

# 6. Delete the post
blog-cli --http delete-post --id 1 --token "eyJ..."
```

## Dependencies

Uses `blog-client` library for transport abstraction and `clap` for argument parsing.
