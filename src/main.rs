extern crate iron;
extern crate mount;
extern crate staticfile;

use std::env;
use std::path::Path;

use std::str::FromStr;
use iron::prelude::*;
use mount::Mount;
use staticfile::Static;


/// Look up our server port number in PORT, for compatibility with Heroku.
fn server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(3000)
}

fn main() {
    let mut mount = Mount::new();
    // Serve the shared JS/CSS at /
    mount.mount("/", Static::new(Path::new("static/")));
    let port = server_port();
    println!("Running on: http://localhost:{}", port);
    Iron::new(mount).http(("0.0.0.0", port)).unwrap();
}
