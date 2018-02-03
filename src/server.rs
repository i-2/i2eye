//! This module consists for stuct the serves
//! the text from the image url.

use std::collections::HashMap;
use futures::future::{ok, FutureResult};
use hyper::Error;
use hyper::header::ContentLength;
use hyper::StatusCode;
use hyper::server::{Http, Request, Response, Service};
use url::{form_urlencoded, Url};
use ocr::ImageBuilder;

pub struct ImageService;

impl Service for ImageService {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = FutureResult<Self::Response,  Self::Error>;

    fn call(&self, _req: Request) -> Self::Future {
        //get the url q parameter which is the image url.
        let qs = _req.query().unwrap_or("").to_owned();
        debug!("QRY!!! {}", qs);
        info!("{:?}", _req);
        let map: HashMap<String, String> = form_urlencoded::parse(qs.as_bytes()).into_owned().collect();
        ok(match map.get("q") {
            Some(s) => {
                if s.len() > 0 && Url::parse(s).is_ok() {
                    debug!("Trying to read url {}", s);
                    let mut builder = ImageBuilder::from_url(s);
                    let possible_text: Option<String> = builder.reader().text();
                    debug!("{:?}", possible_text);
                    Response::new().with_body(possible_text.unwrap())
                } else {
                    debug!("Query String empty!!");
                    Response::new().with_status(StatusCode::NotFound)
                }
            },
            None => Response::new().with_status(StatusCode::NotFound)
        })
    }
}