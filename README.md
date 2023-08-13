# Goxoy Socket Client

Soket İstemcisi için RUST tabanlı kütüphane.


## Kullanım / Örnekler

```rust
// önce nesneyi oluşturup, sonrasında ayarları tanımlayabilirsiniz.
let mut client_obj = SocketClient::new());

// bağlantı esnasında ayarları tanımlayabilirsiniz.
client_obj.connect_with_config(AddressParser {
    ip_address: "127.0.0.1".to_string(),
    port_no: 1234,
    protocol_type: ProtocolType::TCP,
    ip_version: IPAddressVersion::IpV4,
});


// ayarlar ile nesneyi oluşturmak için
let mut client_obj = SocketClient::new_with_config(AddressParser {
    ip_address: "127.0.0.1".to_string(),
    port_no: 1234,
    protocol_type: ProtocolType::TCP,
    ip_version: IPAddressVersion::IpV4,
});

//mesaj gelince devreye girecek fonksiyon
client_obj.on_received( |data| {
    println!("Data Received : {}", String::from_utf8(data.clone()).unwrap());
});

// sunucu bağlantı durumları tetiklendiğinde
client_obj.on_connection_status( |connection_status| {
    match connection_status {
        SocketConnectionStatus::Connected => {
            println!("Socket Connected");
        },
        SocketConnectionStatus::Disconnected => {
            println!("Socket Disconnected");
        },
    }
});

// hata oluştuğunda devreye girecek fonksiyon
client_obj.on_error(|error_type| {
    match error_type {
        SocketClientErrorType::Connection => {
            println!("Connection Error");
        },
        SocketClientErrorType::Communication => {
            println!("Communication Error");
        },
    }
});

// ayarları nesneyi oluştururken belirttiyseniz, doğrudan bağlantı kurabilirsiniz.
client_obj.connect();

// mesaj göndermek için
let result_obj = client_obj.send("test_msg".as_bytes().to_vec());
if result_obj==true {
    println!("Message Sended");
}else{
    println!("Message Sending Error");
}

```

  
## Lisans

[MIT](https://choosealicense.com/licenses/mit/)