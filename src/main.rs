use tokio::net::{TcpListener};

mod command_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // Start an infinite loop that accepts connections and processes them
    loop {

        // Accept a new connection
        let (socket, _) = listener.accept().await?;
        let addr = socket.peer_addr().unwrap();

        // Spawn a new task that processes the connection
        tokio::spawn(async move {
            println!("Client connected: {}", addr);

            // Process the connection using the command handler function from src/command_handler.rs
            if let Err(e) = command_handler::handle_command(socket).await {
                println!("Error: {}", e);
            }

            println!("Client disconnected: {}", addr);
        });
    }
}