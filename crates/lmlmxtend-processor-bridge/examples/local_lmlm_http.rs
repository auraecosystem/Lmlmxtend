//! Connect LMLMxtend to the local LMLM Processor v1 service.
//!
//! Start the LMLM processor service first (see docs/PROCESSOR_BRIDGE.md).

use lmlmxtend_api::{CapabilityId, ExtensionRequest};
use lmlmxtend_core::RuntimeConfig;
use lmlmxtend_processor_bridge::{HttpLmlmProcessor, ProcessorBridge};
use lmlmxtend_runtime::Runtime;
use std::net::SocketAddr;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "127.0.0.1:8765".parse()?;
    let processor = HttpLmlmProcessor::new(address)?;
    processor.health_check()?;

    let bridge = Arc::new(ProcessorBridge::new(Arc::new(processor)));
    let mut runtime = Runtime::new(RuntimeConfig::default());
    runtime.register(bridge)?;

    let request = ExtensionRequest::new(
        CapabilityId::new(ProcessorBridge::CAPABILITY_ID)?,
        "Explain how LMLM and LMLMxtend work together.",
    );
    let response = runtime.execute(request)?;
    println!("{}", response.output);
    Ok(())
}
