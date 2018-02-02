#[macro_use]
extern crate serde_derive;
extern crate log;

extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate hyper;
extern crate env_logger;
extern crate tempfile;
extern crate time;

mod ocr;

fn main() {
   env_logger::init();
}
