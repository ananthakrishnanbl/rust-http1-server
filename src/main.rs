use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

fn read_headers(reader: &mut BufReader<&TcpStream>) -> std::io::Result<HashMap<String,String>>{
    let mut lines: Vec<String>=Vec::new();
    let mut line=String::new();
    
    loop{
        line.clear();
        let n=reader.read_line(&mut line)?;
        if n==0 || line=="\r\n"{
            break;
        }
        let trimmed=line.trim();
        lines.push(trimmed.to_string());
    }

    let mut headers = HashMap::new(); 

    for line in lines{
        match line.split_once(":"){
            Some((key,value))=> headers.insert(key.to_lowercase().to_string(), value.trim().to_string()),
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "bad header line")),
        };
    }

    Ok(headers)
}

fn read_request_line(reader: &mut BufReader<&TcpStream>) -> std::io::Result<(String, String, String)>{
    let mut line=String::new();
    let _=reader.read_line(&mut line);
    let line=line.trim();

    let mut iter=line.split(' ');
    match (iter.next(), iter.next(), iter.next(), iter.next()){
        (Some(m), Some(t), Some(v), None) => Ok((m.to_string(), t.to_string(), v.to_string())),
        _=> Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "bad request line")),
    }
}

fn read_body(
    reader: &mut BufReader<&TcpStream>,
    len: usize,
)->std::io::Result<String>{
    let mut body=vec![0;len];
    let _ = reader.read_exact(&mut body)?;
    let body=String::from_utf8_lossy(&body).to_string();
    Ok(body)
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
        Err(e)=> Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e}"))),
    }
}

fn handle_stream(mut stream: TcpStream)->std::io::Result<()>{
    let mut reader = BufReader::new(&stream);

    let (method,target,version)=read_request_line(&mut reader)?;
    println!("Method : {method}\nTarget : {target}\nVersion : {version}");

    let headers=read_headers(&mut reader)?;
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

    if target=="/" && method=="GET" {
        let Content=read_file("public/index.html")?;
        let _ =write_response(&mut stream,200,"OK", "text/html", Content.as_bytes(), false)?;
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