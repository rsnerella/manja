//! User API group: `/user/`
//!
use crate::kite::connect::{
    client::HTTPClient,
    models::{KiteApiResponse, Segment, SegmentKind, UserMargins, UserProfile},
};
use crate::kite::error::Result;

/// User related API endpoints.
///
/// Refer to the offical [API documentation](https://kite.trade/docs/connect/v3/user/#user).
pub struct User<'c> {
    pub client: &'c HTTPClient,
}

impl<'c> User<'c> {
    pub fn new(client: &'c HTTPClient) -> Self {
        Self { client }
    }

    /// Fetch the user profile from the API endpoint: `/user/profile`
    ///
    /// Refer Kite API [documentation](https://kite.trade/docs/connect/v3/user/#user-profile) for more details.
    pub async fn profile(&self) -> Result<KiteApiResponse<UserProfile>> {
        self.client.get(&"/user/profile").await
    }

    /// Fetch the user margins from the API endpoint: `/user/margins`
    ///
    /// Refer Kite API [documentation](https://kite.trade/docs/connect/v3/user/#funds-and-marginse) for more details.
    pub async fn margins(&self) -> Result<KiteApiResponse<UserMargins>> {
        self.client.get(&"/user/margins").await
    }

    /// Fetch the user margins for a specific segment (`equity` or `commodity`)
    /// from the API endpoint: `/user/margins/:segment`
    ///
    /// Refer Kite API [documentation](https://kite.trade/docs/connect/v3/user/#funds-and-marginse) for more details.
    pub async fn margins_by_segment(
        &self,
        segment: SegmentKind,
    ) -> Result<KiteApiResponse<Segment>> {
        self.client
            .get(&format!("/user/margins/{}", segment.as_ref()))
            .await
    }
}
