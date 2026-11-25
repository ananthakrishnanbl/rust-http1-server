use std::fs;
use std::path::{Path};
use crate::error::{Error, Result};

pub fn content_type(file_path: &str) -> Option<String> {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase());

    let ct = match ext.as_deref() {
        Some("html") | Some("htm") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") | Some("mjs") => "text/javascript; charset=utf-8",
        Some("txt") => "text/plain; charset=utf-8",
        Some("csv") => "text/csv; charset=utf-8",
        Some("md") => "text/markdown; charset=utf-8",

        Some("json") => "application/json",
        Some("xml") => "application/xml",
        Some("pdf") => "application/pdf",
        Some("wasm") => "application/wasm",
        Some("zip") => "application/zip",
        Some("gz") => "application/gzip",

        // images
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",

        // fonts
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",

        // audio / video
        Some("mp3") => "audio/mpeg",
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",

        // unknown or no extension
        _ => return None,
    };

    Some(ct.to_owned())
}

pub fn read_file(file_path: &str) -> Result<Vec<u8>>{
    match fs::read(file_path){
        Ok(content)=> Ok(content),
        Err(_)=> Err(Error::Other("File Not Found".into())),
    }
}