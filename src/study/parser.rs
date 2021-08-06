use std::str::from_utf8;

pub enum ParseResult<T> {
    Complete(T),
    Partial,
    Error,
}

impl<T> ParseResult<T> {
    #[warn(dead_code)]
    fn is_complete(&self) -> bool {
        use self::ParseResult::*;
        match *self {
            Complete(_) => true,
            _ => false
        }
    }
    #[warn(dead_code)]
    fn is_partial(&self) -> bool {
        use self::ParseResult::*;
        match *self {
            Partial => true,
            _ => false
        }
    }
}

impl<T, E> From<Result<T, E>> for ParseResult<T> {
    fn from(r: Result<T, E>) -> Self {
        use self::ParseResult::*;
        match r {
            Ok(t) => Complete(t),
            Err(_) => Error
        }
    }
}

pub struct Request<'a>(pub &'a str);

pub fn parse(mut buf: &[u8]) -> ParseResult<Request> {
    use self::ParseResult::*;

    let get = b"GET";
    let end = b"\r\n";

    if !buf.starts_with(get) {
        return Error;
    }
    buf = &buf[get.len()..];
    if buf.ends_with(end) {
        buf = &buf[0..buf.len() - end.len()];
    } else {
        return Partial;
    }

    from_utf8(buf)
        .map(Request)
        .into()
}

#[test]
fn http09_get_success_root() {
    let _req = b"GET /\r\n";
    let res = parse(_req);
    assert!(res.is_complete());
}

#[test]
fn http09_get_partial_root() {
    let _req = b"GET /\r";
}

#[test]
#[should_panic]
fn http09_post_failure() {
    let _req = b"POST  / \r\n";
    let res = parse(_req);
    assert!(res.is_complete());
}
