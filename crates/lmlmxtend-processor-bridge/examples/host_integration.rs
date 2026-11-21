//! Minimal host wiring example. Replace EchoProcessor with the concrete LMLM backend.

use lmlmxtend_api::{CapabilityId, ExecutionContext, ExtensionRequest};
use lmlmxtend_core::RuntimeConfig;
use lmlmxtend_processor_bridge::{
    LmlmProcessor, Modality, ProcessorBridge, ProcessorError, ProcessorRequest, ProcessorResponse,
};
use lmlmxtend_runtime::Runtime;
use std::sync::Arc;

struct EchoProcessor;

impl LmlmProcessor for EchoProcessor {
    fn process(
        &self,
        request: ProcessorRequest,
        context: &ExecutionContext,
    ) -> Result<ProcessorResponse, ProcessorError> {
        context
            .check_cancelled()
            .map_err(|error| ProcessorError::Failed(error.to_string()))?;
        if request.modality != Modality::Text {
            return Err(ProcessorError::UnsupportedModality(request.modality));
        }
        Ok(ProcessorResponse {
            payload: request.payload,
            content_type: request.content_type,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bridge = Arc::new(ProcessorBridge::new(Arc::new(EchoProcessor)));
    let mut runtime = Runtime::new(RuntimeConfig::default());
    runtime.register(bridge)?;

    let request = ExtensionRequest::new(
        CapabilityId::new(ProcessorBridge::CAPABILITY_ID)?,
        "LMLM local processor",
    );
    let response = runtime.execute(request)?;
    println!("{}", response.output);
    Ok(())
}
