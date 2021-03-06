use crate::traits::Language;
use crate::CombinedDeviceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceError {
    /// The input is not currently supported.
    #[error("UnsupportedInput")]
    UnsupportedInput,
}

#[derive(Debug, PartialEq, Error)]
pub enum InputSelectorError {
    #[error("{0}")]
    Device(DeviceError),
    #[error("{0}")]
    Other(CombinedDeviceError),
}

/// Available input.
#[derive(Debug, PartialEq, Serialize)]
pub struct AvailableInput {
    /// Unique key for the input. The key should not be exposed to users in speech or response.
    pub key: String,
    /// List of names for the input for all available languages.
    pub names: Vec<InputName>,
}

/// Input for a given available language.
#[derive(Debug, PartialEq, Serialize)]
pub struct InputName {
    /// Language code.
    pub lang: Language,
    /// User-friendly names for the input, in a given language. The first synonym is used in Google Assistant's response to the user.
    pub name_synonym: Vec<String>,
}

/// Trait for devices that can change media inputs. These inputs can have dynamic names per device, and may represent audio or video feeds, hardwired or networked.
pub trait InputSelector {
    /// List of objects representing input audio or video feeds.
    /// Feeds can be hardwired or networked. Each feed should be named and reasonably persistent.
    /// Make sure to define your synonyms carefully to prevent undesired (over-)triggering.
    fn get_available_inputs(&self) -> Result<Vec<AvailableInput>, InputSelectorError>;

    /// Indicates if the device supports using one-way (true) or two-way (false) communication. Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    /// Default: false
    fn is_command_only_input_selector(&self) -> Result<Option<bool>, InputSelectorError> {
        Ok(None)
    }

    /// True if the list of output is ordered. This also indicates that the 'next' and 'previous' functionality is available.
    /// Default: false
    fn has_ordered_inputs(&self) -> Result<Option<bool>, InputSelectorError> {
        Ok(None)
    }

    /// Key of the input currently in use.
    fn get_current_input(&self) -> Result<String, InputSelectorError>;

    /// Set the media input.
    /// - `input` Key of the new input.
    fn set_input(&mut self, input: String) -> Result<(), InputSelectorError>;

    /// Select the next input. Only applicable when the orderedInputs attribute is set to true.
    fn set_next_input(&mut self) -> Result<(), InputSelectorError>;

    /// Select the previous input. Only applicable when the orderedInputs attribute is set to true.
    fn set_previous_input(&mut self) -> Result<(), InputSelectorError>;
}
