use std::str;

#[derive(Debug, PartialEq)]
pub enum ParseResult<T> {
    Complete(T),
    Partial,
    Error,
}

impl<T> ParseResult<T> {
    fn is_complete(&self) -> bool {
        use self::ParseResult::*;

        match *self {
            Complete(_) => true,
            _ => false,
        }
    }

    fn is_partial(&self) -> bool {
        use self::ParseResult::*;

        match *self {
            Partial => true,
            _ => false,
        }
    }
}

impl<T, E> From<Result<T, E>> for ParseResult<T> {
    fn from(r: Result<T, E>) -> Self {
        use self::ParseResult::*;

        match r {
            Ok(t) => Complete(t),
            Err(_) => Error,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Request<'a>(&'a str);

pub fn parse(mut buf: &[u8]) -> ParseResult<Request> {
    use self::ParseResult::*;

    let get = b"GET";
    let end = b"\r\n";

    if !buf.starts_with(get) {
        return Error;
    }
    buf = &buf[get.len()..];

    if !buf.ends_with(end) {
        return Partial;
    }
    buf = &buf[0..(buf.len() - end.len())];

    return str::from_utf8(&buf).map(Request).into();
}

#[test]
fn parse_complete_get_request() {
    let result = parse(b"GET_complete_body_\r\n");
    assert_eq!(result, ParseResult::Complete(Request("_complete_body_")));
    assert!(result.is_complete());
}

#[test]
fn parse_partial_get_request() {
    let result = parse(b"GET_partial_body_");
    assert_eq!(result, ParseResult::Partial);
    assert!(result.is_partial());
}

#[test]
fn parse_post_request() {
    assert_eq!(parse(b"POST_not_get_method_\r\n"), ParseResult::Error);
}

#[test]
fn parse_invalid_format_request() {
    assert_eq!(parse(b"_missing_http_method_\r\n"), ParseResult::Error);
}
