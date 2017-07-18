use std::str;

/// Implementation packet parser

pub struct Parser;

impl Parser {

    pub fn parse_packet(&self, mut buf: &[u8])
    {
        /*
         * Read received msg
         */
        match str::from_utf8(&buf) {
            Ok(msg) => {println!("{}", msg);}

            Err(e) => {println!("Error:{}", e);}
        }
    }
}
