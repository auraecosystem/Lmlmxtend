//! Typed generation controls shared by local LMLM processor adapters.

use crate::ProcessorError;

/// Optional text-generation parameters supported by the LMLM Processor v1
/// backends. Unset values preserve the selected backend's own defaults.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GenerationParameters {
    /// Maximum number of generated tokens (1..=4096).
    pub max_tokens: Option<u32>,
    /// Sampling temperature (0.0..=2.0).
    pub temperature: Option<f32>,
}

impl GenerationParameters {
    /// Validates the parameter ranges before any network or model work begins.
    pub fn validate(&self) -> Result<(), ProcessorError> {
        if let Some(max_tokens) = self.max_tokens {
            if !(1..=4096).contains(&max_tokens) {
                return Err(ProcessorError::InvalidInput(
                    "max_tokens must be between 1 and 4096".into(),
                ));
            }
        }
        if let Some(temperature) = self.temperature {
            if !temperature.is_finite() || !(0.0..=2.0).contains(&temperature) {
                return Err(ProcessorError::InvalidInput(
                    "temperature must be finite and between 0 and 2".into(),
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.max_tokens.is_none() && self.temperature.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_supported_generation_parameter_ranges() {
        GenerationParameters {
            max_tokens: Some(4096),
            temperature: Some(2.0),
        }
        .validate()
        .unwrap();
    }

    #[test]
    fn rejects_out_of_range_generation_parameters() {
        assert!(GenerationParameters {
            max_tokens: Some(0),
            temperature: None,
        }
        .validate()
        .is_err());
        assert!(GenerationParameters {
            max_tokens: Some(4097),
            temperature: None,
        }
        .validate()
        .is_err());
        assert!(GenerationParameters {
            max_tokens: None,
            temperature: Some(f32::NAN),
        }
        .validate()
        .is_err());
        assert!(GenerationParameters {
            max_tokens: None,
            temperature: Some(2.01),
        }
        .validate()
        .is_err());
    }
}
