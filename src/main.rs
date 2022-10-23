#[macro_use] extern crate rocket;
#[cfg(test)] mod test;
mod structs;

use structs::{Department, OrganizationsListResponse, Departments, Organization};

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, ErrorKind, Error};
use std::ops::Add;

use reqwest;
use rocket::log::private::warn;
use url::Url;
use rocket::tokio;
use anyhow::{Result};

use rocket_dyn_templates::{Template, context};

/// Returns a HTML with all federal ministries and their number of data sets
#[get("/")]
fn departments_dashboard() -> Template {
    // retrieve organizations from GovData Api
    let url = env::var("ORGANIZATIONS_URL").expect("Env var ORGANIZATIONS_URL is missing");
    let res = tokio::task::block_in_place(|| {
        return get_organization_list(&url).expect("Error while retrieving organizations");
    });
 
    // read local departments.json file, containig the federal ministries and their subordinates
    let mut file = File::open("departments.json").expect("File departments.json is missing");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("departments.json file was not valid UTF-8");
    let departments = parse_departments(&data).departments;

    // process the organizations to get the sum of data sets by federal ministry
    let ministries_with_count = process_organizations(res, &departments);

    let ministries_with_count_sorted = sort_ministries_by_count(ministries_with_count.clone());

    Template::render("index", context! { departments: ministries_with_count_sorted })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![departments_dashboard])
    .attach(Template::fairing())
}

fn parse_departments(departments_string: &str) -> Departments {
    rocket::serde::json::from_str(&departments_string).expect("JSON was not well-formatted")
}

/// Returns a HashMap with all root ministries as keys and the sum of uploaded data packages from department and their subordinates as values
///
/// # Arguments
///
/// * `organizations` - A list of all organizations retrieved by the GovData API with name and package count only 
/// * `departments` - The list of ministries with their subordinates to sum the package count from
///
fn process_organizations(organizations: Vec<Organization>, departments: &Vec<Department>) -> HashMap<String, i32> {
    
    // create temporal HashMap with all organizations and their count
    let mut all_organizations : HashMap<String, i32> = HashMap::new();
    for org in organizations{
        all_organizations.insert(org.display_name, org.package_count);
    }

    let mut ministries : HashMap<String, i32> = HashMap::new();
    for dep in departments{
        let final_name = &dep.name;
        // build a HashMap with only ministries and initialize with 0
        ministries.insert(final_name.to_string(), 0);

        // add the count of the ministry if it's in the list of organizations
        if all_organizations.contains_key(final_name) {
            ministries.insert(final_name.to_string(), all_organizations[final_name]);
        } else {
            info!("all_organizations does not contain {}, will use data from subordinates only",final_name)
        }
        
        // add the count for all subordinates if they are in the list of organizations
        for dep_sub in dep.subordinates.iter().flatten() {
            if all_organizations.contains_key(&dep_sub.name) {
                let new_val = ministries[final_name].add(all_organizations[&dep_sub.name]);
                ministries.insert(final_name.to_string(), new_val);
            } else {
                warn!("all_organizations is missing subordinate {}", &dep_sub.name)
            }
        }
    }
    
    return ministries;
}

/// Fetches all organizations and their packagae count from the GovData API
fn get_organization_list(url : &str) -> Result<Vec<Organization>>{
    let uri = Url::parse_with_params(url, &[("all_fields", "true")]);
    let result = reqwest::blocking::get(uri?)?;
    let parsed_api_result = result.json::<OrganizationsListResponse>()?;
    if !parsed_api_result.success {
        return Err(Error::new(ErrorKind::Other, "Retrieving organizations from URL was not successful").into());
    }
    return Ok(parsed_api_result.result);
}

/// Returns a Vector with ministry and count tuples, sorted by count 
///
/// # Arguments
///
/// * `hashmap` - A HashMap of ministries and their package count
///
fn sort_ministries_by_count(hashmap: HashMap<String, i32>) -> Vec<(String, i32)>{
    let mut list_vector : Vec<(String, i32)> = hashmap.into_iter().map(|(x, y)|( x, y )).collect();
    list_vector.sort_by(|a, b| b.1.cmp(&a.1));
    return list_vector;
}