mod server;

/// Main function
fn main() {

    let mut s = server::Server {
        address : "127.0.0.1:3333".to_string(),
        max_conn : 100
    };

    println!("Maximum connections : {}", s.get_max_conn());
}
