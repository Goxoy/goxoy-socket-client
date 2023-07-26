use goxoy_address_parser::address_parser::*;
use std::{
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8,
};

#[derive(Debug)]
pub struct SocketClient {
    debug: bool,
    stream: Option<TcpStream>,
    defined: bool,
    local_addr: String,
    callback: Option<fn(Vec<u8>)>,
}

impl SocketClient {
    pub fn new() -> Self {
        SocketClient {
            debug: true,
            stream: None,
            local_addr: String::new(),
            defined: false,
            callback: None,
        }
    }
    pub fn new_with_config(config: AddressParser) -> Self {
        SocketClient {
            debug: true,
            stream: None,
            local_addr: AddressParser::object_to_string(config),
            defined: true,
            callback: None,
        }
    }
    pub fn remove_assigned_callback(&mut self) {
        self.callback = None;
    }
    pub fn assign_callback(&mut self, callback: fn(Vec<u8>)) {
        self.callback = Some(callback);
    }
    fn debug_str(&self, str: &str) {
        println!("{}", str);
    }
    fn debug_string(&self, str: String) {
        println!("{}", str);
    }
    pub fn debug_mode(&mut self, debug: bool) {
        self.debug = debug;
    }
    pub fn start_with_config(&mut self, config: AddressParser) -> bool {
        let local_addr = AddressParser::object_to_string(config);
        self.local_addr = local_addr;
        self.defined = true;
        self.start_sub_fn()
    }
    pub fn start(&mut self) -> bool {
        if self.defined == false {
            false
        } else {
            self.start_sub_fn()
        }
    }
    fn start_sub_fn(&mut self) -> bool {
        let addr_obj = AddressParser::string_to_object(self.local_addr.clone());
        let mut local_addr = String::from(addr_obj.ip_address);
        local_addr.push_str(":");
        local_addr.push_str(&addr_obj.port_no.to_string());
        let tcp_stream = TcpStream::connect(local_addr);
        if tcp_stream.is_err() {
            return false;
        }
        self.stream = Some(tcp_stream.unwrap());
        return true;
    }
    pub fn send_to(&mut self) -> bool {
        let stream = self.stream.as_mut().unwrap().try_clone();
        if stream.is_ok() {
            let mut stream = stream.unwrap();
            let msg = b"test_text";
            let write_result = stream.write(msg);
            if write_result.is_ok() {
                let _write_result = write_result.unwrap();
                let mut data = [0 as u8; 9];
                match stream.read_exact(&mut data) {
                    Ok(income) => {
                        dbg!(income);
                        if &data == msg {
                            self.debug_str("Reply is ok!");
                        } else {
                            let text = from_utf8(&data).unwrap();
                            self.debug_string(format!("Unexpected reply: {}", text));
                        }
                        return true;
                    }
                    Err(e) => {
                        self.debug_string(format!("Failed to receive data: {}", e));
                    }
                }
            } else {
                self.debug_str("write error");
            }
        } else {
            self.debug_str("stream error");
        }
        return false;
    }
    pub fn close_connection(&mut self) {
        self.stream = None;
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:1234") {
        println!("Connected to the server!");
        let msg = b"Hello!";
        for _i in 1..5 {
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        std::thread::sleep(std::time::Duration::from_millis(5000));
        assert!(true)
    } else {
        println!("Couldn't connect to server...");
        assert!(false)
    }
}
