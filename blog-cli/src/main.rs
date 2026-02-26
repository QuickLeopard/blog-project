use anyhow::Result;

use clap::{ArgGroup, Parser};

use blog_client::blog_client::BlogClient;

#[derive(clap::Subcommand, Debug)]
enum Commands {
    GetPosts {
        #[arg(short, long, default_value_t = 0)]
        offset: i32,
        #[arg(short, long, default_value_t = 10)]
        limit: i32,
    },
    GetPost {
        id: i64,
    },
    CountPosts,
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
    #[arg(long, default_value = "127.0.0.1:3000", global = true)]
    server: String,

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
            "Post ID: {}, Title: {}, Content: {}",
            post.id, post.title, post.content
        );
    }

    Ok(())
}

async fn get_post(client: &BlogClient, id: i64) -> Result<()> {
    let post = client.get_post(id).await?;
    println!(
        "Post ID: {}, Title: {}, Content: {}",
        post.id, post.title, post.content
    );
    Ok(())
}

async fn count_posts(client: &BlogClient) -> Result<()> {
    let count = client.count_posts().await?;
    println!("Total posts: {}", count);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let host = cli.server.clone();

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
        Commands::GetPosts { offset, limit } => {
            get_posts(&client, offset, limit).await?;
        }
        Commands::GetPost { id } => {
            get_post(&client, id).await?;
        }
        Commands::CountPosts => {
            count_posts(&client).await?;
        }
    }

    Ok(())
}
