use std::collections::HashMap;
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

mod error;
mod request;
mod static_file;

use static_file::{StaticServe, static_serve};
use error::{Result};
use request::{Request, read_request};

// fn write_response(
//     stream: &mut TcpStream,
//     status: u8,
//     reason: &str,
//     content_type: &str,
//     body: &[u8],
//     keep_alive: bool,
// ) -> std::io::Result<()>{
//     let head = format!(
//         "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: {}\r\n\r\n",
//         status,
//         reason,
//         content_type,
//         body.len(),
//         if keep_alive { "keep-alive" } else { "close" },
//     );
//     stream.write_all(head.as_bytes())?;
//     stream.write_all(body)?;
//     stream.flush()
// }


fn handle_stream(mut stream: TcpStream)->Result<()>{
    let mut reader = BufReader::new(&stream);

    let r: Request=read_request(&mut reader)?;
    println!("{:#?}",r);

    let s:StaticServe=static_serve(&r.target)?;
    println!("{:#?}",s);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("Listening on http://127.0.0.1:3000 \n");

    for stream in listener.incoming() {
        match stream{
            Ok(stream)=>{
                match handle_stream(stream){
                    Ok(_)=> println!("Incoming Request Sucessfully Handled"),
                    Err(e)=> println!("Error Handling Request {}",e),
                }
            },
            Err(e)=>{
                println!("ERROR {e}");
            }
        }
        println!("");
    }
    Ok(())
}