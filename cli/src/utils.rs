use std::net::TcpListener;

pub fn find_available_port(start: u16, end: u16) -> Option<u16> {
    for port in start..end {
        let addr = format!("127.0.0.1:{}", port);
        match TcpListener::bind(&addr) {
            Ok(listener) => {
                drop(listener);
                return Some(port);
            }
            Err(_) => continue,
        }
    }

    None // Failed to find an available port after 100 attempts
}
