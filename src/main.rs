#[macro_use]
extern crate nickel;

use nickel::Nickel;
use nickel::HttpRouter;

fn main() {
    let mut server = Nickel::new();

    server.get("**", middleware!("Ready To Roll!"));
    server.listen("127.0.0.1:3000");
}
