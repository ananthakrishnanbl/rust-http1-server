mod file;
mod path;

use file::{content_type, read_file};
use path::{safe_path};
use crate::error::{Error,Result};

#[derive(Debug)]
pub struct StaticServe{
    status_code: u32,
    reason: String,
    content_type: Option<String>,
    content: Vec<u8>,
}

pub fn static_serve(target: &str) -> Result<StaticServe>{
    match safe_path(target,"./public"){
        Ok(path) =>{
            Ok(
                StaticServe{
                    status_code: 200,
                    reason: "Ok".into(),
                    content_type: content_type(&path),
                    content: read_file(&path)?,
                }
            )
        },
        Err(e)=>{
            println!("{}",e);
            Ok(
                StaticServe{
                    status_code:404,
                    reason: "Not Found".into(),
                    content_type: Some("text/html".into()),
                    content: "<h1>404 Not Found<h1>".into()
                }
            )
        }
    }
}

