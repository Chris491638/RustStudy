#[cfg(test)]
mod tests {
    use super::client;
    use super::network;

    #[test]
    fn it_works() {
        client::connect();
        network::connect();
    }
}

pub mod client;
pub mod network;