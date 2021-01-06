#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
    return "Hellooo";
}

#[get("/screen/<port>/<speed>")]
fn screen(port: String, speed: u16) -> String {
    format!("Yaay, speed: {}, port: {}", speed, port)
}

fn main() {
    rocket::ignite().mount("/", routes![index, screen]).launch();
}
