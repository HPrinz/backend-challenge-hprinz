use super::rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_hello() {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello World!".into()));
}
