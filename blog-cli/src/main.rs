use anyhow::Result;

use clap::{ArgGroup, Parser};

use blog_client::blog_client::BlogClient;

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
        token: String,
    },
    #[command(alias = "delete")]
    DeletePost {
        #[arg(long)]
        id: i64,
        #[arg(long)]
        token: String,
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
        token: String,
    },
    #[command(alias = "list")]
    ListPosts {
        #[arg(short, long, default_value_t = 0)]
        offset: i32,
        #[arg(short, long, default_value_t = 10)]
        limit: i32,
    },
    #[command(alias = "get")]
    GetPost { id: i64 },
    //CountPosts,
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

async fn get_posts(client: &BlogClient, offset: i32, limit: i32) -> Result<()> {
    let posts = client.get_posts(offset, limit).await?;
    println!("Fetched {} posts...", posts.len());
    for post in posts {
        println!(
            "Post ID: {}, Title: \"{}\", Content: \"{}\"",
            post.id, post.title, post.content
        );
    }

    Ok(())
}

async fn get_post(client: &BlogClient, id: i64) -> Result<()> {
    let post = client.get_post(id).await?;
    println!(
        "Post ID: {}, Title: \"{}\", Content: \"{}\"",
        post.id, post.title, post.content
    );
    Ok(())
}

/*async fn count_posts(client: &BlogClient) -> Result<()> {
    let count = client.count_posts().await?;
    println!("Total posts: {}", count);
    Ok(())
}*/

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set different defaults based on protocol
    let host = cli.server.unwrap_or_else(|| {
        if cli.grpc {
            "127.0.0.1:50051".to_string() // Default gRPC port
        } else {
            "127.0.0.1:3000".to_string() // Default HTTP port
        }
    });

    println!(
        "Using server address: {} protocol: {}",
        host,
        if cli.grpc { "gRPC" } else { "HTTP" }
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
            println!(
                "Registered user: {} with email: {} token: \"{}\"",
                response.user.username, response.user.email, response.token
            );
        }
        Commands::Login { username, password } => {
            let response = client.login_user(username, password).await?;
            println!(
                "Logged in user: {} with email: {} token: \"{}\"",
                response.user.username, response.user.email, response.token
            );
        }
        Commands::CreatePost {
            title,
            content,
            token,
        } => {
            let post = client.create_post(title, content, token).await?;
            println!(
                "Created post with ID: {}, Title: \"{}\", Content: \"{}\"",
                post.id, post.title, post.content
            );
        }
        Commands::DeletePost { id, token } => {
            client.delete_post(id, token).await?;
            println!("Deleted post with ID: {}", id);
        }
        Commands::UpdatePost {
            id,
            title,
            content,
            token,
        } => {
            let post = client.update_post(id, title, content, token).await?;
            println!(
                "Updated post with ID: {}, Title: \"{}\", Content: \"{}\"",
                post.id, post.title, post.content
            );
        }
        Commands::ListPosts { offset, limit } => {
            get_posts(&client, offset, limit).await?;
        }
        Commands::GetPost { id } => {
            get_post(&client, id).await?;
        } //Commands::CountPosts => {
          //    count_posts(&client).await?;
          //}
    }

    Ok(())
}
