use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufWriter;
use std::process::Command;

pub fn rust_temp(binaryname: String, path: String, prototype: String) -> () {
    let funcname = prototype.split("(").collect::<Vec<&str>>()[0].trim();
    let args = prototype.split("(").collect::<Vec<&str>>()[1]
        .split(")")
        .collect::<Vec<&str>>()[0]
        .split(",")
        .collect::<Vec<&str>>();

    let mut types: Vec<&str> = Vec::new();

    for i in &args {
        types.push(i.split(":").collect::<Vec<&str>>()[1].trim());
    }

    let mut pvec: Vec<String> = Vec::new();
    println!("parsing args");
    for i in 0..args.len() {
        match types[i] {
            "&u128" => pvec.push(format!("&args[{}].parse::<u128>().unwrap()", i + 1)),
            "u128" => pvec.push(format!("args[{}].parse::<u128>().unwrap()", i + 1)),
            "&u64" => pvec.push(format!("&args[{}].parse::<u64>().unwrap()", i + 1)),
            "u64" => pvec.push(format!("args[{}].parse::<u64>().unwrap()", i + 1)),
            "&u32" => pvec.push(format!("&args[{}].parse::<u32>().unwrap()", i + 1)),
            "u32" => pvec.push(format!("args[{}].parse::<u32>().unwrap()", i + 1)),
            "String" => pvec.push(format!("args[{}]", i + 1)),
            "&String" => pvec.push(format!("&args[{}]", i + 1)),
            _ => return (),
        }
    }
    let params = pvec.join(",");

    let a = format!(
        "
fn main() {{
    use std::env;
    
    enum faastype{{
        int(u32),
        string(String)
    }}
    
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
        .open(format!("{}/src/main.rs", path))
        .unwrap();
    let mut buf = BufWriter::new(file);

    buf.write_all(a.as_bytes()).unwrap();
    buf.flush().unwrap();

    println!("{}", path);
    let output1 = Command::new("cargo")
        .args(&["init", format!("--name={}", binaryname).as_str()])
        .current_dir(format!("{}", path))
        .status()
        .expect("Error");

    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(format!("{}", path))
        .status()
        .expect("Error");
}
