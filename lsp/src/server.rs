use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct NotificationParams {
    title: String,
    message: String,
    description: String,
};

enum CustomNotifaction{};

impl Notifaction for NotificationParams {
    type Params = CustomNotifaction;

    const METHOD: &'static str = "custom/notification";
     
}

#[derive(Debug)]
struct Backend {
    client: Client,
}; 

impl LanguageServer for Backend {
    async fn initialize (&self, _: InitializeParams) -> Result<InitializeResults, Error> {

        Ok(
            InitializeResults{
                server_info: None,
                capabilities: ServerCapabilities{
                    execute_command_provider: Some(ExecuteCommandOptions{
                        commands: vec!["custom/notification".to_string()],
                        work_done_progress: Default::default()
                    }),
                    ..ServerCapabilities::default()

                }, 
                ..Defualt::default()
            }
        )
    }

    async fn shutdown(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>, Error> {
        match params.commands.as_str(){
            "custom/notification" => {
                self.client.log_message(MessageType::Info, "Custom notification with params {:?}".to_string(), params).await;
                Ok(None )
            },
            _ => {
                self.client.log_message(MessageType::Error, "Unknown command".to_string()).await;
                Err(Error::invalid_request()) 
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.0:5000").await.unwrap();

    let (stream, _) = listener.accept().await.unwrap();

     let (read, write) = tokio::io::split(stream);

     let( service, socket) = Service::new(|client| Backend(client));

    Server::new(read, write)  
    .serve(service)
    .await;
}