//! This module is our IRC library, abstracts TCP away from user

use std::net::TcpStream;
use std::io::{Write, BufRead, BufReader};

/// This will store and IRC session, authenticate and allow us to reuse the TCP 
/// stream
pub struct Session{
    stream: TcpStream,
    reader: BufReader<TcpStream>,
}
impl Session {
    /// Create a new irc session with NICK and PASS
    pub fn new(uri: &str) -> Self {

        let stream = TcpStream::connect(uri).expect("Cannot connect");
        let reader = std::io::BufReader::new(
            stream.try_clone().expect("Could not clone TcpStream")
        );

        Self{
            stream,
            reader,
        }
    }
    /// Authenticate with a NICK and PASS, gotten from OAUTH Client Auth
    pub fn authenticate(&mut self, nick: &str, pass: &str) {

        println!("{nick}, {pass}");
        self.write(&format!("PASS oauth:{pass}\r\n
        "));
        self.write(&format!("NICK {nick}"));

        println!("{:?}", self.read());
        println!("{:?}", self.read());
        println!("{:?}", self.read());
        println!("{:?}", self.read());
        println!("{:?}", self.read());
        println!("{:?}", self.read());
        println!("{:?}", self.read());
    }
    /// IRC JOIN command
    pub fn join(&mut self, channel: &str) {
        self.write(&format!("JOIN #{channel}"));
        println!("{:?}", self.read());
        println!("{:?}", self.read());
        println!("{:?}", self.read());
    }
    /// IRC PRIVMSG command
    pub fn send(&mut self, channel: &str, msg: &str) {
        self.write(&format!("PRIVMSG #{channel} :{msg}"));
        println!("{:?}", self.read());
        println!("{:?}", self.read());
        println!("{:?}", self.read());
    }


    /// Wrapper around doing TCP Reads
    fn read(&mut self) -> String {
        let mut res: String = String::new(); // This is terrible, SO MANY ALLOCATIONS, do this better............ TODO
        self.reader.read_line(&mut res)
            .expect("Failed to Read from TCP Stream");
        res
    }
    /// Wrapper around doing TCP Writes
    fn write(&mut self, msg: &str) {
        self.stream.write(format!("{msg}\r\n").as_bytes()).expect("Write to TCP Stream Failed");
    }
}
