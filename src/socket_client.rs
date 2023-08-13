#![allow(warnings, unused)]
use mpsc::TryRecvError;
use std::{
    io::{self, ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration,Instant},
};
use goxoy_address_parser::address_parser::*;

pub enum SocketClientErrorType {
    Connection,
    Communication,
}
pub enum SocketConnectionStatus {
    Connected,
    Disconnected,
}
pub struct SocketClient {
    defined: bool,
    tx:Option<Sender<Vec<u8>>>,
    max_message_size:usize,
    local_addr: String,
    fn_received: Option<fn(Vec<u8>)>,
    fn_error: Option<fn(SocketClientErrorType)>,
    fn_status: Option<fn(SocketConnectionStatus)>,
}

impl SocketClient {
    pub fn new() -> Self {
        SocketClient {
            defined: false,
            tx:None,
            local_addr: String::new(),
            max_message_size:1024,
            fn_error: None,
            fn_received: None,
            fn_status: None,
        }
    }
    pub fn new_with_config(config: AddressParser) -> Self {
        SocketClient {
            defined: true,
            tx:None,
            local_addr: AddressParser::object_to_string(config),
            max_message_size:1024,
            fn_error: None,
            fn_received: None,
            fn_status: None,
        }
    }
    pub fn on_received(&mut self, on_received_callback: fn(Vec<u8>)) {
        self.fn_received = Some(on_received_callback);
    }
    pub fn on_connection_status(&mut self, on_connection_status: fn(SocketConnectionStatus)) {
        self.fn_status = Some(on_connection_status);
    }
    pub fn on_error(&mut self, on_error_callback: fn(SocketClientErrorType)) {
        self.fn_error = Some(on_error_callback);
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
            self.connect_sub_fn();
            return true;
        }
    }
    fn connect_sub_fn(&mut self) -> bool {
        let msg_size=self.max_message_size;
        let addr_obj = AddressParser::string_to_object(self.local_addr.clone());
        let mut client_obj = TcpStream::connect(AddressParser::local_addr_for_binding(addr_obj));
        if client_obj.is_err(){
            return false;
        }
        
        let mut client=client_obj.unwrap();
        client
            .set_nonblocking(true)
            .expect("failed to initiate non-blocking");
    
        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        self.tx=Some(tx.clone());
        let fn_received_clone=self.fn_received;
        let fn_error_clone=self.fn_error;
        thread::spawn(move || loop {
            let mut buff = vec![0; msg_size];
            match client.read_exact(&mut buff) {
                Ok(_) => {
                    let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    if fn_received_clone.is_some() {
                        fn_received_clone.unwrap()(msg.to_vec());
                    }
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => {
                    if fn_error_clone.is_some() {
                        fn_error_clone.unwrap()(SocketClientErrorType::Communication);
                    }
                },
                Err(_) => {
                    println!("connection with server was severed");
                    break;
                }
            }
            match rx.try_recv() {
                Ok(msg) => {
                    client.write_all(&msg).expect("writing to socket failed");
                    println!("message sent {:?}", msg);
                }
                Err(TryRecvError::Empty) => {
                    
                },
                Err(TryRecvError::Disconnected) => {
                    if fn_error_clone.is_some() {
                        fn_error_clone.unwrap()(SocketClientErrorType::Connection);
                    }
                },
            }
    
            thread::sleep(Duration::from_millis(10));
        });        
        return true;
    }
    pub fn send(&mut self, data: Vec<u8>) -> bool {
        if self.tx.is_some(){
            self.tx.as_mut().unwrap().send(data);
            return true;
        }
        return false;
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
    client_obj.on_received(|data| {
        println!(
            "Data Received : {}",
            String::from_utf8(data.clone()).unwrap()
        );
    });
    client_obj.on_connection_status(|connection_status| match connection_status {
        SocketConnectionStatus::Connected => {
            println!("Socket Connected");
        }
        SocketConnectionStatus::Disconnected => {
            println!("Socket Disconnected");
        }
    });
    client_obj.on_error(|error_type| match error_type {
        SocketClientErrorType::Connection => {
            println!("Connection Error");
        }
        SocketClientErrorType::Communication => {
            println!("Communication Error");
        }
    });

    let mut since_the_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    loop {
        if since_the_epoch >= 1_048_575 {
            since_the_epoch = since_the_epoch / 2;
        } else {
            break;
        }
    }
    if client_obj.connect() {
        let client_id_str = format!("{:0>5}", since_the_epoch.to_string());
        println!("CTRL+C to Exit");
        let mut test_data = String::from("message from => ");
        test_data.push_str(&client_id_str);
        client_obj.send(test_data.as_bytes().to_vec());
        /*
        let mut count = 1;
        loop {
            let result_obj = client_obj.send(test_data.as_bytes().to_vec());
            if result_obj == true {
                println!("Message Sended");
            } else {
                println!("Message Sending Error");
            }
            //client_obj.listen(1500);
            count = count + 1;
            if count > 1_000 {
                break;
            }
        }
        */
        //client_obj.close_connection();
    } else {
        println!("Not Connected To Server");
    }
    assert!(true)
}
