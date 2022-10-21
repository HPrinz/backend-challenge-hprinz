#[macro_use] extern crate rocket;
#[cfg(test)] mod test;

#[get("/")]
fn index() -> &'static str {
   "Hello World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
