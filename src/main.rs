mod server;

extern crate sysfs_gpio;

/// Main function
///
/// #Description
///
/// Application entry point which starts the TCP
/// server and listens for connection.
fn main() {


    /*
     * Declare and start server
     */
    let mut io_server = server::Server {
        address : "127.0.0.1:3333".to_string(),
    };

    io_server.start();
}
