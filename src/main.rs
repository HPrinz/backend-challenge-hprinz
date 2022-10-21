#[macro_use] extern crate rocket;
#[cfg(test)] mod test;
mod structs;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Add;

use reqwest::{self};
use url::Url;
use rocket::tokio;
use rocket::serde::json::Json;

#[get("/hello")]
fn hello() -> &'static str {
   "Hello World!"
}

#[get("/")]
fn organizations() -> String {
    let res = tokio::task::block_in_place(|| {
        let return_value = get_organization_list("true", "true");
        return_value
    });
    let mut file = File::open("departments.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    
    return filter_organizations(res.result, &parse_departments(&data).departments).iter()
    .map(|(k, v)| format!("{}:{}", k, v))
    .collect::<Vec<String>>()
    .join(",");

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, organizations])
}

fn parse_organizations(orgs_string: &str) -> structs::OrganizationsListResponse {
    rocket::serde::json::from_str(&orgs_string).expect("JSON was not well-formatted")
}

fn parse_departments(departments_string: &str) -> structs::Departments {
    rocket::serde::json::from_str(&departments_string).expect("JSON was not well-formatted")
}

fn filter_organizations(organizations: Vec<structs::Organization>, departments: &Vec<structs::Department>) -> HashMap<String, i32> {
    let mut organizations_tmp : HashMap<String, i32> = HashMap::new();

    
    for org in organizations{
        organizations_tmp.insert(org.title, org.package_count);
    }

    let mut final_organizations : HashMap<String, i32> = HashMap::new();

    for dep in departments{
        let final_name = &dep.name;
        if organizations_tmp.contains_key(final_name){
            final_organizations.insert(final_name.to_string(), organizations_tmp[final_name]);
            for dep_sub in dep.subordinates.iter().flatten() {
                let new_val = final_organizations[final_name].add(organizations_tmp[&dep_sub.name]);
                final_organizations.insert(final_name.to_string(), new_val);
            }
        }
    }
    
    return final_organizations;
}

// https://www.govdata.de/ckan/api/3/action/organization_list?all_fields=true&include_dataset_count=true&sort=package_count
fn get_organization_list(all_fields: &'static str, include_dataset_count: &'static str) -> structs::OrganizationsListResponse{
        
    let uri = Url::parse_with_params("https://www.govdata.de/ckan/api/3/action/organization_list", 
    &[("all_fields", all_fields), ("include_dataset_count", include_dataset_count)]);
    
    let result = reqwest::blocking::get(uri.unwrap()).unwrap();

    println!("{:#?}", result);

    return result.json::<structs::OrganizationsListResponse>().unwrap();
}
