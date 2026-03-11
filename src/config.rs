use secrecy::{ExposeSecret, SecretString};
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum Environment {
    Sandbox,
    Production,
}

impl Environment {
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Sandbox => "https://api.circle.com",
            Environment::Production => "https://api.circle.com",
        }
    }
}

#[derive(Clone)]
pub struct StableflowConfig {
    api_key: SecretString,
    environment: Environment,
    timeout: Duration,
    user_agent: String,
}

impl StableflowConfig {
    pub fn new(api_key: impl Into<String>, environment: Environment) -> Self {
        Self {
            api_key: SecretString::new(api_key.into()),
            environment,
            timeout: Duration::from_secs(30),
            user_agent: format!("stableflow-rs/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn base_url(&self) -> &str {
        self.environment.base_url()
    }

    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    pub(crate) fn bearer_token(&self) -> String {
        format!("Bearer {}", self.api_key.expose_secret())
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }
}
