use serde::{Deserialize, Serialize};
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
    data: String,
) -> Result<MsgType, MsgTypeError> {
    let mut buffer = [0; 55512];
    let no = stream.read(&mut buffer).unwrap();
    let recv_data: Value =
        serde_json::from_str(std::str::from_utf8(&buffer[0..no]).unwrap()).unwrap();

    // Handle different client commands
    match recv_data["msg_type"].as_str() {
        Some("MANAGE") => {
            //      let rc: NodeResources = serde_json::from_str(&recv_data.content).unwrap();
            // TODO: Change unwrap()
            let mut result = action(&recv_data).unwrap();
            stream.write(result.as_bytes()).unwrap();
            stream.flush().unwrap();
            //println!("REGISTER\n{:?}", rc);
            Ok(MsgType::MANAGE)
        }
        Some("INVOKE") => {
            //    let rc: StatUpdate = serde_json::from_str(&recv_data.content).unwrap();
            let params = recv_data["params"].to_string();
            let id = recv_data["id"].to_string();
            let mut result = invoke(id, params).unwrap();
            //let put = b"Hello from server--";
            stream.write(result.as_bytes()).unwrap();
            stream.flush().unwrap();
            //println!("UPDATE_SYSSTAT\n{:?}", rc);
            Ok(MsgType::INVOKE)
        }
        _ => Err(MsgTypeError::UnknownMsg),
    }
}
