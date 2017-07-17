use std::net::TcpListener;
use std::thread;

/// Server structure and implementation

pub struct Server {
    pub address : String,
    pub max_conn : u16,
}

impl Server {

    /// get maximum connection
    pub fn get_max_conn(&mut self) -> u16
    {
        return self.max_conn;
    }

    /// start server
    pub fn start(&mut self) -> bool
    {
        let rc : bool = true;
        let mut listener = TcpListener::bind(&self.address).unwrap();
        return rc;
    }

}
