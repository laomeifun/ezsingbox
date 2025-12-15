use serde::Serialize;
use serde_json::{Value, json};

#[derive(Debug, Clone, Serialize)]
pub struct SingBoxConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Value>,

    pub inbounds: Vec<Value>,
    pub outbounds: Vec<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<Value>,
}

impl SingBoxConfig {
    fn default_dns_https() -> Value {
        json!({
            "servers": [
                {
                    "type": "https",
                    "tag": "cloudflare",
                    "server": "1.1.1.1",
                    "server_port": 443,
                    "path": "/dns-query"
                },
                {
                    "type": "https",
                    "tag": "google",
                    "server": "8.8.8.8",
                    "server_port": 443,
                    "path": "/dns-query"
                }
            ],
            "final": "cloudflare"
        })
    }

    pub fn server_default(inbounds: Vec<Value>, log_level: &str) -> Self {
        let log = Some(json!({
            "level": log_level,
            "timestamp": true
        }));

        // DNS module using sing-box 1.12+ "new dns servers" format.
        // (Avoid legacy `address` field which is deprecated since 1.12.0.)
        let dns = Some(Self::default_dns_https());

        let outbounds = vec![
            json!({ "type": "direct", "tag": "direct" }),
            json!({ "type": "block", "tag": "block" }),
        ];

        let route = Some(json!({
            "rules": [],
            "default_domain_resolver": "cloudflare",
            "final": "direct"
        }));

        Self {
            log,
            dns,
            inbounds,
            outbounds,
            route,
        }
    }

    pub fn client_default(proxy_outbound: Value, log_level: &str, mixed_listen: &str, mixed_port: u16) -> Self {
        let log = Some(json!({
            "level": log_level,
            "timestamp": true
        }));

        let dns = Some(Self::default_dns_https());

        let inbounds = vec![json!({
            "type": "mixed",
            "tag": "mixed-in",
            "listen": mixed_listen,
            "listen_port": mixed_port
        })];

        let outbounds = vec![
            proxy_outbound,
            json!({ "type": "direct", "tag": "direct" }),
            json!({ "type": "block", "tag": "block" }),
        ];

        let route = Some(json!({
            "rules": [],
            "default_domain_resolver": "cloudflare",
            "final": "proxy"
        }));

        Self {
            log,
            dns,
            inbounds,
            outbounds,
            route,
        }
    }

    pub fn to_pretty_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
