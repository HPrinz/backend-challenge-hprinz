use crate::{parse_organizations, filter_organizations, parse_departments};
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
            "display_name": "Amt für X",
            "package_count": 7,
            "title": "Amt für X"
        },
        {
            "display_name": "Amt für Y",
            "package_count": 273846,
            "title": "Amt für Y"
        }]
    }"#;

    let organizations_response = parse_organizations(organization_list_string);
    assert_eq!(organizations_response.success, true);
}



#[test]
fn test_filter_organizations() {
    let organization_list_string = r#"{
        "success": true,
        "other_attribute": "should be ignored"
        "result": [{
            "display_name": "Amt für X",
            "package_count": 7,
            "title": "Amt für X"
        },
        {
            "display_name": "Amt für Y",
            "package_count": 273846,
            "title": "Amt für Y"
        }
        ]
    }"#;
    let departmnts_list_string = r#"{
        "departments": [
        {
            "name": "Amt für X",
            "subordinates": [
            {
                "name": "Amt für Y"
            }
            ]
        }"#;

    
    let organizations_response = parse_organizations(organization_list_string);
    let departments = parse_departments(departmnts_list_string);
    let organizations_filtered = filter_organizations(organizations_response.result, departments);
    assert_eq!(organizations_filtered[0].display_name, "Amt für X");
}


#[test]
fn test_organizations() {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
