pub mod blog_client;
pub mod error;
pub mod grpc_client;
pub mod http_client;
pub mod post;
pub mod traits;
pub mod user;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
