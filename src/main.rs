//This is a barebone server

use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
/*
**To check if the connection is established or not
fn main(){
    let Listener=TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in Listener.incoming(){
        let stream=stream.unwrap();
        println!("Connection Established");
    }
}
*/


fn main(){
    let Listener=TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in Listener.incoming(){
        let stream=stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream){
    let mut buffer=[0;1024];
    stream.read(&mut buffer).unwrap();
    /*println!(
        "Request:{}",String::from_utf8_lossy(&buffer[..])
    );*/
    let contents=fs::read_to_string("index.html").unwrap();
    let response=format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
/*
after cargo run;
/*
     Running `target/debug/server`
Request:GET / HTTP/1.1
Host: 127.0.0.1:8000
Connection: keep-alive
Cache-Control: max-age=0
sec-ch-ua: " Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "Linux"
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
Sec-Fetch-Site: none
Sec-Fetch-Mode: navigate
Sec-Fetch-User: ?1
Sec-Fetch-Dest: document
Accept-Encoding: gzip, deflate, br
Accept-Language: en-GB,en-US;q=0.9,en;q=0.8
*/
