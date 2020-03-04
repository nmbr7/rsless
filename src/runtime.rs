// Runtime for different languages using execve
use std::process::Command;

#[derive(Debug)]
pub enum RuntimeError {
    RustError,
}

///  Currently only RPC is implemented (Not Statefull)
pub fn invoke(id: String, params: String) -> Result<String, RuntimeError> {
    //Check for the id to find the function

    //...

    //TODO use command to run the function wrapped binary in a secure environment
    //Fetch the function from the disk and run the function
    let output = Command::new("./binary")
        .arg("params")
        .output()
        .expect("Error");

    //return the result
    Ok(String::from("output"))
}
