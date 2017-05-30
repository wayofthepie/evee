use rocket_contrib::JSON;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Execution {
    repo: String,
    init: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExecutionCreated {
    id: Uuid,
    execution: Execution,
}

#[post("/execution", format = "application/json", data = "<execution>")]
pub fn execution(execution: JSON<Execution>) -> JSON<ExecutionCreated> {
    let created = ExecutionCreated{ execution: execution.into_inner(), id: Uuid::new_v4() };
    return JSON(created);
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use ::rocket;
    use uuid::Uuid;
    use rocket::testing::MockRequest;
    use rocket::http::ContentType;
    use rocket::http::Method::*;
    use rocket::http::{Status};
    use ::spectral::prelude::*;
    use serde_json;

    #[test]
    fn test_post_event_success_returns_created_event() {
        // Arrange
        let rocket = rocket::ignite().mount("/", routes![super::execution]);
        let execution = super::Execution { repo: "test".to_string(), init: "test".to_string() };
        let execution_json = serde_json::to_string(&execution).unwrap();
        let execution_str: &str = &execution_json.to_string();
        let mut req = MockRequest::new(Post, "/execution")
            .header(ContentType::JSON)
            .body(execution_json);

        // Act
        let mut response = req.dispatch_with(&rocket);
        let body_str = response.body().and_then(|b| b.into_string()).unwrap();
        let created: super::ExecutionCreated = serde_json::from_str(&body_str).unwrap();

        // Assert
        assert_eq!(response.status(), Status::Ok);
        assert_that(&body_str).contains(execution_str);
        assert_that(&Uuid::parse_str(&created.id.to_string()).unwrap()).is_equal_to(created.id);
    }
}
