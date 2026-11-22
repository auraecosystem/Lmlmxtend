//! End-to-end generation against an installed local LMLM native CPU model.
//!
//! The companion scripts/e2e_native_lmlm.sh starts the real LMLM Processor v1
//! service and sets LMLM_PROCESSOR_ADDR before running this example.

use lmlmxtend_api::ExecutionContext;
use lmlmxtend_processor_bridge::{
    GenerationParameters, HttpLmlmProcessor, ProcessorBridge, ProcessorRequest,
};
use std::net::SocketAddr;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = std::env::var("LMLM_PROCESSOR_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8765".to_owned())
        .parse()?;

    let processor = HttpLmlmProcessor::new(address)?;
    processor.health_check()?;

    let bridge = ProcessorBridge::new(Arc::new(processor));
    let response = bridge.process_with_parameters(
        ProcessorRequest::text(
            "In one short sentence, explain why local model execution protects data locality.",
        ),
        &GenerationParameters {
            max_tokens: Some(64),
            temperature: Some(0.2),
        },
        &ExecutionContext::default(),
    )?;

    let output = String::from_utf8(response.payload)?;
    if output.trim().is_empty() {
        return Err("LMLM returned an empty completion".into());
    }

    println!("LMLM Processor v1 end-to-end response:\n{output}");
    Ok(())
}
