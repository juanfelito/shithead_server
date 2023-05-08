use tonic::Status;

pub mod discard;
pub mod game;
pub mod player;
pub mod user;

#[derive(Debug)]
pub enum MediatorError {
    NotFound(String),
    Unauthorized(String),
    Unavailable(String),
    AlreadyExists(String),
    Internal(String),
    InvalidArgument(String),
}

impl std::fmt::Display for MediatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "{}", msg),
            Self::Unauthorized(msg) => write!(f, "{}", msg),
            Self::Unavailable(msg) => write!(f, "{}", msg),
            Self::AlreadyExists(msg) => write!(f, "{}", msg),
            Self::Internal(msg) => write!(f, "{}", msg),
            Self::InvalidArgument(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for MediatorError {}

impl Into<Status> for &MediatorError {
    fn into(self) -> Status {
        match self {
            MediatorError::AlreadyExists(str) => {
                Status::already_exists(format!("Already exists: {}", str))
            }
            MediatorError::Unavailable(str) => {
                Status::unavailable(format!("Unavailable: {}", str))
            }
            MediatorError::Unauthorized(str) => {
                Status::permission_denied(format!("Unauthorized: {}", str))
            }
            MediatorError::NotFound(str) => {
                Status::not_found(format!("Not found: {}", str))
            }
            MediatorError::Internal(str) => {
                Status::internal(format!("Internal error: {}", str))
            }
            MediatorError::InvalidArgument(str) => {
                Status::invalid_argument(format!("Invalid argument: {}", str))
            }
        }
    }
}
