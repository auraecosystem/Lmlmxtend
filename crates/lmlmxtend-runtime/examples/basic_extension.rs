use lmlmxtend_api::{
    CapabilityId, Extension, ExtensionError, ExtensionManifest, ExtensionRequest, ExtensionResponse,
};
use lmlmxtend_core::RuntimeConfig;
use lmlmxtend_runtime::Runtime;
use std::sync::Arc;

struct Greeting;

impl Extension for Greeting {
    fn manifest(&self) -> ExtensionManifest {
        ExtensionManifest {
            id: "example.greeting".into(),
            version: "0.1.0".into(),
            api_version: lmlmxtend_api::API_VERSION,
            capabilities: vec![CapabilityId::new("text.greet").expect("static capability is valid")],
        }
    }

    fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError> {
        Ok(ExtensionResponse::text(format!(
            "Hello, {}!",
            request.input
        )))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Runtime::new(RuntimeConfig::default());
    runtime.register(Arc::new(Greeting))?;
    let request = ExtensionRequest::new(CapabilityId::new("text.greet")?, "LMLMxtend");
    println!("{}", runtime.execute(request)?.output);
    Ok(())
}
