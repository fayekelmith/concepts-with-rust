use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = tokio::net::TcpStream::connect("127.0.0.0:5000").await?;

    let initialize_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": 1234,
            "rootUri": "file:///home/user/project",
            "capabilities": {
                "workspace": {
                    "executeCommandProvider": {
                        "commands": ["custom/notification"]
                    }
                }
            }
        }
    });

    let initialize_request_str = initialize_request.to_string();
     
     let initialize_stream_formated = format!("Content-Length: {}\r\n\r\n{}", initialize_request_str.len(), initialize_request_str);

     stream.write_all(initialize_stream_formated.as_bytes()).await?;

     let mut buffer = [0; 1024];

     let n = stream.read(&mut buffer).await?;

     let response = String::from_utf8_lossy(&buffer[..n]);

     println!("Response: {}", response); 

     let execute_command_request = json!({
         "jsonrpc": "2.0",
         "id": 2,
         "method": "custom/notification",
         "params": {
             "message": "Hello from the client"
         }
     });

     let execute_command_request_str = execute_command_request.to_string();

     let formatted_execute_command_request = format!("Content-Length: {}\r\n\r\n{}", execute_command_request_str.len(), execute_command_request_str); 

        stream.write_all(formatted_execute_command_request.as_bytes()).await?;

        let n = stream.read(&mut buffer).await?;

        let response = String::from_utf8_lossy(&buffer[..n]);

     Ok(())
}