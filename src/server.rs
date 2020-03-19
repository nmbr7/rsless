mod manage;
mod runtime;
mod templates;

use serde_json::Value;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;

use crate::manage::action;
use crate::runtime::invoke;

pub enum MsgType {
    MANAGE,
    INVOKE,
}

#[derive(Debug)]
pub enum MsgTypeError {
    UnknownMsg,
}

// TODO : Implement using a http server inorder to handle file upload
// Main FaaS server handler
pub fn server_api_handler(
    mut stream: TcpStream,
    server_dup_tx: mpsc::Sender<String>,
) -> Result<MsgType, MsgTypeError> {
    //TODO reduce  the buffer size  and use a loop
    let mut buffer = [0; 55512];
    let no = stream.read(&mut buffer).unwrap();

    let mut data = std::str::from_utf8(&buffer[0..no]).unwrap();
    if data.contains("\r\n\r\n") {
        data = data.split("\r\n\r\n").collect::<Vec<&str>>()[1];
    }

    //println!("{:?}", data);
    let recv_data: Value = serde_json::from_str(data).unwrap();
    // Handle different client commands
    msg_parser(&mut stream, recv_data)
}

pub fn msg_parser(stream: &mut TcpStream, recv_data: Value) -> Result<MsgType, MsgTypeError> {
    match recv_data["msg_type"].as_str() {
        Some("MANAGE") => {
            // TODO: Change unwrap()
            let mut result = action(&recv_data).unwrap();
            stream.write_all(result.as_bytes()).unwrap();
            stream.flush().unwrap();
            Ok(MsgType::MANAGE)
        }
        Some("INVOKE") => {
            let paramsval = recv_data["params"].as_array().unwrap();
            let mut params: Vec<String> = Vec::new();
            for i in 0..paramsval.len() {
                params.push(paramsval[i].as_str().unwrap().to_string());
            }
            let id = recv_data["id"].to_string();
            let mut result = invoke(id, params).unwrap();
            stream
                .write_all(format!("HTTP/1.1 200 OK\r\n\r\n{}", result).as_bytes())
                .unwrap();
            stream.flush().unwrap();
            Ok(MsgType::INVOKE)
        }
        _ => Err(MsgTypeError::UnknownMsg),
    }
}
