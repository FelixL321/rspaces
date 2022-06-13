use serde::{Deserialize, Serialize};

use crate::{Template, Tuple};

#[derive(Deserialize, Serialize)]
enum RequestType {
    GetResponse,
    GetpResponse,
    GetallResponse,
    QueryResponse,
    QuerypResponse,
    QueryallResponse,
}
#[derive(Serialize, Deserialize)]
struct MessageRequest {
    action: RequestType,
    source: u32,
    target: String,
    tuple: Tuple,
    template: Template,
}
