use crate::error::StableflowError;

#[derive(Debug, Clone)]
pub struct EntitySecret(String);

impl EntitySecret {
    pub fn from_env() -> Result<Self, StableflowError> {
        let value = std::env::var("CIRCLE_ENTITY_SECRET")
            .map_err(|_| StableflowError::InvalidConfig("missing CIRCLE_ENTITY_SECRET".into()))?;

        if value.trim().is_empty() {
            return Err(StableflowError::InvalidConfig(
                "CIRCLE_ENTITY_SECRET is empty".into(),
            ));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
