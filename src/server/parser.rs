use std::str;

/// Implementation packet parser
///
/// # Methods
///
/// * `fn parse_packet()` : parses packet from buffer
pub struct Parser;

impl Parser {

    /// Parse received packets
    ///
    /// # Arguments
    ///
    /// * `buf` : reference to packet buffer
    ///
    /// This function parses received packets from
    /// client to interpret function codes. According
    /// to proper function codes, corresponding action
    /// is taken on GPIO
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
