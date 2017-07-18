mod server;

/// Main function
fn main() {

    let mut s = server::Server {
        address : "127.0.0.1:3333".to_string(),
    };

    s.start();
}
