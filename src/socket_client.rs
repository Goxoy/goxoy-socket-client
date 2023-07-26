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
    pub fn connect_with_config(&mut self, config: AddressParser) -> bool {
        let local_addr = AddressParser::object_to_string(config);
        self.local_addr = local_addr;
        self.defined = true;
        self.connect_sub_fn()
    }
    pub fn connect(&mut self) -> bool {
        if self.defined == false {
            false
        } else {
            self.connect_sub_fn()
        }
    }
    fn connect_sub_fn(&mut self) -> bool {
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
    pub fn listen(&mut self) {
        loop {
            let stream = self.stream.as_mut().unwrap().try_clone();
            if stream.is_ok() {
                let mut stream = stream.unwrap();
                let mut data = [0 as u8; 1024];
                match stream.read(&mut data) {
                    Ok(_income) => {
                        self.debug_string(format!("reply: {}", from_utf8(&data).unwrap()));
                        if self.callback.is_some() {
                            let callback_obj = self.callback.unwrap();
                            callback_obj(data.to_vec());
                        } else {
                            self.debug_str("callback undefined");
                        }

                        let text = from_utf8(&data);
                        if text.is_ok() {
                            self.debug_string(format!("listen text: {}", text.unwrap()));
                        } else {
                            self.debug_string(format!("listen data: {:?}", data));
                        }
                    }
                    Err(e) => {
                        self.debug_string(format!("failed to listen data: {}", e));
                    }
                }
            } else {
                self.debug_str("tcp stream error");
            }
        }
    }
    pub fn send(&mut self, data: Vec<u8>) -> bool {
        let stream = self.stream.as_mut().unwrap().try_clone();
        if stream.is_ok() {
            let mut stream = stream.unwrap();
            let write_result = stream.write(data.as_slice());
            if write_result.is_ok() {
                self.debug_str("message sended!");
                let _write_result = write_result.unwrap();
                let mut data = [0 as u8; 1024];
                match stream.read(&mut data) {
                    Ok(_income) => {
                        self.debug_string(format!("reply: {}", from_utf8(&data).unwrap()));
                        if self.callback.is_some() {
                            let callback_obj = self.callback.unwrap();
                            callback_obj(data.to_vec());
                        } else {
                            self.debug_str("callback undefined");
                        }

                        let text = from_utf8(&data);
                        if text.is_ok() {
                            self.debug_string(format!("reply text: {}", text.unwrap()));
                        } else {
                            self.debug_string(format!("reply data: {:?}", data));
                        }
                        return true;
                    }
                    Err(e) => {
                        self.debug_string(format!("failed to receive data: {}", e));
                    }
                }
            } else {
                self.debug_str("message sending error");
            }
        } else {
            self.debug_str("tcp stream error");
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
    let mut client_obj = SocketClient::new_with_config(AddressParser {
        ip_address: "127.0.0.1".to_string(),
        port_no: 1234,
        protocol_type: ProtocolType::TCP,
        ip_version: IPAddressVersion::IpV4,
    });
    client_obj.debug_mode(true);
    client_obj.assign_callback(|data| {
        let vec_to_string = String::from_utf8(data).unwrap();
        println!("vec_to_string: {}", vec_to_string);
    });
    client_obj.connect();
    let result_obj = client_obj.send("test_msg".as_bytes().to_vec());
    println!("result_obj: {:?}", result_obj);
    client_obj.listen();
    client_obj.close_connection();
    assert!(true)
}
