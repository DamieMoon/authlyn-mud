use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

// This function processes the commands sent by the client
pub async fn handle_command(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {

    // Create a buffer to store the data received from the client
    let mut buf = [0; 1024];

    loop {
        // Write a prompt to the client and read the command
        socket.write_all(b"> ").await?;
        let n = socket.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }

        // Process the command and send a response to the client
        let cmd = String::from_utf8_lossy(&buf[..n]);
        match cmd.trim() {
            "quit" => {
                socket.write_all(b"Bye!\n").await?;
                return Ok(());
            }
            "echo" => {
                socket.write_all(b"Echo: ").await?;
                socket.write_all(&buf[..n]).await?;
            }
            _ => {
                socket.write_all(b"Unknown command\n").await?;
            }
        }
    }
}
