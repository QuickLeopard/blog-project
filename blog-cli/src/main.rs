use std::process;

use clap::{ArgGroup, Parser};

use blog_client::blog_client::BlogClient;
use blog_client::error::BlogClientError;

/// File in the current working directory where the JWT is stored after login.
/// The token is stored as plaintext — do not commit this file to version control.
const TOKEN_FILE: &str = ".blog_token";

/// Default server address when `--http` is used and `--server` is not specified.
const DEFAULT_HTTP_ADDR: &str = "127.0.0.1:3000";

/// Default server address when `--grpc` is used and `--server` is not specified.
const DEFAULT_GRPC_ADDR: &str = "127.0.0.1:50051";

/// Default number of posts to show per page in `list-posts`.
const DEFAULT_LIST_LIMIT: i32 = 10;

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Register {
        #[arg(long)]
        username: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
    },
    Login {
        #[arg(long)]
        username: String,
        #[arg(long)]
        password: String,
    },
    #[command(alias = "create")]
    CreatePost {
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: String,
        #[arg(long)]
        token: Option<String>,
    },
    #[command(alias = "delete")]
    DeletePost {
        #[arg(long)]
        id: i64,
        #[arg(long)]
        token: Option<String>,
    },
    #[command(alias = "update")]
    UpdatePost {
        #[arg(long)]
        id: i64,
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: String,
        #[arg(long)]
        token: Option<String>,
    },
    #[command(alias = "list")]
    ListPosts {
        #[arg(short, long, default_value_t = 0)]
        offset: i32,
        #[arg(short, long, default_value_t = DEFAULT_LIST_LIMIT)]
        limit: i32,
    },
    #[command(alias = "get")]
    GetPost { id: i64 },
}

#[derive(Parser, Debug)]
#[command(name = "cli", about = "Утилита для работы с блогом", version = "0.1")]
#[command(group(
    ArgGroup::new("protocol")
        .required(true)
        .args(["grpc", "http"])
))]
struct Cli {
    /// Server host address
    #[arg(long, global = true)]
    server: Option<String>,

    /// Use gRPC protocol
    #[arg(long)]
    grpc: bool,

    /// Use HTTP protocol
    #[arg(long)]
    http: bool,

    #[command(subcommand)]
    command: Commands,
}

fn save_token(token: &str) {
    if let Err(e) = std::fs::write(TOKEN_FILE, token) {
        eprintln!("Warning: could not save token to {TOKEN_FILE}: {e}");
    }
}

fn load_token() -> Option<String> {
    std::fs::read_to_string(TOKEN_FILE)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn resolve_token(explicit: Option<String>) -> Result<String, BlogClientError> {
    explicit
        .or_else(load_token)
        .ok_or_else(|| {
            BlogClientError::Unauthorized(
                "Not authenticated. Run `register` or `login` first, or pass --token.".into(),
            )
        })
}

fn print_error(err: &BlogClientError) {
    match err {
        BlogClientError::Unauthorized(msg) => eprintln!("Error: {msg}"),
        BlogClientError::NotFound => eprintln!("Error: Resource not found."),
        BlogClientError::Conflict(msg) => eprintln!("Error: Conflict — {msg}"),
        BlogClientError::InvalidRequest(msg) => eprintln!("Error: Bad request — {msg}"),
        other => eprintln!("Error: {other}"),
    }
}

async fn run() -> Result<(), BlogClientError> {
    let cli = Cli::parse();

    let host = cli.server.unwrap_or_else(|| {
        if cli.grpc {
            DEFAULT_GRPC_ADDR.to_string()
        } else {
            DEFAULT_HTTP_ADDR.to_string()
        }
    });

    println!(
        "Using {} at {}",
        if cli.grpc { "gRPC" } else { "HTTP" },
        host
    );

    let transport: Box<dyn blog_client::traits::BlogService> = if cli.grpc {
        Box::new(
            blog_client::grpc_client::BlogGrpcClient::connect(format!("http://{}", host)).await?,
        )
    } else {
        Box::new(blog_client::http_client::HttpClient::new(format!(
            "http://{}",
            host
        )))
    };

    let client = BlogClient::new(transport);

    match cli.command {
        Commands::Register {
            username,
            email,
            password,
        } => {
            let response = client.register_user(username, email, password).await?;
            save_token(&response.token);
            println!("✓ Registered as {}", response.user.username);
            println!("  Token saved to {TOKEN_FILE}");
        }
        Commands::Login { username, password } => {
            let response = client.login_user(username, password).await?;
            save_token(&response.token);
            println!("✓ Logged in as {}", response.user.username);
            println!("  Token saved to {TOKEN_FILE}");
        }
        Commands::CreatePost {
            title,
            content,
            token,
        } => {
            let token = resolve_token(token)?;
            let post = client.create_post(title, content, token).await?;
            println!("✓ Created post #{}: \"{}\"", post.id, post.title);
        }
        Commands::DeletePost { id, token } => {
            let token = resolve_token(token)?;
            client.delete_post(id, token).await?;
            println!("✓ Deleted post #{id}");
        }
        Commands::UpdatePost {
            id,
            title,
            content,
            token,
        } => {
            let token = resolve_token(token)?;
            let post = client.update_post(id, title, content, token).await?;
            println!("✓ Updated post #{}: \"{}\"", post.id, post.title);
        }
        Commands::ListPosts { offset, limit } => {
            let posts = client.get_posts(offset, limit).await?;
            if posts.is_empty() {
                println!("No posts found.");
            } else {
                println!("Posts (offset={offset}, limit={limit}):");
                for post in &posts {
                    println!(
                        "  #{:<4} {} (by user #{})",
                        post.id, post.title, post.author_id
                    );
                }
                println!("({} posts shown)", posts.len());
            }
        }
        Commands::GetPost { id } => {
            let post = client.get_post(id).await?;
            println!("Post #{}", post.id);
            println!("  Title:   {}", post.title);
            println!("  Author:  user #{}", post.author_id);
            println!("  Created: {}", post.created_at.format("%Y-%m-%d %H:%M:%S"));
            println!("  Updated: {}", post.updated_at.format("%Y-%m-%d %H:%M:%S"));
            println!("  ─────────────────────────");
            println!("  {}", post.content);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        print_error(&err);
        process::exit(1);
    }
}
