use {RpcError, ClientError};
use core::fmt;
use std::fmt::Formatter;
use std::error::Error;

#[derive(Debug)]
enum ApiError {
    RPC(RpcError),
    Client(ClientError),
    Other
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ApiError::RPC(ref cause) => write!(f, "RPC error: {}", cause.message ),
            ApiError::Client(ref _cause) =>
                write!(f, "Client error"),
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