use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User<'a> {
    id: uuid::Uuid,
    email: &'a str,
    first_name: &'a str,
    last_name: &'a str,
}
