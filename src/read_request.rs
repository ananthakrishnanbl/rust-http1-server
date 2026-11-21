use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpStream};

pub fn read_headers(reader: &mut BufReader<&TcpStream>) -> std::io::Result<HashMap<String,String>>{
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

pub fn read_request_line(reader: &mut BufReader<&TcpStream>) -> std::io::Result<(String, String, String)>{
    let mut line=String::new();
    let _=reader.read_line(&mut line);
    let line=line.trim();

    let mut iter=line.split(' ');
    match (iter.next(), iter.next(), iter.next(), iter.next()){
        (Some(m), Some(t), Some(v), None) => Ok((m.to_string(), t.to_string(), v.to_string())),
        _=> Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "bad request line")),
    }
}

pub fn read_body(
    reader: &mut BufReader<&TcpStream>,
    len: usize,
)->std::io::Result<String>{
    let mut body=vec![0;len];
    let _ = reader.read_exact(&mut body)?;
    let body=String::from_utf8_lossy(&body).to_string();
    Ok(body)
}