#[macro_use] extern crate rocket;
#[cfg(test)] mod test;
mod structs;

use reqwest::{self};
use url::Url;
use rocket::tokio;
use rocket::serde::json::Json;

#[get("/hello")]
fn hello() -> &'static str {
   "Hello World!"
}

#[get("/")]
fn organizations() -> Json<structs::OrganizationsListResponse> {
    let res = tokio::task::block_in_place(|| {
        let return_value = get_organization_list("true", "true");
        return_value
    });
    return Json(res);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, organizations])
}

fn parse_organizations(orgs_string: &str) -> structs::OrganizationsListResponse {
    rocket::serde::json::from_str(&orgs_string).expect("JSON was not well-formatted")
}

fn parse_departments(departments_string: &str) -> structs::Departments {
    panic!("todo");
}

fn filter_organizations(organizations: Vec<structs::Organization>, departments: structs::Departments) -> Vec<structs::Organization> {
    panic!("todo");
}

// https://www.govdata.de/ckan/api/3/action/organization_list?all_fields=true&include_dataset_count=true&sort=package_count
fn get_organization_list(all_fields: &'static str, include_dataset_count: &'static str) -> structs::OrganizationsListResponse{
        
    let uri = Url::parse_with_params("https://www.govdata.de/ckan/api/3/action/organization_list", 
    &[("all_fields", all_fields), ("include_dataset_count", include_dataset_count)]);
    
    let result = reqwest::blocking::get(uri.unwrap()).unwrap();

    println!("{:#?}", result);

    return result.json::<structs::OrganizationsListResponse>().unwrap();
}
