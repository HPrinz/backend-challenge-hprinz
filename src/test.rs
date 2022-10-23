use crate::structs::OrganizationsListResponse;
use crate::{process_organizations, parse_departments, sort_ministries_by_count};
use super::rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;
use std::collections::HashMap;

#[test]
fn test_organizations() {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_parse_organizations() {

    let organization_list_string = r#"{
        "success": true,
        "other_attribute": "should be ignored",
        "result": [{
            "package_count": 7,
            "display_name": "Amt für X"
        },
        {
            "package_count": 273846,
            "display_name": "Amt für Y"
        }]
    }"#;

    let parsed_organizations : OrganizationsListResponse = rocket::serde::json::from_str(organization_list_string).unwrap();
    assert_eq!(parsed_organizations.success, true);
}


#[test]
fn test_parse_departments() {
    let departmnts_list_string = r#"{
        "departments": [{
            "name": "Amt für X",
            "subordinates": [{
                "name": "Amt für Y"
            }]
        }]
    }"#;
    
    let departments = parse_departments(departmnts_list_string);
    assert_eq!(departments.departments[0].name, "Amt für X");
    assert!(departments.departments[0].subordinates.is_some());
}

#[test]
fn test_process_organization_with_subordinate() {
    let parsed_organizations : OrganizationsListResponse = rocket::serde::json::from_str(r#"{
        "success": true,
        "result": [{
            "package_count": 5,
            "display_name": "Amt für X"
        },
        {
            "package_count": 1,
            "display_name": "Amt für Y"
        }
        ]
    }"#).unwrap();

    let departments = parse_departments(r#"{
        "departments": [{
            "name": "Amt für X",
            "subordinates": [{
                "name": "Amt für Y"
            }]
        }]
    }"#).departments;
    
    let organizations_filtered = process_organizations(parsed_organizations.result, &departments);
    assert_eq!(organizations_filtered["Amt für X"], 6);
}



#[test]
fn test_process_organization_with_only_subordinate() {
    let parsed_organizations : OrganizationsListResponse = rocket::serde::json::from_str(r#"{
        "success": true,
        "result": [{
            "package_count": 3,
            "display_name": "subordinate with organization"
        }
        ]
    }"#).unwrap();

    let departments = parse_departments(r#"{
        "departments": [{
            "name": "department without organization",
            "subordinates": [{
                "name": "subordinate with organization"
            }]
        }]
    }"#).departments;
    
    let organizations_filtered = process_organizations(parsed_organizations.result, &departments);
    assert_eq!(organizations_filtered["department without organization"], 3);
}

#[test]
fn test_process_organization_with_missing_subordinate() {
    let parsed_organizations : OrganizationsListResponse = rocket::serde::json::from_str(r#"{
        "success": true,
        "result": [{
            "package_count": 3,
            "display_name": "subordinate with organization"
        }]
    }"#).unwrap();

    let departments = parse_departments(r#"{
        "departments": [{
            "name": "department without organization",
            "subordinates": [{
                "name": "subordinate with organization"
            },{
                "name": "subordinate without organization"
            }]
        }]
    }"#).departments;
    
    let organizations_filtered = process_organizations(parsed_organizations.result, &departments);
    assert_eq!(organizations_filtered["department without organization"], 3);
}

#[test]
fn test_sort_departments() {
    let mut departments = HashMap::new();
    departments.insert("Department 1".to_string(), 1);
    departments.insert("Department 2".to_string(), 3);
    departments.insert("Department 3".to_string(), 2);
    
    let sorted = sort_ministries_by_count(departments);
    assert_eq!(sorted[0], ("Department 2".to_string(), 3));
    assert_eq!(sorted[1], ("Department 3".to_string(), 2));
    assert_eq!(sorted[2], ("Department 1".to_string(), 1));
}