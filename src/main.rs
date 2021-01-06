#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;

#[get("/start/<port>/<speed>")]
fn start(port: String, speed: u16) -> String {
    format!("Yaay, speed: {}, port: {}", speed, port)
}

#[get("/getOutput")]
fn get_output() -> &'static str {
    return "Here it is!!";
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static") )
        .mount("/api", routes![start, get_output])
        .launch();
}
