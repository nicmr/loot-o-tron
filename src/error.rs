use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseCommandError {
    #[error("Wrong Parameter Count provided. Required: {required:?}, Actual: {actual:?}")]
    WrongParameterCount{
        required: u32,
        actual: u32
    },
    #[error("Parsing parameter failed.")]
    ParsingFailure{  // TODO: improve error variant name
    }
}

#[derive(Debug, Error)]
pub enum WowheadError {
    #[error("Failed to decode tooltip.")]
    TooltipParsingError{
        #[from]
        source: quick_xml::Error,
    },
}