use goxoy_address_parser::address_parser::*;
use std::{
    io::{Read, Write},
    net::TcpStream,
    time::{Duration, Instant},
};

pub enum SocketClientErrorType {
    Connection,
    Communication,
}
pub enum SocketConnectionStatus {
    Connected,
    Disconnected,
}
#[derive(Debug)]
pub struct SocketClient {
    stream: Option<TcpStream>,
    defined: bool,
    local_addr: String,
    fn_received: Option<fn(Vec<u8>)>,
    fn_error: Option<fn(SocketClientErrorType)>,
    fn_status: Option<fn(SocketConnectionStatus)>,
}

impl SocketClient {
    pub fn new() -> Self {
        SocketClient {
            stream: None,
            local_addr: String::new(),
            defined: false,
            fn_error: None,
            fn_received: None,
            fn_status: None,
        }
    }
    pub fn new_with_config(config: AddressParser) -> Self {
        SocketClient {
            stream: None,
            local_addr: AddressParser::object_to_string(config),
            defined: true,
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
            if self.fn_error.is_some() {
                let fn_error_obj = self.fn_error.unwrap();
                fn_error_obj(SocketClientErrorType::Connection);
            }
            return false;
        }
        if self.fn_status.is_some() {
            let fn_status_obj = self.fn_status.unwrap();
            fn_status_obj(SocketConnectionStatus::Connected);
        }
        self.stream = Some(tcp_stream.unwrap());
        return true;
    }
    pub fn listen(&mut self, how_many_milisecond: u64) {
        let mut new_timeout_val = 0;
        if how_many_milisecond != 0 {
            if how_many_milisecond > 100 {
                new_timeout_val = how_many_milisecond;
            } else {
                new_timeout_val = 100;
            }
        }
        let start = Instant::now();
        loop {
            if new_timeout_val > 0 {
                if start.elapsed().as_millis() > how_many_milisecond as u128 {
                    break;
                }
            }
            let stream = self.stream.as_mut().unwrap().try_clone();
            if stream.is_ok() {
                let mut stream = stream.unwrap();
                let mut data = [0 as u8; 1024];
                if new_timeout_val > 0 {
                    _ = stream.set_read_timeout(Some(Duration::from_millis(new_timeout_val)));
                }
                match stream.read(&mut data) {
                    Ok(_income) => {
                        if self.fn_received.is_some() {
                            let fn_received_obj = self.fn_received.unwrap();
                            fn_received_obj(data.to_vec());
                        }
                    }
                    Err(_) => {}
                }
            } else {
                if self.fn_error.is_some() {
                    let fn_error_obj = self.fn_error.unwrap();
                    fn_error_obj(SocketClientErrorType::Communication);
                }
            }
        }
    }
    pub fn send(&mut self, data: Vec<u8>) -> bool {
        let stream = self.stream.as_mut().unwrap().try_clone();
        if stream.is_ok() {
            let mut stream = stream.unwrap();
            let write_result = stream.write(data.as_slice());
            if write_result.is_ok() {
                let _write_result = write_result.unwrap();
                let mut data = [0 as u8; 1024];
                match stream.read(&mut data) {
                    Ok(_income) => {
                        if self.fn_received.is_some() {
                            let fn_received_obj = self.fn_received.unwrap();
                            fn_received_obj(data.to_vec());
                        }
                        return true;
                    }
                    Err(_) => {
                        if self.fn_error.is_some() {
                            let fn_error_obj = self.fn_error.unwrap();
                            fn_error_obj(SocketClientErrorType::Communication);
                        }
                    }
                }
            } else {
                if self.fn_error.is_some() {
                    let fn_error_obj = self.fn_error.unwrap();
                    fn_error_obj(SocketClientErrorType::Communication);
                }
            }
        } else {
            if self.fn_error.is_some() {
                let fn_error_obj = self.fn_error.unwrap();
                fn_error_obj(SocketClientErrorType::Communication);
            }
        }
        return false;
    }
    /*

            thread::spawn(move || {
                let stream = outer_stream_clone.unwrap();
                let write_result = stream.write(data.as_slice());
                if write_result.is_ok() {
                    let _write_result = write_result.unwrap();
                    let mut data = [0 as u8; 1024];
                    match stream.read(&mut data) {
                        Ok(_income) => {
                            if received_fnc.is_some() {
                                received_fnc.unwrap()(data.to_vec());
                            }
                        }
                        Err(_) => {
                            if error_fnc.is_some() {
                                error_fnc.unwrap()(SocketClientErrorType::Communication);
                            }
                        }
                    }
                }

                /*
                 else {
                    if error_fnc.is_some() {
                        error_fnc.unwrap()(SocketClientErrorType::Communication);
                    }
                    if error_fnc.is_some() {
                        let inner_error = inner_error.unwrap();
                        inner_error(SocketClientErrorType::Communication);
                    } else {
                        println!("try-lock-error-3");
                    }
                    return true;
                }
                */
            });


    */
    pub fn close_connection(&mut self) {
        self.stream = None;
        if self.fn_status.is_some() {
            let fn_status_obj = self.fn_status.unwrap();
            fn_status_obj(SocketConnectionStatus::Disconnected);
        }
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
        let mut count = 1;
        loop {
            let result_obj = client_obj.send(test_data.as_bytes().to_vec());
            if result_obj == true {
                println!("Message Sended");
            } else {
                println!("Message Sending Error");
            }
            client_obj.listen(1500);
            count = count + 1;
            if count > 1_000 {
                break;
            }
        }
        client_obj.close_connection();
    } else {
        println!("Not Connected To Server");
    }
    assert!(true)
}
