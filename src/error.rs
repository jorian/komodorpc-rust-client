use {RpcError, ClientError};
use core::fmt;
use std::fmt::Formatter;
use std::error::Error;

#[derive(Debug)]
pub enum ApiError {
    RPC(RpcError),
    Client(ClientError),
    Other
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ApiError::RPC(ref cause) => write!(f, "RPC error: {}", cause.message ),
            ApiError::Client(ref _cause) =>
                match _cause {
                    ClientError::Json(json_err) => fmt::Display::fmt(json_err, f),
                    ClientError::Transport(transport_error) => fmt::Display::fmt(transport_error, f),
                },
            ApiError::Other => write!(f, "Unknown error")
        }
    }
}

impl Error for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::RPC(ref cause) => cause.description(),
            ApiError::Client(ref cause) => cause.description(),
            ApiError::Other => "Unknown error!",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApiError::RPC(ref cause) => Some(cause),
            ApiError::Client(ref cause) => Some(cause),
            ApiError::Other => None,
        }
    }
}
impl From<RpcError> for ApiError {
    fn from(cause: RpcError) -> ApiError {
        ApiError::RPC(cause)
    }
}
impl From<ClientError> for ApiError {
    fn from(cause: ClientError) -> ApiError {
        ApiError::Client(cause)
    }
}