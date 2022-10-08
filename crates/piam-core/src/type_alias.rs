use hyper::Body;

pub type HttpRequest = http::Request<Body>;
pub type HttpResponse = http::Response<Body>;

pub enum ApplyResult {
    Forward(HttpRequest),
    Reject(HttpResponse),
}
