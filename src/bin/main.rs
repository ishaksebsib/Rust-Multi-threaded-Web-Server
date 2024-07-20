use multi_threaded_web_server::ThreadPool;
use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    println!("Server is running on http://127.0.0.1:7878");
    for stream in listener.incoming().take(10) {
        let stream = stream.unwrap();
        pool.excute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // check is the request is to main page (url+/) else response with 404
    let route_home = b"GET / HTTP/1.1\r\n";
    let route_sleep = b"GET /sleep HTTP/1.1\r\n";

    let response = if buffer.starts_with(route_home) {
        create_response("hello.html", "200 OK")
    } else if buffer.starts_with(route_sleep) {
        thread::sleep(Duration::from_secs(5));
        create_response("hello.html", "200 OK")
    } else {
        create_response("404.html", "404 Not Found")
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn create_response(file_path: &str, status: &str) -> String {
    let mut page = File::open(&file_path).unwrap();
    let mut page_contents = String::new();
    page.read_to_string(&mut page_contents).unwrap();

    format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        status,
        page_contents.len(),
        page_contents
    )
}
