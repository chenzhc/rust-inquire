#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]
use dotenv::dotenv;
use log::info;

const ECHO_SERVER_ADDRESS: &str = "localhost:8000";



#[cfg(test)]
mod tests {
    use std::{io::{Read, Write}, net::TcpStream};

    use rust_inquire::init;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    use super::*;

    #[tokio::test]
    async fn it_echo_client_tokio_test() {
        init();
        // connection 
        info!("connecting to {}", ECHO_SERVER_ADDRESS);

        if let Ok(mut stream) = tokio::net::TcpStream::connect(ECHO_SERVER_ADDRESS).await {
            // connected
            info!("connected to echo server {}:{}",
                stream.local_addr().unwrap().ip(),
                stream.local_addr().unwrap().port()
            );

            // write a hello world message 
            let message = "hellow rold";
            let _ = stream.write_all(message.as_bytes()).await;
            info!("send: {}", message);

            // read the result
            let mut buffer: [u8; 1024] = [0; 1024];
            let len = stream.read(&mut buffer).await.unwrap();
            let message = String::from_utf8_lossy(&buffer[..len]);
            info!("received: {}", message);

        } else {
            info!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS);
        }
    }

    #[tokio::test]
    async fn it_test01() {
        init();
        // connection 
        info!("connecting to {}", ECHO_SERVER_ADDRESS);

        if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
            // connected
            info!("connected to echo server {}:{}",
                stream.local_addr().unwrap().ip(),
                stream.local_addr().unwrap().port()
            );

            // write a hello world message 
            let message = "hellow rold";
            let _ = stream.write(message.as_bytes());
            let _ = stream.flush();
            info!("send: {}", message);

            // read the result
            let mut buffer: [u8; 1024] = [0; 1024];
            let len = stream.read(&mut buffer).unwrap();
            let message = String::from_utf8_lossy(&buffer);
            info!("received: {}", message);

        } else {
            info!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS);
        }


    }
}