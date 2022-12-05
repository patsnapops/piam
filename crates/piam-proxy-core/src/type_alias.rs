use hyper::{client::HttpConnector, Body};

pub type HttpRequest = http::Request<Body>;
pub type HttpResponse = http::Response<Body>;
pub type HttpClient = hyper::Client<HttpConnector, Body>;
pub type IamEntityIdType = String;
