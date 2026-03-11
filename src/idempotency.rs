use uuid::Uuid;

pub fn new_idempotency_key() -> String {
    Uuid::new_v4().to_string()
}
