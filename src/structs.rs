	

use rocket::serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OrganizationsListResponse{
    pub success: bool,
    pub result: Vec<Organization>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Organization{
    pub display_name: String,
    pub package_count: i32,
    pub title: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Departments{
    pub departments: Vec<Department>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Department{
    pub name: String,
    pub subordinates: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Subordinate{
    pub name: String,
}