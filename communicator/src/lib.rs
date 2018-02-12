// declares modules - defined in eponymous files
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    // super very useful for tests
    use super::client;
    use super::network;
    use super::network::server;

    #[test]
    fn client_connect() {
        client::connect();
    }

    fn network_connect() {
        network::connect();
    }

    fn server_connect() {
        server::connect();
    }        
}
