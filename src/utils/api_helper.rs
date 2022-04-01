use std::collections::HashMap;
use lambda_http::{
    http::{header::HeaderName, StatusCode},
    Response,
};

pub struct ApiHelper;

impl ApiHelper {
    pub fn response(status_code: StatusCode, body: String,  headers: HashMap<HeaderName, String>) -> Response<String> {
        let mut res = Response::builder().status(status_code);

        headers.iter().for_each(|(key, value)| {
            let headers = res.headers_mut().unwrap();
            headers.insert(key, value.parse().unwrap());
        });

        res.body(body).unwrap()
    }
}
