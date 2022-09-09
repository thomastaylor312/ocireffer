use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct OcirefferActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for OcirefferActor {
    async fn handle_request(
        &self,
        _ctx: &Context,
        _req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        Ok(HttpResponse {
            body: serde_json::to_vec(&ShieldsResponse::new(
                "nats_messaging",
                "wasmcloud.azurecr.io/nats_messaging:0.14.5",
                "green",
            ))
            .unwrap_or_default(),
            ..Default::default()
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ShieldsResponse {
    #[serde(rename = "schemaVersion")]
    schema_version: u8,
    label: String,
    message: String,
    color: String,
    #[serde(rename = "namedLogo")]
    named_logo: String,
}

impl ShieldsResponse {
    fn new(label: &str, message: &str, color: &str) -> Self {
        ShieldsResponse {
            schema_version: 1,
            label: label.to_string(),
            message: message.to_string(),
            color: color.to_string(),
            named_logo: "wasmcloud".to_string(),
        }
    }
}