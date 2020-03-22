# rsless
An RPC tool/library written in Rust

## Start Server
`
cargo run --release -- server
`

## Creating a function
`
cargo run --release --  client -c 127.0.0.1:9888 create -l 'Rust' -d fibexample -p "fib(arg1:u128) -> u128"
`

Returns a uuid

## Publish uploaded function
`
cargo run --release --  client -c 127.0.0.1:9888 publish -i 5c72527c-373e-43a2-9d29-4bb8f5fe69e1
`

If published returns OK

## Testing using curl
`
curl --header "Content-Type: application/json" --request POST\  --data '{"msg_type":"INVOKE", "params" : ["100"] , "id": "1dbe0457-5747-4ef1-b468-fd5ece355c8f"  }'  127.0.0.1:9888
`


Not Yet Stable
