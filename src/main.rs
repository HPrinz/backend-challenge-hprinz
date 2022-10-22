#[macro_use] extern crate rocket;
#[cfg(test)] mod test;
mod structs;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Add;

use reqwest;
use url::Url;
use rocket::tokio;
use anyhow::Result;

#[get("/hello")]
fn hello() -> &'static str {
   "Hello World!"
}

#[get("/")]
fn organizations() -> String {
    let res = tokio::task::block_in_place(|| {
        return get_organization_list();
    });
    let mut file = File::open("departments.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    
    let departments = parse_departments(&data).departments;

    return process_organizations(res.unwrap().result, &departments).iter()
    .map(|(k, v)| format!("{}:{}", k, v))
    .collect::<Vec<String>>()
    .join(",\n");
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

/// Returns a HashMap with all root departments as keys and the sum of package count from department and their subordinates as values
///
/// # Arguments
///
/// * `organizations` - A list of all organizations retrieved by the ckan API with name and package_count attributes only 
/// * `departments` - The list of departments with their subordinates to sum the package count from
///
fn process_organizations(organizations: Vec<structs::Organization>, departments: &Vec<structs::Department>) -> HashMap<String, i32> {
    
    let mut all_organizations : HashMap<String, i32> = HashMap::new();
    for org in organizations{
        all_organizations.insert(org.title, org.package_count);
    }

    let mut final_organizations : HashMap<String, i32> = HashMap::new();
    for dep in departments{
        let final_name = &dep.name;

        if all_organizations.contains_key(final_name){
            final_organizations.insert(final_name.to_string(), all_organizations[final_name]);
            
            for dep_sub in dep.subordinates.iter().flatten() {
                let new_val = final_organizations[final_name].add(all_organizations[&dep_sub.name]);
                final_organizations.insert(final_name.to_string(), new_val);
            }
        } else {
            println!("all orgs does not contain {}", final_name);
        }
    }
    
    return final_organizations;
}

fn get_organization_list() -> Result<structs::OrganizationsListResponse>{
    
    let uri = Url::parse_with_params("https://www.govdata.de/ckan/api/3/action/organization_list", 
    &[("all_fields", "true")]);
    
    let result = reqwest::blocking::get(uri?)?;

    return Ok(result.json::<structs::OrganizationsListResponse>()?);
}
