use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

enum RequestState {
    IDLE,
    STARTED,
    SEND,
}
// Logical State                  __state            __response
// -------------                  -------            ----------
// Idle                           _CS_IDLE           None
// Request-started                _CS_REQ_STARTED    None
// Request-sent                   _CS_REQ_SENT       None
// Unread-response                _CS_IDLE           <response_class>
// Req-started-unread-response    _CS_REQ_STARTED    <response_class>
// Req-sent-unread-response       _CS_REQ_SENT       <response_class>

struct HttpConnection {
    http_vsn: i32,
    http_vsn_str: String,
    host: String,
    port: usize,
    timeout: usize,
    source_address: String,
    blocksize: usize,
    state: RequestState,
    method: String,

    output: Vec<Vec<u8>>,

    socket: Option<TcpStream>,
    headers: HashMap<String, String>,
}

impl Default for HttpConnection {
    fn default() -> Self {
        HttpConnection {
            http_vsn: 11,
            http_vsn_str: "HTTP/1.1".to_string(),
            host: "0.0.0.0".to_string(),
            port: 0,
            timeout: 5,
            source_address: "localhost".to_string(),
            blocksize: 32,
            state: RequestState::IDLE,
            method: "GET".to_string(),
            output: Vec::new(),
            socket: None,
            headers: HashMap::new(),
        }
    }
}

impl HttpConnection {
    pub fn new(addr: &str) -> Option<Self> {
        // let (host, port) = addr.splitn(1, ":").collect();
        let mut new_http_connection: HttpConnection = HttpConnection {
            http_vsn: 11,
            http_vsn_str: "HTTP/1.1".to_string(),
            host: "0.0.0.0".to_string(),
            port: 0,
            timeout: 5,
            source_address: "localhost".to_string(),
            blocksize: 32,
            state: RequestState::IDLE,
            method: "GET".to_string(),
            output: Vec::new(),
            socket: None,
            headers: HashMap::new(),
        };

        match Self::get_hostport(addr) {
            Some((host, port)) => {
                new_http_connection.host = host;
                new_http_connection.port = port;
            }
            None => {
                // return None
            }
        }
        match TcpStream::connect(addr) {
            Ok(tcpstream) => new_http_connection.socket = Some(tcpstream),
            Err(_) => return None,
        }

        Some(new_http_connection)
    }

    pub fn get_hostport(addr: &str) -> Option<(String, usize)> {
        if let Some(i) = addr.find(':') {
            let port = &addr[i..];
            let host = &addr[0..i];
            let port: usize = match port.parse() {
                Ok(a) => a,
                Err(_) => return None,
            };
            return Some((host.to_string(), port));
        }
        None
    }

    pub fn request(
        &mut self,
        method: &str,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> HttpResponse {
        match Self::putrequest(self, method, url) {
            Some(request) => {
                self.output.push(request.as_bytes().to_vec());
            }
            None => {}
        }

        // http版本1.1 而且 指定分块编码 或者 以下条件是会发生分块编码
        // 1. 没有指定content-length
        // 2. body的类型是File或者iterator(不包括str, bytes等)
        // 3. 没有明确指定Transfer-Encoding
        let mut encode_chunked = false;
        if !self.headers.contains_key("content-length") {
            if !self.headers.contains_key("transfer-encoding") {
                match Self::get_content_length(body.clone(), method) {
                    Some(content_length) => {
                        encode_chunked = true;
                        self.headers
                            .insert("Content-Length".to_string(), "chunked".to_string());
                    }
                    None => {
                        self.headers
                            .insert("Transfer-Encoding".to_string(), "chunked".to_string());
                    }
                }
            }
        }

        for (hdr, value) in self.headers.clone() {
            Self::putheader(self, hdr, value);
        }

        Self::endheader(self, body, encode_chunked);

        HttpResponse {}
    }

    pub fn putheader(&mut self, header: String, value: String) {
        //
        match self.state {
            RequestState::STARTED => {
                // 需要规范编码规则
                let header = format!("{}{}{}{}", header, ": ", value, "\r\n\t");
                self.output.push(header.as_bytes().to_vec());
            }
            _ => {}
        }
    }

    pub fn endheader(&mut self, body: Option<String>, encode_chunked: bool) {
        match self.state {
            RequestState::STARTED => {
                self.state = RequestState::SEND;
            }
            _ => {}
        }
    }

    pub fn send_output(&mut self, message_body: &str, encode_chunked: bool) {
        // Send the currently buffered request and clear the buffer.
        self.output.push("".as_bytes().to_vec());
        self.output.push("".as_bytes().to_vec());

        let mut msg = Vec::new();
        let len = self.output.len();
        for row in self.output[0..(len - 2)].iter() {
            msg.append(&mut row.clone());
            msg.append(&mut "\r\n".as_bytes().to_vec());
        }
        msg.append(&mut self.output[len - 1]);
        self.send(msg);

        //
    }

    pub fn send(&mut self, data: Vec<u8>) {
        if let Some(mut socket) = self.socket.take() {
            for chunk in data.chunks(self.blocksize) {
                socket.write_all(chunk);
            }
        }
    }

    pub fn putrequest(&mut self, method: &str, url: &str) -> Option<String> {
        match self.state {
            RequestState::IDLE => self.state = RequestState::IDLE,
            _ => {}
        }

        if !Self::_validate_method(method) {
            return None;
        }

        let request = format!("{} {} {}", method, url, self.http_vsn_str);
        if self.http_vsn == 11 {
            // 可以设置一些默认的请求头
        }
        Some(request)
    }

    pub fn _validate_method(method: &str) -> bool {
        true
    }

    pub fn get_content_length(body: Option<String>, method: &str) -> Option<usize> {
        let METHOD_EXCEPTING_BODY: Vec<String> =
            vec!["PATCH".to_string(), "POST".to_string(), "PUT".to_string()];

        match body {
            Some(body) => Some(body.len()),
            None => {
                if METHOD_EXCEPTING_BODY.contains(&method.to_ascii_uppercase()) {
                    return Some(0);
                } else {
                    return None;
                }
            }
        }
    }

    pub fn getresponse(&mut self) {
        match self.state {
            RequestState::SEND => {
                match self.socket.take() {
                    Some(socket) => {
                        let mut buf = Vec::new();
                        let mut reader = BufReader::new(socket);
                        reader.read(&mut buf).unwrap();
                        println!("{}", String::from_utf8_lossy(&buf));
                    }
                    None => {
                        println!("self.socket = None");
                    }
                }
                //
            }
            _ => {
                println!("失去连接???")
            }
        }
    }
}

struct HttpResponse {}

#[cfg(test)]
mod test {
    use super::HttpConnection;

    #[test]
    fn a() {
        let mut a: HttpConnection = HttpConnection::new("localhost:3000").unwrap();
        a.request("GET", "/", None, None);
        let s = a.getresponse();
    }
}
