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

//geri gelen değerleri bu fonksiyon ile alabilirsiniz.
client_obj.assign_callback(|data| {
    let vec_to_string = String::from_utf8(data).unwrap();
    println!("vec_to_string: {}", vec_to_string);
});

// ayarları nesneyi oluştururken belirttiyseniz.
client_obj.connect();

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

// mesaj göndermek için
let result_obj = client_obj.send("test_msg".as_bytes().to_vec());
if result_obj==true {
    println!("Message Sended");
}else{
    println!("Message Sending Error");
}

// gelen mesajları dinlemek için
// parametre olarak kaç mili saniye dinleyeceği belirtilmeli
// parametre olarak 0 (sıfır) belirtilirse, sürekli dinleme yapar
// en düşük sayı 100 milisaniye
client_obj.listen(1500);

// bağlantıyı kapatmak için
client_obj.close_connection();

```

  
## Lisans

[MIT](https://choosealicense.com/licenses/mit/)