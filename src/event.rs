use rocket_contrib::JSON;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Event {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct EventCreated {
    id: Uuid,
    event: Event,
}

#[post("/event", format = "application/json", data = "<event>")]
pub fn event(event: JSON<Event>) -> JSON<EventCreated> {
    let created = EventCreated{ event: event.into_inner(), id: Uuid::new_v4() };
    return JSON(created);
}

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
        let rocket = rocket::ignite().mount("/", routes![super::event]);
        let event = super::Event { name: "test".to_string() };
        let event_json = serde_json::to_string(&event).unwrap();
        let event_str: &str = &event_json.to_string();
        let mut req = MockRequest::new(Post, "/event")
            .header(ContentType::JSON)
            .body(event_json);

        // Act
        let mut response = req.dispatch_with(&rocket);
        let body_str = response.body().and_then(|b| b.into_string()).unwrap();
        let created: super::EventCreated = serde_json::from_str(&body_str).unwrap();

        // Assert
        assert_eq!(response.status(), Status::Ok);
        assert_that(&body_str).contains(event_str);
        assert_that(&Uuid::parse_str(&created.id.to_string()).unwrap()).is_equal_to(created.id);
    }
}
