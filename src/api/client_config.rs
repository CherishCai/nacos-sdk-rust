use std::collections::HashMap;

/// Configures settings for Client.
#[derive(Debug, Clone, Default)]
pub struct ClientConfig {
    /// server_addr like 127.0.0.1:9848
    pub(crate) server_addr: Option<String>,
    /// client_name maybe the same as app_name
    pub(crate) client_name: Option<String>,
    /// metadata
    pub(crate) labels: HashMap<String, String>,
}

impl ClientConfig {
    /// Creates a new `ClientConfig`.
    pub fn new() -> Self {
        ClientConfig {
            server_addr: None,
            client_name: None,
            labels: HashMap::default(),
        }
    }

    /// Sets the server addr against.
    pub fn server_addr(self, server_addr: impl Into<String>) -> Self {
        ClientConfig {
            server_addr: Some(server_addr.into()),
            ..self
        }
    }

    /// Sets the client name against.
    pub fn client_name(self, client_name: impl Into<String>) -> Self {
        ClientConfig {
            client_name: Some(client_name.into()),
            ..self
        }
    }

    /// Sets the labels against.
    pub fn labels(self, labels: HashMap<String, String>) -> Self {
        ClientConfig { labels, ..self }
    }
}
