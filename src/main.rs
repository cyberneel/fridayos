// FridayOS runs on its own server, not here. This launcher signs in with publik, then serves one
// local page that sends you to your account. Friday's server runs the same src/publik.rs.
#[allow(dead_code)] // the launcher uses only the gate; Friday uses the rest
mod publik;
use publik::GateUi;
use std::io::{Read, Write};

const ADDR: &str = "127.0.0.1:7340";
const PAGE: &str = include_str!("page.html");

fn main() {
    publik::ensure_signed_in(&mut publik::TerminalUi);
    let listener = std::net::TcpListener::bind(ADDR).unwrap_or_else(|e| panic!("{ADDR}: {e}"));
    publik::TerminalUi.open_url(&format!("http://{ADDR}"));
    for mut s in listener.incoming().flatten() {
        let _ = s.read(&mut [0u8; 2048]);
        let _ = write!(s, "HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=utf-8\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{PAGE}", PAGE.len());
    }
}
