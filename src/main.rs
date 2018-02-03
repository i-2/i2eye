#[macro_use]
extern crate log;
extern crate env_logger;


extern crate reqwest;
extern crate hyper;
extern crate time;
extern crate futures;
extern crate url;
extern crate clap;
use clap::{Arg, App}; 

mod ocr;
mod server;

static VERSION: &str = "0.5.0";

fn main() {
   env_logger::init();
//    let addr = "0.0.0.0:3890".parse().unwrap();
//    let server = hyper::server::Http::new().bind(&addr, || Ok(server::ImageService)).unwrap();
//    server.run().unwrap();
   let application = App::new("i2eye")
                          .version(VERSION)
                          .author("sourcepirate")
                          .arg(Arg::with_name("address")
                               .short("a")
                               .value_name("HOST:PORT")
                               .required(true)
                               .takes_value(true)).get_matches();
    let config = application.value_of("address").unwrap_or("0.0.0.0:3890");
    info!("Running server on {:?}", config);
    let address = config.parse().unwrap();
    let server = hyper::server::Http::new().bind(&address, || Ok(server::ImageService)).unwrap();
    server.run().unwrap();
}
