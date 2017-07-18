mod parser;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::time;
use std::thread;
use std::str;


/// Server structure and implementation

pub struct Server {
    pub address : String,
}

impl Server {

    /// start server
    pub fn start(&mut self) -> bool
    {
        let rc : bool = true;
        let mut listener = TcpListener::bind(&self.address).unwrap();
        
        println!("Listening for connections...");

        loop{
            match listener.accept() {
                Ok((mut socket, addr)) => {

                    let client_thread = thread::spawn(move || conn_thread(socket));
                    let res  = client_thread.join();
                }

                Err(e) => {println!("Failed to create connection:{}", e)}

            }
        }

        return rc;
    }

}


/// Client connection thread
///
/// # Arguments
///
/// * `socket` - TcpStream variable of connected client
///
/// # Description
///
/// This is a thread function spawned when atleast one client
/// connects to the server. It has infinite loop which reads
/// incoming packets. On closed connection from client, this
/// thread returns.

fn conn_thread(mut socket: TcpStream)
{
    let hundred_ms = time::Duration::from_millis(100);
    let mut buf = [0;1024];
    let mut p = parser::Parser;

    /*
     * Thread loop
     */
    while true {

        let mut msg_len:usize = 0;
        
        /*
         * Check if there is anything to read. Break on
         * closed client connection.
         */

        if let Ok(is_alive) = socket.read(&mut buf) {
            
            if is_alive == 0 {

                println!("Connection to client is closed!\n");
                break;
            }

            else {
                msg_len = is_alive;
            }
        
        }

        p.parse_packet(&buf[0..msg_len]);

        thread::sleep(hundred_ms);
    }
}
