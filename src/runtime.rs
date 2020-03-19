// Runtime for different languages

/* Json Format for the function invocation
{
    "id" : "13123-312312-21312-13223",
    "params" : ["arg1", "arg2", "..."],
}
*/

use redis::Commands;
use std::process::Command;

#[derive(Debug)]
pub enum RuntimeError {
    RustError,
}

//  Currently only RPC is implemented (Not Statefull)
pub fn invoke(id: String, mut params: Vec<String>) -> Result<String, RuntimeError> {
    //Check for the id to find the function
    let pub_db = redis::Client::open("redis://172.28.5.3/3").unwrap();
    let mut con_pub = pub_db.get_connection().unwrap();
    let val: String = con_pub
        .get(&id.replace("\"", ""))
        .unwrap_or("Function not created or published".to_string());

    //TODO Return the sandboxed container address
    //Now  Return the path of the function executable

    let mut v7: Vec<&str> = params.iter().map(|s| s.as_ref()).collect();

    //TODO use command to run the function wrapped binary in a secure environment
    let output = Command::new(val)
        .args(&v7)
        .output()
        .expect("Error Running the function");
    let response = std::str::from_utf8(&output.stdout).unwrap();

    Ok(response
        .to_string()
        .split("FaaSRESULT:")
        .collect::<Vec<&str>>()[1]
        .to_string())
}
