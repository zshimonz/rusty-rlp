use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use rlp::PayloadInfo;

create_exception!(rusty_rlp, EncodingError, PyException);
create_exception!(rusty_rlp, DecodingError, PyException);

// We can not implement From for rlp::DecoderError as it is in a foreign crate. Hence, we use
// map_err and implement From on RlpDecoderError instead.
pub struct RlpDecoderError(pub rlp::DecoderError);
impl std::convert::From<RlpDecoderError> for PyErr {
    fn from(err: RlpDecoderError) -> PyErr {
        DecodingError::new_err(err.0.to_string())
    }
}

pub fn construct_short_string_error<T>(val: &u8) -> Result<T, PyErr> {
    Err(DecodingError::new_err(format!(
        "Encoded {} as short string although single byte was possible",
        val
    )))
}

pub fn construct_invariant_error<T>() -> Result<T, PyErr> {
    Err(DecodingError::new_err("Invariant"))
}

pub fn construct_trailing_bytes_error<T>(payload_info: &PayloadInfo) -> Result<T, PyErr> {
    Err(DecodingError::new_err(format!(
        "Trailing bytes. Payload Info {:?}",
        payload_info
    )))
}
