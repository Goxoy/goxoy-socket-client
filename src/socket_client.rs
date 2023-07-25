use goxoy_address_parser::address_parser::*;
use std::{
    io::{Read, Write},
    net::TcpStream, str::from_utf8,
};

#[derive(Debug)]
pub struct SocketClient {
    stream:Option<TcpStream>,
    defined: bool,
    local_addr: String,
    callback: Option<fn(Vec<u8>)>,
}

impl SocketClient {
    pub fn new() -> Self {
        SocketClient {
            stream:None,
            local_addr: String::new(),
            defined: false,
            callback: None,
        }
    }
    pub fn new_with_config(config: AddressParser) -> Self {
        SocketClient {
            stream:None,
            local_addr:AddressParser::object_to_string(config),
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
    pub fn start_with_config(&mut self, config: AddressParser)->bool{
        let local_addr = AddressParser::object_to_string(config);
        self.local_addr = local_addr;
        self.defined = true;
        self.start_sub_fn()
    }
    pub fn start(&mut self)->bool{
        if self.defined==false{
            false
        }else{
            self.start_sub_fn()
        }
    }
    fn start_sub_fn(&mut self)->bool{
        let addr_obj=AddressParser::string_to_object(self.local_addr.clone());
        let mut local_addr=String::from(addr_obj.ip_address);
        local_addr.push_str(":");
        local_addr.push_str(&addr_obj.port_no.to_string());
        let tcp_stream=TcpStream::connect(local_addr);
        if tcp_stream.is_err(){
            return false;
        }
        self.stream=Some(tcp_stream.unwrap());
        return true;
    }
    pub fn send_to(&mut self){
        let mut stream=self.stream.as_mut().unwrap().try_clone().unwrap();
        let msg = b"test_text";
        stream.write(msg).unwrap();
        let mut data = [0 as u8; 9];
        match stream.read_exact(&mut data) {
            Ok(_) => {
                if &data == msg {
                    println!("Reply is ok!");
                } else {
                    let text = from_utf8(&data).unwrap();
                    println!("Unexpected reply: {}", text);
                }
            },
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }
    pub fn close_connection(&mut self){
        self.stream=None;
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    let mut client_obj = SocketClient::new();
    client_obj.start_with_config(AddressParser { 
        ip_address: "127.0.0.1".to_string(),
        port_no: 1234,
        protocol_type: ProtocolType::TCP,
        ip_version: IPAddressVersion::IpV4,
    });
    
    println!("server_obj.local_addr: {}", client_obj.local_addr);
    client_obj.assign_callback(|data| {
        let vec_to_string = String::from_utf8(data).unwrap(); // Converting to string
        println!("vec_to_string: {}", vec_to_string); // Output: Hello World
    });
    client_obj.start();
    assert!(true)
}
