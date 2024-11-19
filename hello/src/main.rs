// main.rs
use hello::ThreadPool;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process::Command,
    thread,
    time::Duration,
};

#[derive(Serialize, Deserialize)]
struct DenoResponse {
    html: String,
    status: u16,
}

fn main() {
    // Start Deno server as a separate process
    let deno_handle = thread::spawn(|| {
        Command::new("deno")
            .args(["run", "--allow-net", "frontend/server.ts"])
            .spawn()
            .expect("Failed to start Deno server");
    });

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    println!("Rust server listening on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Forward request to Deno server
    let client = reqwest::blocking::Client::new();
    let deno_response = match &request_line[..] {
        "GET / HTTP/1.1" => client
            .get("http://localhost:8000")
            .send()
            .unwrap()
            .json::<DenoResponse>()
            .unwrap(),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            client
                .get("http://localhost:8000")
                .send()
                .unwrap()
                .json::<DenoResponse>()
                .unwrap()
        }
        _ => client
            .get("http://localhost:8000/404")
            .send()
            .unwrap()
            .json::<DenoResponse>()
            .unwrap(),
    };

    let status_line = format!("HTTP/1.1 {} OK", deno_response.status);
    let contents = deno_response.html;
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
