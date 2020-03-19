// FaaS Base Tcp Server

// Doesn't handle anything related to the core functionality of the FaaS system
// The server.rs module provide the core functions

/*
   CURRENT IMPLEMENTATION

   <PHASE 1>
      1) A Request-Response FaaS system
      2) A Remote Procedure call model
*/

#[macro_use]
extern crate clap;
extern crate redis;
extern crate walkdir;

//mod manage;
//mod runtime;
//mod server;
//mod templates;

use serde_json::{json, Value};
use walkdir::WalkDir;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

use librsless::server_api_handler;

pub fn server_api_main(server_tx: mpsc::Sender<String>) -> () {
    let listener = TcpListener::bind("0.0.0.0:9888").unwrap();
    println!("Waiting for connections");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let data = stream.peer_addr().unwrap().to_string();

        println!("Received connection from IP :- {}", &data);

        let server_dup_tx = mpsc::Sender::clone(&server_tx);

        thread::spawn(move || {
            let response = server_api_handler(stream, server_dup_tx);
        });
    }
}

fn dirjson(dir: String) -> String {
    let mut directory: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_dir() {
            directory.push(entry.path().to_str().unwrap().to_string());
        } else {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }

    let mut all: Vec<String> = Vec::new();
    for i in &files {
        let mut file = File::open(&i).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let test = format!(" {:?} : {:?} ", i, std::str::from_utf8(&buf).unwrap());
        all.push(test);
        //let format!("{}",test);
    }
    let dirs = format!("\"dirs\" : {:?}", directory);
    let file_name = format!("\"file_name\" : {:?}", files);
    let file_data = format!("\"files\" : {{ {} }}", all.join(",")).replace("\'", "\\u{27}");
    let all = format!(" {} , {} , {} ", dirs, file_name, file_data);
    all
}

fn main() -> std::io::Result<()> {
    let matches = clap_app!(rsless =>
    (version: "0.1.0")
    (author: "nmbr_7")
    (about: "A Server less solution")
    (@subcommand server =>
     (about: "Subcommand to use server")
     (version: "0.1.0")
     (author: "nmbr_7")
     )
    (@subcommand client =>
     (about: "Subcommand to use client")
     (version: "0.1.0")
     (author: "nmbr_7")
     (@arg connect: -c --connect +takes_value "destination addr and port")
     (@subcommand create =>
            (about: "list all the files")
            (version: "0.1.0")
            (author: "nmbr_7")
            (@arg lang: -l --lang +takes_value "function language")
            (@arg prototype: -p --proto +takes_value "function language")
            (@arg dir: -d --dir +takes_value "Function Directory (Directory must contain the function prototype file, funtion definition file, dependency modules and config files MAX Size should be less than 5MB)")
            )
     (@subcommand update =>
            (about: "list all the files")
            (version: "0.1.0")
            (author: "nmbr_7")
            (@arg id: -id --identifier +takes_value "function id")
            )
     (@subcommand delete =>
            (about: "list all the files")
            (version: "0.1.0")
            (author: "nmbr_7")
            (@arg id: -id --identifier +takes_value "function id")
            )
     (@subcommand publish =>
            (about: "list all the files")
            (version: "0.1.0")
            (author: "nmbr_7")
            (@arg id: -id --identifier +takes_value "function id")
            )
     )
     )
    .get_matches();

    match matches.subcommand() {
        ("server", Some(server_matches)) => {
            println!("server");
            let (server_tx, server_rx) = mpsc::channel();
            let _server_thread = thread::spawn(move || {
                server_api_main(server_tx);
            });
            loop {
                let received = server_rx.try_recv();
                match received {
                    Ok(s) => {
                        println!("Received from Node Client: {}", &s);
                    }
                    Err(_) => (),
                };
            }
        }

        ("client", Some(client_matches)) => {
            println!("client");
            let addr = client_matches.value_of("connect");
            let mut stream = TcpStream::connect(addr.unwrap())?;
            let msg_data = match client_matches.subcommand() {
                ("create", Some(create_matches)) => {
                    let lang = create_matches.value_of("lang").unwrap().to_string();
                    let prototype = create_matches.value_of("prototype").unwrap().to_string();
                    let dir = create_matches.value_of("dir").unwrap().to_string();
                    let djson = dirjson(dir);
                    let data = format!("{{ \"msg_type\": \"MANAGE\" , \"action\": \"create\",\"lang\": {:?}, \"prototype\": {:?}, {} }}",lang, prototype, djson);
                    //TEST
                    Ok(data)
                    //println!("{}",data);
                    //stream.write(data.as_bytes()).unwrap();
                    //stream.flush().unwrap();
                }
                ("update", Some(update_matches)) => {
                    let id = update_matches.value_of("id");
                    let data = json!({
                        "msg_type": "MANAGE",
                    })
                    .to_string();
                    Ok(data)
                }
                ("delete", Some(delete_matches)) => {
                    let id = delete_matches.value_of("id");
                    let data = json!({
                        "msg_type": "MANAGE",
                        "action": "delete",
                        "id": id
                    })
                    .to_string();
                    Ok(data)
                }
                ("publish", Some(publish_matches)) => {
                    let id = publish_matches.value_of("id");
                    let data = json!({
                        "msg_type": "MANAGE",
                        "action": "publish",
                        "id": id
                    })
                    .to_string();
                    Ok(data)
                }

                _ => Err("No valid subcommand was used"),
            };
            stream.write_all(msg_data.unwrap().as_bytes()).unwrap();
            stream.flush().unwrap();
            println!("Sent");
            let mut buffer = [0; 512];
            let no = stream.read(&mut buffer).unwrap();
            let mut data = std::str::from_utf8(&buffer[0..no]).unwrap();
            println!("Returned: {}", data);
        }

        _ => println!("No valid subcommand was used"),
    };
    Ok(())
}
