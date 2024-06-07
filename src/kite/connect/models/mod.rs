use serde::{Deserialize, Serialize};

/// Default `T` is `HashMap`
#[derive(Serialize, Deserialize, Debug)]
pub struct KiteApiResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error_type: Option<String>,
}

// Models for the `/session` API group
mod session;
pub use session::UserSession;

// Models for the `/user/` API group
mod user;
pub use user::{Available, Segment, SegmentKind, UserMargins, UserProfile, Utilised};
