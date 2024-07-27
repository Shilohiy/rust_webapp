use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

/// Converts a string slice into a `Method` enum variant.
///
/// # Arguments
///
/// * `s` - The string slice to convert.
///
/// # Returns
///
/// The corresponding `Method` enum variant.
///
/// # Examples
///
/// ```
/// use http::httprequest::Method;
///
/// let method: Method = "GET".into();
/// assert_eq!(method, Method::Get);
///
/// let method: Method = "POST".into();
/// assert_eq!(method, Method::Post);
///
/// let method: Method = "PUT".into();
/// assert_eq!(method, Method::Uninitialized);
/// ```
impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    // V3_0,
    Uninitialized,
}

/// Converts a string representation of a version into a `Version` enum.
///
/// # Arguments
///
/// * `s` - The string representation of the version.
///
/// # Returns
///
/// The corresponding `Version` enum value.
///
/// # Examples
///
/// ```
/// use http::httprequest::Version;
///
/// let version: Version = "HTTP/1.1".into();
/// assert_eq!(version, Version::V1_1);
///
/// let version: Version = "HTTP/2.0".into();
/// assert_eq!(version, Version::V2_0);
///
/// let version: Version = "HTTP/3.0".into();
/// assert_eq!(version, Version::Uninitialized);
/// ```
impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            // "HTTP/3.0" => Version::V3_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
    // Uninitialized,
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

/// Converts a `String` into an `HttpRequest` struct.
///
/// This implementation parses the provided `String` line by line to extract the HTTP method,
/// resource, version, headers, and message body. It then constructs and returns an `HttpRequest`
/// struct with the parsed values.
///
/// # Arguments
///
/// * `req` - The `String` representation of the HTTP request.
///
/// # Returns
///
/// An `HttpRequest` struct with the parsed values.
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = parse_request_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = parse_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                // Empty line indicates the end of the headers
                // Empty instructions, delivered to the operating system, are ignored.
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn parse_request_line(s: &str) -> (Method, Resource, Version) {
    // An iterator over the whitespace-separated words in the input string.
    // Example："GET /index.html HTTP/1.1" to be ["GET", "/index.html", "HTTP/1.1"]
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn parse_header_line(s: &str) -> (String, String) {
    // Represents the items in the header of an HTTP request.
    // Example: "Host: localhost:3000" to be ["Host", "localhost:3000"]
    // to slice in ":"
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.71.1".into());
        let req: HttpRequest = s.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
