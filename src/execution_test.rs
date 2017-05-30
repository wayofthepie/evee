#[cfg(test)]
mod tests {
    pub use execution::{Execution, ExecutionCreated};
    use execution::static_rocket_route_info_for_execution;
    use execution::static_rocket_route_info_for_get_execution;
    use ::rocket;
    use rocket::testing::MockRequest;
    use rocket::http::ContentType;
    use rocket::http::Method::*;
    use rocket::http::{Status};
    use ::spectral::prelude::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use serde_json;
    use uuid::Uuid;

    describe! execution {
        before_each {
            let init_exec_id = Uuid::new_v4();
            let init_exec = Execution { repo: "test".to_string(), init: "test".to_string() };
            let init_exec_json = serde_json::to_string(&init_exec).unwrap();
            let init_exec_str: &str = &init_exec_json.to_string();
            let mut executions: HashMap<Uuid, Execution> = HashMap::new();
            executions.insert(init_exec_id, init_exec);
            let ref_execs = Arc::new(Mutex::new(executions));
            let rocket = rocket::ignite().mount("/", routes![execution, get_execution])
                .manage(ref_execs);
        }

        it "should return the created execution on success" {
            // Arrange
            let mut req = MockRequest::new(Post, "/execution")
                .header(ContentType::JSON)
                .body(init_exec_json);

            // Act
            let mut response = req.dispatch_with(&rocket);
            let body_str = response.body().and_then(|b| b.into_string()).unwrap();
            let created: ExecutionCreated = serde_json::from_str(&body_str).unwrap();

            // Assert
            assert_eq!(response.status(), Status::Ok);
            assert_that(&body_str).contains(init_exec_str);
            assert_that(&Uuid::parse_str(&created.id.to_string()).unwrap()).is_equal_to(created.id);
        }

        it "should return the execution at the given id" {
            // Arrange
            let mut req = MockRequest::new(Get, format!("/execution/{}", init_exec_id));

            // Act
            let mut response = req.dispatch_with(&rocket);
            let body_str = response.body().and_then(|b| b.into_string()).unwrap();
            let created: ExecutionCreated = serde_json::from_str(&body_str).unwrap();

            // Assert
            assert_eq!(init_exec_id, created.id);
            assert_that(&body_str.contains(init_exec_str));
        }
    }
}
