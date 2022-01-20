use tiny_http::{Server, Response};

const PORT: &str = "0.0.0.0:8000";

fn main() {
    println!("Starting server on {}", PORT);
    let server = Server::http(PORT).unwrap();

    for request in server.incoming_requests() {
        println!("Received request to {:?} {:?}", request.method(), request.url());

        let response = Response::from_string("Hello world!");
        request.respond(response);
    }
}
