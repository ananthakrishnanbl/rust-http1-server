use crate::static_file::{StaticServe};
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use crate::error::{Result};

#[derive(Debug)]
pub struct Response{
    status_code: u32,
    reason: String,
    content_type: Option<String>,
    body: Option<Vec<u8>>,
    keep_alive: bool,
}

impl From<StaticServe> for Response{
    fn from(serve: StaticServe) -> Self{
        Response{
            status_code: serve.status_code,
            reason: serve.reason,
            content_type: serve.content_type,
            body: serve.content,
            keep_alive: false,
        }
    }
}

impl Response{
    pub fn set_keep_alive(&mut self, keep_alive: bool){
        self.keep_alive=keep_alive;
    }
}

pub fn write_response(
    stream: &mut TcpStream,
    r: Response,
) -> Result<()>{
    let mut head=String::new();

    let body: Vec<u8> = match r.body{
        Some(b)=>b,
        None=> Vec::new(),
    };

    if let Some(c_type)=r.content_type{
        head = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: {}\r\n\r\n",
            r.status_code,
            r.reason,
            c_type,
            body.len(),
            if r.keep_alive { "keep-alive" } else { "close" },
        );
    }
    else{
       head=format!(
            "HTTP/1.1 {} {}\r\nConnection: {}\r\n\r\n",
            r.status_code,
            r.reason,
            if r.keep_alive { "keep-alive" } else { "close" }
        ); 
    }

    stream.write_all(head.as_bytes())?;
    stream.write_all(&body)?;
    Ok(stream.flush()?)
}