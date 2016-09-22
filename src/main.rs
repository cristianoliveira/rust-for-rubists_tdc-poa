extern crate iron;
extern crate mount;
extern crate staticfile;

use std::io;
use std::fs;

use iron::prelude::*;
use iron::status;
use iron::method;
use mount::Mount;
use staticfile::Static;
use std::path::Path;


fn main() {
    let mut mount = Mount::new();
    // Serve the shared JS/CSS at /
    mount.mount("/", Static::new(Path::new("static/")));
    println!("Running on: http://localhost:3000");
    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
