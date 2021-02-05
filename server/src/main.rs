use h2::server;
use http::{Response, StatusCode};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() {

    let mut listener = TcpListener::bind("127.0.0.1:5928").await.unwrap();

    // Accept all incoming TCP connections.
    loop {
        if let Ok((socket, _)) = listener.accept().await {
            
            // Spawn a new task to process each connection.
            tokio::spawn(async {
                
                // Start the HTTP/2.0 connection handshake
                let mut h2 = server::handshake(socket).await.unwrap();
                
                // Accept all inbound HTTP/2.0 streams sent over the
                // connection.
                while let Some(request) = h2.accept().await {
                    
                    let (request, mut respond) = request.unwrap();
                    
                    println!("Server: Received request: {:?}", request);

                    // Build a response with no body
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .body(())
                        .unwrap();

                    // Send the response back to the client
                    respond.send_response(response, true)
                        .unwrap();
                }

            });
        }
    }
}