use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufWriter;
use std::process::Command;

pub fn rust_temp(ind: String, path: String) -> () {
    //TODO
    //Get function name and function params count from the prototype
    let funcname = String::from("call");
    let params = String::from("&args[1],&args[2]");

    let a = format!(
        "

            fn main() {{
                use std::env;
                let args: Vec<String> = env::args().collect();
                
                //let params = &args[1];
                let result = {}({});
                println!(\"FaaSRESULT:{{:?}}\",result);
            }}
    
    ",
        funcname, params
    );

    // Append the Driver funtion to the function file
    let file = OpenOptions::new()
        .append(true)
        .open(format!("{}/{}", path, ind))
        .unwrap();
    let mut buf = BufWriter::new(file);

    buf.write_all(a.as_bytes()).unwrap();
    buf.flush().unwrap();

    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(format!("{}/newsrc", path))
        .status()
        .expect("Error");
}
