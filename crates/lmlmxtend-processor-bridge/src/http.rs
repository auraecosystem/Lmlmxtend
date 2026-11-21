//! HTTP transport for the versioned, loopback-only LMLM Processor v1 service.

use crate::{LmlmProcessor, Modality, ProcessorError, ProcessorRequest, ProcessorResponse};
use lmlmxtend_api::ExecutionContext;
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

const PROTOCOL: &str = "lmlm.processor.v1";
const PROTOCOL_VERSION: u64 = 1;
const MAX_HTTP_RESPONSE_BYTES: u64 = 18 * 1024 * 1024;
static NEXT_REQUEST_ID: AtomicU64 = AtomicU64::new(1);

/// Calls a local LMLM Processor v1 service over HTTP.
///
/// The constructor accepts a socket address rather than a URL so hostnames cannot
/// resolve to remote machines. Non-loopback addresses are rejected.
pub struct HttpLmlmProcessor {
    address: SocketAddr,
    timeout: Duration,
}

impl HttpLmlmProcessor {
    /// Creates a local processor client with a 120-second transport timeout.
    pub fn new(address: SocketAddr) -> Result<Self, ProcessorError> {
        Self::with_timeout(address, Duration::from_secs(120))
    }

    /// Creates a local processor client with an explicit bounded timeout.
    pub fn with_timeout(address: SocketAddr, timeout: Duration) -> Result<Self, ProcessorError> {
        if !address.is_ipv4() || !address.ip().is_loopback() {
            return Err(ProcessorError::InvalidInput(
                "LMLM Processor v1 must use a loopback address".into(),
            ));
        }
        if timeout.is_zero() || timeout > Duration::from_secs(600) {
            return Err(ProcessorError::InvalidInput(
                "processor timeout must be between 1ms and 600s".into(),
            ));
        }
        Ok(Self { address, timeout })
    }

    fn exchange(
        &self,
        method: &str,
        path: &str,
        body: &[u8],
    ) -> Result<(u16, Vec<u8>), ProcessorError> {
        let mut stream = TcpStream::connect_timeout(&self.address, self.timeout)
            .map_err(|error| ProcessorError::Unavailable(error.to_string()))?;
        stream
            .set_read_timeout(Some(self.timeout))
            .map_err(|error| ProcessorError::Failed(error.to_string()))?;
        stream
            .set_write_timeout(Some(self.timeout))
            .map_err(|error| ProcessorError::Failed(error.to_string()))?;

        let headers = format!(
            "{method} {path} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            self.address,
            body.len()
        );
        stream
            .write_all(headers.as_bytes())
            .and_then(|_| stream.write_all(body))
            .map_err(|error| ProcessorError::Unavailable(error.to_string()))?;

        let mut response = Vec::new();
        stream
            .take(MAX_HTTP_RESPONSE_BYTES)
            .read_to_end(&mut response)
            .map_err(|error| ProcessorError::Unavailable(error.to_string()))?;
        let separator = response
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .ok_or_else(|| ProcessorError::Failed("invalid HTTP response headers".into()))?;
        let header_text = std::str::from_utf8(&response[..separator])
            .map_err(|error| ProcessorError::Failed(error.to_string()))?;
        let status = header_text
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|value| value.parse::<u16>().ok())
            .ok_or_else(|| ProcessorError::Failed("invalid HTTP status line".into()))?;
        let payload = response[separator + 4..].to_vec();
        Ok((status, payload))
    }
}

impl LmlmProcessor for HttpLmlmProcessor {
    fn health_check(&self) -> Result<(), ProcessorError> {
        let (status, body) = self.exchange("GET", "/v1/health", &[])?;
        if status != 200 {
            return Err(ProcessorError::Unavailable(format!(
                "local LMLM processor health returned HTTP {status}"
            )));
        }
        let value: Value = serde_json::from_slice(&body)
            .map_err(|error| ProcessorError::Failed(error.to_string()))?;
        if value.get("protocol").and_then(Value::as_str) != Some(PROTOCOL)
            || value.get("protocol_version").and_then(Value::as_u64) != Some(PROTOCOL_VERSION)
            || value.get("status").and_then(Value::as_str) != Some("ready")
        {
            return Err(ProcessorError::Failed(
                "local processor returned an incompatible health response".into(),
            ));
        }
        Ok(())
    }

    fn process(
        &self,
        request: ProcessorRequest,
        context: &ExecutionContext,
    ) -> Result<ProcessorResponse, ProcessorError> {
        context
            .check_cancelled()
            .map_err(|error| ProcessorError::Failed(error.to_string()))?;
        let request_id = request.correlation_id.unwrap_or_else(|| {
            format!(
                "lmlmxtend-{}",
                NEXT_REQUEST_ID.fetch_add(1, Ordering::Relaxed)
            )
        });
        let modality = match request.modality {
            Modality::Text => "text",
            Modality::Image => "image",
            Modality::Audio => "audio",
            Modality::Video => "video",
            Modality::Multimodal => "multimodal",
        };
        let body = json!({
            "protocol": PROTOCOL,
            "protocol_version": PROTOCOL_VERSION,
            "request_id": request_id,
            "modality": modality,
            "content_type": request.content_type,
            "payload_hex": encode_hex(&request.payload),
            "prompt": request.prompt,
            "parameters": {},
        })
        .to_string();
        let (status, response_body) = self.exchange("POST", "/v1/process", body.as_bytes())?;
        context
            .check_cancelled()
            .map_err(|error| ProcessorError::Failed(error.to_string()))?;
        let value: Value = serde_json::from_slice(&response_body)
            .map_err(|error| ProcessorError::Failed(error.to_string()))?;
        if status != 200 {
            let message = value
                .get("error")
                .and_then(|error| error.get("message"))
                .and_then(Value::as_str)
                .unwrap_or("local processor returned an error");
            return Err(ProcessorError::Failed(format!("HTTP {status}: {message}")));
        }
        if value.get("protocol").and_then(Value::as_str) != Some(PROTOCOL)
            || value.get("protocol_version").and_then(Value::as_u64) != Some(PROTOCOL_VERSION)
        {
            return Err(ProcessorError::Failed(
                "local processor returned an incompatible protocol version".into(),
            ));
        }
        let response_id = value
            .get("request_id")
            .and_then(Value::as_str)
            .ok_or_else(|| ProcessorError::Failed("response is missing request_id".into()))?;
        if response_id != request_id {
            return Err(ProcessorError::Failed(
                "local processor response request_id does not match".into(),
            ));
        }
        let content_type = value
            .get("content_type")
            .and_then(Value::as_str)
            .ok_or_else(|| ProcessorError::Failed("response is missing content_type".into()))?
            .to_owned();
        let payload_hex = value
            .get("payload_hex")
            .and_then(Value::as_str)
            .ok_or_else(|| ProcessorError::Failed("response is missing payload_hex".into()))?;
        let payload = decode_hex(payload_hex)?;
        if payload.len() > context.max_output_bytes() {
            return Err(ProcessorError::Failed(
                "local processor output exceeds the configured output limit".into(),
            ));
        }
        Ok(ProcessorResponse {
            payload,
            content_type,
        })
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn decode_hex(value: &str) -> Result<Vec<u8>, ProcessorError> {
    if value.len() % 2 != 0 {
        return Err(ProcessorError::Failed(
            "processor payload hex has odd length".into(),
        ));
    }
    let mut output = Vec::with_capacity(value.len() / 2);
    for pair in value.as_bytes().chunks_exact(2) {
        let high = hex_digit(pair[0])?;
        let low = hex_digit(pair[1])?;
        output.push((high << 4) | low);
    }
    Ok(output)
}

fn hex_digit(value: u8) -> Result<u8, ProcessorError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),
        _ => Err(ProcessorError::Failed(
            "processor payload contains invalid hex".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_encoding_is_lossless_for_arbitrary_bytes() {
        let input = [0, 1, 15, 16, 127, 128, 255];
        let encoded = encode_hex(&input);
        assert_eq!(encoded, "00010f107f80ff");
        assert_eq!(decode_hex(&encoded).unwrap(), input);
    }

    #[test]
    fn rejects_non_loopback_processor_address() {
        let address = "192.0.2.10:8765".parse().unwrap();
        assert!(HttpLmlmProcessor::new(address).is_err());
    }

    #[test]
    fn accepts_loopback_processor_address() {
        let address = "127.0.0.1:8765".parse().unwrap();
        assert!(HttpLmlmProcessor::new(address).is_ok());
    }

    #[test]
    fn sends_versioned_request_to_loopback_service() {
        use std::io::{Read, Write};
        use std::net::TcpListener;
        use std::thread;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 4096];
            let _ = stream.read(&mut request).unwrap();
            let body = r#"{"protocol":"lmlm.processor.v1","protocol_version":1,"request_id":"bridge-test-7","content_type":"text/plain; charset=utf-8","payload_hex":"4f4b"}"#;
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            )
            .unwrap();
        });

        let processor = HttpLmlmProcessor::new(address).unwrap();
        let mut request = ProcessorRequest::text("hello");
        request.correlation_id = Some("bridge-test-7".into());
        let response = processor
            .process(request, &ExecutionContext::default())
            .unwrap();
        assert_eq!(response.payload, b"OK");
        assert_eq!(response.content_type, "text/plain; charset=utf-8");
        server.join().unwrap();
    }
}
