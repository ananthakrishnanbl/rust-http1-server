use std::collections::HashMap;
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

mod error;
mod request;

use error::{Result};
use request::{Request, read_request};

// fn get_file_name(target: &str) -> String{
//     match target{
//         "/" => String::from("public/index.html"),
//         other=> format!("public/{other}")
//     }
// }

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

// fn read_file(file_path: &str) -> std::io::Result<String>{
//     match fs::read_to_string(file_path){
//         Ok(content)=> Ok(content),
//         Err(_)=> {
//             match fs::read_to_string("public/404.html"){
//                 Ok(content)=> Ok(content),
//                 Err(e)=> Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e}"))),
//             }
//         },
//     }
// }

fn handle_stream(mut stream: TcpStream)->Result<()>{
    let mut reader = BufReader::new(&stream);

    let r: Request=read_request(&mut reader)?;
    println!("{:#?}",r);

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