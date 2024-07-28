use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

/// Routes the incoming HTTP request to the appropriate handler based on the request method and resource path.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request.
/// * `stream` - A mutable reference to the stream to write the response to.
///
/// # Examples
///
/// ```
/// use httprequest::HttpRequest;
/// use std::io::Write;
///
/// let req = HttpRequest::new();
/// let mut stream = Vec::new();
/// Router::route(req, &mut stream);
/// ```
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
