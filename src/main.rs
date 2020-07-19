#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use rocket::Request;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("404: '{}' is not a valid path.", req.uri())
}

fn main() {
    rocket::ignite().mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
                    .register(catchers![not_found])
                    .launch();
}
