extern crate iron;
extern crate time;

use iron::prelude::*;

mod controller;

use crate::controller::ResponseTime;


fn main() {
    let mut chain = Chain::new(controller::hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000").unwrap();
}
