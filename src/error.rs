pub enum Error{
    Io(std::io::Error),
    RequestError(String),
    Other(String),
    Forbidden
}

impl std::fmt::Display for Error{
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match self{
            Error::Io(e)=> write!(f,"IO Error {}",e),
            Error::RequestError(e)=>write!(f,"Error in Request: {}",e),
            Error::Other(e)=>write!(f,"{}",e),
            Error::Forbidden=>write!(f,"Forbidden"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T,Error>;