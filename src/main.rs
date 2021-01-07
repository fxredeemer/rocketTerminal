#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate rand;

use rocket_contrib::serve::StaticFiles;

#[get("/start/<port>/<speed>")]
fn start(port: String, speed: u16) -> String {
    format!("Yaay, speed: {}, port: {}", speed, port)
}

#[get("/getOutput")]
fn get_output() -> String {
    let i: i32 = rand::random();
    return format!("Here it is!! {}", i);
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static") )
        .mount("/api", routes![start, get_output])
        .launch();
}
