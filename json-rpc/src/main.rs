use jsonrpc_core::{Error, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;
use serde_json::Value;

fn main() {
    let mut io = IoHandler::default();

    io.add_method("say_hello", |_params: Params| async {
        Ok(Value::String("Kelmith says Hello".to_string()))
    });

    io.add_method("add", |params: Params| async {
        match params.parse::<(u32, u32)>() {
            Ok((a, b)) => Ok(Value::Number(serde_json::Number::from(a + b))),
            Err(err) => {
                let message = format!("Invalid params: {}", err);
                Err(Error::invalid_params(message))
            }
        }
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"0.0.0.0:3000".parse::<std::net::SocketAddr>().unwrap())
        .unwrap();

    println!("Server running at {}", server.address());

    server.wait();
}
