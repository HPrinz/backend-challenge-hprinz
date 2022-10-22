use crate::{parse_organizations, process_organizations, parse_departments};
use super::rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_hello() {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/hello").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello World!".into()));
}

#[test]
fn test_parse_organizations() {

    let organization_list_string = r#"{
        "success": true,
        "other_attribute": "should be ignored",
        "result": [{
            "package_count": 7,
            "title": "Amt für X"
        },
        {
            "package_count": 273846,
            "title": "Amt für Y"
        }]
    }"#;

    let organizations_response = parse_organizations(organization_list_string);
    assert_eq!(organizations_response.success, true);
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
    let organizations = parse_organizations(r#"{
        "success": true,
        "result": [{
            "package_count": 5,
            "title": "Amt für X"
        },
        {
            "package_count": 1,
            "title": "Amt für Y"
        }
        ]
    }"#).result;

    let departments = parse_departments(r#"{
        "departments": [{
            "name": "Amt für X",
            "subordinates": [{
                "name": "Amt für Y"
            }]
        }]
    }"#).departments;
    
    let organizations_filtered = process_organizations(organizations, &departments);
    assert_eq!(organizations_filtered["Amt für X"], 6);
}



#[test]
fn test_process_organization_with_only_subordinate() {
    let organizations = parse_organizations(r#"{
        "success": true,
        "result": [{
            "package_count": 3,
            "title": "subordinate with organization"
        }
        ]
    }"#).result;

    let departments = parse_departments(r#"{
        "departments": [{
            "name": "department without organization",
            "subordinates": [{
                "name": "subordinate with organization"
            }]
        }]
    }"#).departments;
    
    let organizations_filtered = process_organizations(organizations, &departments);
    assert_eq!(organizations_filtered["department without organization"], 3);
}

#[test]
fn test_organizations() {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
