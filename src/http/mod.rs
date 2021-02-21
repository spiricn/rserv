use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

trait Stream {
    fn write(&mut self, buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>>;

    fn write_str(&mut self, string: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.write(string.as_bytes())
    }

    fn writeln(&mut self, string: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.write_str(string)?;
        self.new_line()
    }

    fn new_line(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.write_str(CRLF)
    }
}

const CRLF: &'static str = "\r\n";

///
/// rfc2616 codes
enum Code {
    Ok,
    NotFound,
}

impl Code {
    ///
    /// rfc2616 reason phrase
    fn reason_phrase(&self) -> &'static str {
        match *self {
            Code::Ok => "OK",
            Code::NotFound => "Not Found",
        }
    }

    ///
    /// rfc2616 integer value
    fn value(&self) -> i32 {
        match *self {
            Code::Ok => 200,
            Code::NotFound => 404,
        }
    }
}

enum Method {
    Get,
    Post,
}

impl Method {
    fn value(&self) -> &'static str {
        match &self {
            Method::Get => "Get",
            Method::Post => "Post",
        }
    }
}

struct Base {
    pub headers: HashMap<String, String>,
    pub version: Option<String>,
}

impl Default for Base {
    fn default() -> Base {
        Base {
            headers: HashMap::new(),
            version: None,
        }
    }
}

struct Request {
    pub base: Base,
    pub method: Option<Method>,
    pub path: String,
}

struct Response {
    pub base: Base,
    pub code: Option<Code>,
}

impl Default for Response {
    fn default() -> Response {
        Response {
            base: Base::default(),
            code: None,
        }
    }
}

impl Response {
    pub fn write_header(
        self,
        mut stream: Box<dyn Stream>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Response status
        stream.writeln(&format!(
            "HTTP/{} {} {}",
            self.base.version.unwrap(),
            self.code.as_ref().unwrap().value(),
            self.code.as_ref().unwrap().reason_phrase()
        ))?;

        // Headers
        for (key, value) in self.base.headers {
            stream.writeln(&format!("{}: {}", key, value))?;
        }

        // Body split
        stream.new_line()?;

        Ok(())
    }
}

struct StringStream {
    pub string: Rc<RefCell<String>>,
}

impl Stream for StringStream {
    fn write(&mut self, buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.string
            .borrow_mut()
            .push_str(&String::from_utf8_lossy(buffer));

        Ok(())
    }
}

mod test {
    use super::*;

    #[test]
    fn generate() {
        let str = Rc::new(RefCell::new(String::new()));

        let ss = Box::new(StringStream {
            string: Rc::clone(&str),
        });

        let mut response = Response {
            ..Response::default()
        };

        response.code = Some(Code::Ok);
        response.base.version = Some(String::from("1.1"));

        response
            .base
            .headers
            .insert(String::from("test"), String::from("header"));

        response.write_header(ss);

        println!("{}", str.borrow());
    }
}
