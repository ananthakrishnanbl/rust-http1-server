mod file;
mod path;

use file::{content_type, read_file};
use path::{safe_path};
use crate::error::{Error,Result};

#[derive(Debug)]
pub struct StaticServe{
    pub status_code: u32,
    pub reason: String,
    pub content_type: Option<String>,
    pub content: Option<Vec<u8>>,
}

pub fn static_serve(target: &str) -> StaticServe{
    let Ok(path)=safe_path(target,"./public") else{
        return StaticServe{
            status_code:404,
            reason: "Not Found".into(),
            content_type: Some("text/html".into()),
            content: Some("<h1>404 Not Found<h1>".into())
            }
    };

    let Some(c_type)=content_type(&path) else{
        return StaticServe{
            status_code:404,
            reason: "Not Found".into(),
            content_type: Some("text/html".into()),
            content: Some("<h1>404 Not Found<h1>".into())
            }
    };

    let Ok(content) = read_file(&path) else{
        return StaticServe{
            status_code:404,
            reason: "Not Found".into(),
            content_type: Some("text/html".into()),
            content: Some("<h1>404 Not Found<h1>".into())
            }
    };

    StaticServe{
        status_code: 200,
        reason: "Ok".into(),
        content_type: Some(c_type),
        content: Some(content),
    }
}

