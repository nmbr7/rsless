use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufWriter;
use std::process::Command;

pub fn rust_temp(mainfile: String, path: String, prototype: String) -> () {
    //TODO Get function name and function params count from the prototype
    let funcname = prototype.split("(").collect::<Vec<&str>>()[0].trim();
    let arg_count = prototype.split("(").collect::<Vec<&str>>()[1]
        .split(")")
        .collect::<Vec<&str>>()[0]
        .split(",")
        .collect::<Vec<&str>>()
        .len();

    let mut pvec: Vec<String> = Vec::new();
    println!("parsing args");
    for i in 0..arg_count {
        pvec.push(format!("&args[{}]", i + 1));
    }
    let params = pvec.join(",");

    let a = format!(
        "
fn main() {{
    use std::env;
    let args: Vec<String> = env::args().collect();
    let result = {}({});
    println!(\"FaaSRESULT:{{:?}}\",result);
}}
",
        funcname, params
    );

    // Append the Driver funtion to the function file
    let file = OpenOptions::new()
        .append(true)
        .open(format!("{}/{}", path, mainfile))
        .unwrap();
    let mut buf = BufWriter::new(file);

    buf.write_all(a.as_bytes()).unwrap();
    buf.flush().unwrap();

    println!("{}", path);
    let output1 = Command::new("cargo")
        .arg("init")
        .current_dir(format!("{}/new", path))
        .status()
        .expect("Error");

    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(format!("{}/new", path))
        .status()
        .expect("Error");
}
