pub mod post;
pub mod http_client;
pub mod blog_client;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
