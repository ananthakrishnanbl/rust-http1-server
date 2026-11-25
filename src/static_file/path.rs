use std::path::{Path,PathBuf};
use std::fs;
use crate::error::{Error,Result};

fn find_target_path(target: &str) -> Option<String>{
    let path_only = target.split(['?', '#']).next().unwrap_or("/");

    let bytes=path_only.as_bytes();
    let mut out: Vec<u8> = Vec::new();
    let mut i=0;
    let len=bytes.len();
    while i<len {
        if bytes[i]==b'%'{
            if(i+1>=len || i+2 >=len){
                return None;
            }
            else{
                match ((bytes[i + 1] as char).to_digit(16), (bytes[i + 2] as char).to_digit(16)) {
                    (Some(hi), Some(lo)) => out.push((hi * 16 + lo) as u8),
                    _ => return None,
                }
            }
            i+=3;
        }
        else{
            out.push(bytes[i]);
            i+=1;
        }
    }
    let out= match String::from_utf8(out){
        Ok(p)=>p,
        _=> return None,
    };

    if out.contains('\0') || out.contains('\\') {
        return None;
    }

    if out.split('/').any(|seg| seg == "..") {
        return None;
    }

    Some(out)
}

pub fn safe_path(target: &str, root: &str) -> Result<String> {
    let target = if let Some(p) = find_target_path(target) {
        if p == "/" { format!("{root}/index.html") }
        else { format!("{root}{p}") }
    } else {
        return Err(Error::RequestError("Corrupted Target".into()));
    };

    let canonical_root = fs::canonicalize(root).map_err(|_| Error::Other("Static Path Not Found".into()))?;
    let canonical = fs::canonicalize(&target).map_err(|_| Error::Other("Path Not Found".into()))?;

    if !canonical.starts_with(&canonical_root) {
        return Err(Error::Forbidden);
    }

    if !canonical.is_file() {
        return Err(Error::Other("Path Not Found".into()));
    }

    Ok(canonical.to_string_lossy().into_owned())
}