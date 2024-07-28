use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

/// A trait representing a handler for HTTP requests.
pub trait Handler {
    /// Handles an HTTP request and returns an HTTP response.
    ///
    /// # Arguments
    ///
    /// * `req` - The HTTP request to handle.
    ///
    /// # Returns
    ///
    /// An `HttpResponse` representing the response to the request.
    fn handle(req: &HttpRequest) -> HttpResponse;

    /// Loads the contents of a file.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to load.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the contents of the file if it exists, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let file_contents = Handler::load_file("example.txt");
    /// match file_contents {
    ///     Some(contents) => println!("File contents: {}", contents),
    ///     None => println!("File not found"),
    /// }
    /// ```
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path.clone());
        contents.ok()
    }
}
pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

/// Implementation of the `Handler` trait for the `PageNotFoundHandler` struct.
impl Handler for PageNotFoundHandler {
    /// Handles the HTTP request and returns an HTTP response.
    ///
    /// # Arguments
    ///
    /// * `_req` - The HTTP request object.
    ///
    /// # Returns
    ///
    /// An `HttpResponse` object representing the response to the request.
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

/// Implementation of the `Handler` trait for the `StaticPageHandler` struct.
impl Handler for StaticPageHandler {
    /// Handles the incoming HTTP request and returns an HTTP response.
    ///
    /// # Arguments
    ///
    /// * `req` - The HTTP request to handle.
    ///
    /// # Returns
    ///
    /// An `HttpResponse` representing the response to the request.
    fn handle(req: &HttpRequest) -> HttpResponse {
        // Extract the path from the request resource
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split('/').collect();

        // Match the route to determine the appropriate response
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    println!("Serving file: {} with contents:\n{}", path, contents);
                    let mut map: HashMap<&str, &str> = HashMap::new();

                    // Set the appropriate Content-Type header based on the file extension
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "application/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }

                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}

/// Implements the `Handler` trait for the `WebServiceHandler` struct.
impl Handler for WebServiceHandler {
    /// Handles the incoming HTTP request and returns an HTTP response.
    ///
    /// # Arguments
    ///
    /// * `req` - A reference to the `HttpRequest` object representing the incoming request.
    ///
    /// # Returns
    ///
    /// An `HttpResponse` object representing the response to the request.
    fn handle(req: &HttpRequest) -> HttpResponse {
        // Extract the path from the request resource
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split('/').collect();

        // Check the route and generate the appropriate response
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                // Generate a JSON response with a 200 status code
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => {
                // Generate a 404 response with a custom HTML file
                HttpResponse::new("404", None, Self::load_file("404.html"))
            }
        }
    }
}
