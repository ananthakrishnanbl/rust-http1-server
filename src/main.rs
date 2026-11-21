use std::collections::HashMap;
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
mod read_request;
use read_request::{read_body,read_headers,read_request_line};

fn get_file_name(target: &str) -> String{
    match target{
        "/" => String::from("public/index.html"),
        other=> format!("public/{other}")
    }
}

fn write_response(
    stream: &mut TcpStream,
    status: u8,
    reason: &str,
    content_type: &str,
    body: &[u8],
    keep_alive: bool,
) -> std::io::Result<()>{
    let head = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: {}\r\n\r\n",
        status,
        reason,
        content_type,
        body.len(),
        if keep_alive { "keep-alive" } else { "close" },
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}

fn read_file(file_path: &str) -> std::io::Result<String>{
    match fs::read_to_string(file_path){
        Ok(content)=> Ok(content),
        Err(_)=> {
            match fs::read_to_string("public/404.html"){
                Ok(content)=> Ok(content),
                Err(e)=> Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e}"))),
            }
        },
    }
}

fn handle_stream(mut stream: TcpStream)->std::io::Result<()>{
    let mut reader = BufReader::new(&stream);

    let (method,target,version)=read_request_line(&mut reader)?;
    println!("Method : {method}\nTarget : {target}\nVersion : {version}");

    let headers: HashMap<String,String>=read_headers(&mut reader)?;
    for (key,value) in &headers{
        println!("{key} : {value}");
    }
    let mut body=String::new();

    if headers.contains_key("transfer-encoding"){
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "chunked not supported"));
    }

    if let Some(v) = headers.get("content-length"){
        let len=v.parse::<usize>()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "bad content-length"))?;
        body=read_body(&mut reader,len)?;
    }
    println!("{body}");

    if method=="GET" {
        let file=get_file_name(&target);
        let content=read_file(&file)?;
        let _ =write_response(&mut stream,200,"OK", "text/html", content.as_bytes(), false)?;
    }

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