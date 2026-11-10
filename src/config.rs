/// Runtime configuration, read only from the environment (NFR-3, ADR-0006).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub const DEFAULT_PORT: u16 = 8080;

    pub fn from_env() -> Self {
        Self::from_port_var(std::env::var("PORT").ok().as_deref())
    }

    fn from_port_var(port: Option<&str>) -> Self {
        let port = port
            .and_then(|p| p.trim().parse().ok())
            .unwrap_or(Self::DEFAULT_PORT);
        Self { port }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nfr3_port_defaults_to_8080() {
        assert_eq!(Config::from_port_var(None).port, 8080);
    }

    #[test]
    fn nfr3_port_read_from_env_value() {
        assert_eq!(Config::from_port_var(Some("9000")).port, 9000);
    }

    #[test]
    fn nfr3_invalid_port_falls_back_to_default() {
        assert_eq!(Config::from_port_var(Some("not-a-port")).port, 8080);
    }
}
