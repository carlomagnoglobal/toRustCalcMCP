//! Web-based REPL for rcalc.

use serde_json::{json, Value as J};
use std::io::Read;
use std::sync::{Arc, Mutex};
use tiny_http::{Request, Response, Server};
use torustcalcmcp::eval::Interp;

fn main() {
    let port = 8888;
    let server = Server::http(format!("127.0.0.1:{}", port))
        .expect("Failed to bind to port");

    println!("rcalc web interface running at: http://localhost:{}", port);
    println!("Press Ctrl+C to stop.\n");

    let interp = Arc::new(Mutex::new(Interp::new()));

    for request in server.incoming_requests() {
        let interp = Arc::clone(&interp);
        handle_request(request, &interp);
    }
}

fn handle_request(mut request: Request, interp: &Arc<Mutex<Interp>>) {
    let path = request.url();

    match (request.method().as_str(), path) {
        ("GET", "/") => {
            let html = include_str!("../web/index.html");
            let response = Response::from_string(html).with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
                    .unwrap(),
            );
            let _ = request.respond(response);
        }
        ("GET", "/style.css") => {
            let css = include_str!("../web/style.css");
            let response = Response::from_string(css).with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/css"[..]).unwrap(),
            );
            let _ = request.respond(response);
        }
        ("GET", "/app.js") => {
            let js = include_str!("../web/app.js");
            let response = Response::from_string(js).with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/javascript"[..])
                    .unwrap(),
            );
            let _ = request.respond(response);
        }
        ("POST", "/api/calc") => {
            let mut body_str = String::new();
            if request.as_reader().read_to_string(&mut body_str).is_ok() {
                if let Ok(body) = serde_json::from_str::<J>(&body_str) {
                    if let Some(expr) = body.get("expression").and_then(|v| v.as_str()) {
                        let mut it = interp.lock().unwrap();
                        let result = it.eval_render(expr);

                        let response_json = match result {
                            Ok(text) => json!({
                                "success": true,
                                "result": text,
                                "expression": expr
                            }),
                            Err(err) => json!({
                                "success": false,
                                "error": err,
                                "expression": expr
                            }),
                        };

                        let response = Response::from_string(response_json.to_string())
                            .with_header(
                                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                                    .unwrap(),
                            )
                            .with_header(
                                tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..])
                                    .unwrap(),
                            );
                        let _ = request.respond(response);
                        return;
                    }
                }
            }

            let error_response = json!({
                "success": false,
                "error": "Invalid request"
            });
            let response = Response::from_string(error_response.to_string())
                .with_status_code(400)
                .with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                        .unwrap(),
                );
            let _ = request.respond(response);
        }
        _ => {
            let response = Response::from_string("Not found").with_status_code(404);
            let _ = request.respond(response);
        }
    }
}
