use crate::templates::rust_temp;
use redis::Commands;
use serde_json::Value;
use std::fs::OpenOptions;
use std::fs::{self, DirBuilder};
use std::io::prelude::*;
use std::io::BufWriter;

#[derive(Debug)]
pub enum action_error {
    PubError,
    DeleteError,
    CreateError,
    Unknown,
}
fn faas_create(data: &Value) -> Result<String, action_error> {
    let lang = data["lang"].as_str().unwrap();
    let filenames = data["file_name"].as_array().unwrap();
    let dirs = &data["dirs"].as_array().unwrap();
    let files = &data["files"];

    //println!("{}\n{:?}\n{:?}",lang,filenames,dirs);

    let path = String::from("temp");
    for i in 0..dirs.len() {
        DirBuilder::new()
            .recursive(true)
            .create(format!("{}/{}", path, dirs[i].as_str().unwrap()))
            .unwrap();
    }

    let mut ind: String = String::from("");
    for i in 0..filenames.len() {
        if filenames[i].as_str().unwrap().ends_with("/main.rs") {
            ind = filenames[i].as_str().unwrap().to_string();
        }
    }
    //let mainfile = &files[ind].as_str().unwrap();
    //println!("{}",mainfile);

    for i in 0..filenames.len() {
        let file = filenames[i].as_str().unwrap().to_string();
        let fh = OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{}/{}", path, file))
            .unwrap();
        let mut buf = BufWriter::new(fh);
        buf.write_all(
            files[file]
                .as_str()
                .unwrap()
                .replace("\\u{27}", "\'")
                .as_bytes(),
        )
        .unwrap();
        buf.flush().unwrap();
    }
    match lang {
        "Rust" => {
            //println!("Reached {}",lang);
            let path = String::from("temp");
            //let proto = String::from("sd");
            rust_temp(ind, path);
            Ok(String::from("uuid"))
        }
        //Some("Python") => ,
        //Some("c") => ,
        _ => Err(action_error::Unknown),
    }
}
/*
fn faas_update() -> () {
}
*/
fn faas_delete(id: String) -> Result<String, action_error> {
    let client = redis::Client::open("redis://172.28.5.3/2").unwrap();
    let mut con = client.get_connection().unwrap();
    let _: () = con.del(&id).unwrap();
    Ok(String::from("OK"))
}
//After publishing, the function will be invokable
fn faas_publish(id: String) -> Result<String, action_error> {
    let dev_db = redis::Client::open("redis://172.28.5.3/2").unwrap();
    let pub_db = redis::Client::open("redis://172.28.5.3/3").unwrap();

    let mut con_dev = dev_db.get_connection().unwrap();
    let mut con_pub = pub_db.get_connection().unwrap();

    let val: String = con_dev.get(&id).unwrap();
    let _: () = con_pub.set(&id, &val).unwrap();
    Ok(String::from("OK"))
}
// TODO Currently only RPC is implemented (ie, Not Statefull)
pub fn action(data: &Value) -> Result<String, action_error> {
    //Check for the subcommands
    match data["action"].as_str() {
        // Create
        Some("create") => faas_create(&data),
        // Update
        //Some("update") => faas_update(id, function_proto,function_body),
        // Delete
        Some("delete") => {
            let id = data["id"].to_string();
            faas_delete(id)
        }
        // Publish
        Some("publish") => {
            let id = data["id"].to_string();
            faas_publish(id)
        }
        _ => Err(action_error::Unknown),
    }
}