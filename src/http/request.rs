use crate::http::request;

use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::fmt::{Display,Debug};
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str;
use std::str::Utf8Error;
use super::{QueryString};
#[derive(Debug)]
pub struct Request<'buff> {
    path: &'buff str,
    query_string: Option<QueryString<'buff>>,
    method: Method,
}
impl<'buf> Request<'buf>{
    pub fn path(&self)->&str{
        &self.path
    }
    pub fn method(&self)->&Method{
        &self.method
    }
    pub fn query_string(&self)->Option<&QueryString>{
        self.query_string.as_ref()
        
    }
}
//impl Request{
//    fn from_byte_array(buf:&[u8])-> Result<Self,String>{}
//}
//GET /search?name=abc&sort=1 HTTP/1/1
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        //  match str::from_utf8(buf){
        //      Ok(request)=>{}
        //      Err(_)=>return Err(ParseError::InvalidEncoding),
        //  }
        let request = str::from_utf8(buf)?;
        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }
        // match path.find('?'){
        //     Some(i)=>{
        //         query_string=Some(&path[i+1..]);
        //        path=&path[..i] ;

        //     },
        //     None=>{},
        // }
        // let q=path.find('?');
        // if q.is_some(){
        //     let i=q.unwrap();
        //     query_string==Some(&path[i+1..]);
        //    path=&path[..i] ;

        // }
        Ok(Self {
            path: path,
            query_string: query_string,
            method: method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let chars = request.chars();
    for (i, c) in chars.enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
    //let mut iter=    request.chars();
    //
    //loop{
    //    let item =iter.next();
    //    match item{
    //        Some(c)=>{},
    //        None=>break,
    //    }
    //}
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
