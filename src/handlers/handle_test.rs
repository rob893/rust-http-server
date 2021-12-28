use crate::http::{status_code::StatusCode, QueryStringValue, Request, Response};
use serde_json::json;

pub fn handle_test(req: &Request) -> Response {
    let mut message = "test";

    if let Some(q_string) = req.query_string() {
        if let Some(name) = q_string.get("name") {
            message = match name {
                QueryStringValue::Single(v) => v,
                QueryStringValue::Multiple(vec) => vec.first().unwrap_or(&message),
            }
        }
    }

    let body = json!({ "message": message }).to_string();

    Response::new(StatusCode::Ok, Some(body))
}
