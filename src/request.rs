use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpStream};
use crate::error::{Error,Result};

#[derive(Debug)]
pub struct Request{
    method: String,
    target: String,
    version: String,
    headers: HashMap<String,String>,
    body: String,
}

fn read_headers(reader: &mut BufReader<&TcpStream>) -> Result<HashMap<String,String>>{
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
            None => return Err(Error::RequestError("Bad Header Lines".into())),
        };
    }

    Ok(headers)
}

fn read_request_line(reader: &mut BufReader<&TcpStream>) -> Result<(String, String, String)>{
    let mut line=String::new();
    let _=reader.read_line(&mut line);
    let line=line.trim();

    let mut iter=line.split(' ');
    match (iter.next(), iter.next(), iter.next(), iter.next()){
        (Some(m), Some(t), Some(v), None) => Ok((m.to_string(), t.to_string(), v.to_string())),
        _=> Err(Error::RequestError("Request Line is not Proper".into()))
    }
}

fn read_body(
    reader: &mut BufReader<&TcpStream>,
    len: usize,
)->Result<String>{
    let mut body=vec![0;len];
    let _ = reader.read_exact(&mut body)?;
    let body=String::from_utf8_lossy(&body).to_string();
    Ok(body)
}

pub fn read_request(
    reader: &mut BufReader<&TcpStream>,
)->Result<Request>{

    let (method,target,version)=read_request_line(reader)?;
    let headers=read_headers(reader)?;
    let mut body=String::new();
    if let Some(v) = headers.get("content-length"){
        let len=v.parse::<usize>()
        .map_err(|_| Error::RequestError("Bad Content Length".into()))?;
        body=read_body(reader,len)?;
    }

    Ok(Request{
        method,
        target,
        version,
        headers,
        body
    })
}