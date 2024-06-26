use backoff::ExponentialBackoff;

use crate::kite::connect::api::create_backoff_policy;
use crate::kite::connect::models::OrderReceipt;
use crate::kite::connect::{
    client::HTTPClient,
    models::{KiteApiResponse, Order, Trade},
};
use crate::kite::error::Result;

pub struct Orders<'c> {
    /// Reference to the HTTP client used for making API requests.
    pub client: &'c HTTPClient,
    /// Backoff policy for retrying API requests.
    backoff: ExponentialBackoff,
}

impl<'c> Orders<'c> {
    /// Creates a new instance of `Orders` with default API rate limits.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the `HTTPClient` used for making API requests.
    ///
    /// # Returns
    ///
    /// A new instance of `Orders`.
    pub fn new(client: &'c HTTPClient) -> Self {
        Self {
            client,
            // Default API rate limit: 10 req/sec
            backoff: create_backoff_policy(10),
        }
    }

    /// Sets a custom backoff policy for the `Orders` instance.
    ///
    /// # Arguments
    ///
    /// * `backoff` - An `ExponentialBackoff` instance specifying the backoff policy.
    ///
    /// # Returns
    ///
    /// The `Orders` instance with the updated backoff policy.
    pub fn with_backoff(mut self, backoff: ExponentialBackoff) -> Self {
        self.backoff = backoff;
        self
    }

    // KiteConnect API endpoints

    // POST
    pub async fn place_order(&self, order: &Order) -> Result<KiteApiResponse<OrderReceipt>> {
        self.client
            .post(&format!("/orders/{}", order.variety), order, &self.backoff)
            .await
    }

    pub async fn modify_order(
        &self,
        variety: &str,
        order_id: &str,
        order: &Order,
    ) -> Result<KiteApiResponse<OrderReceipt>> {
        self.client
            .put(
                &format!("/orders/{}/{}", variety, order_id),
                order,
                &self.backoff,
            )
            .await
    }

    pub async fn cancel_order(
        &self,
        variety: &str,
        order_id: &str,
    ) -> Result<KiteApiResponse<OrderReceipt>> {
        self.client
            .delete(
                &format!("/orders/{}/{}", variety, order_id),
                true,
                &self.backoff,
            )
            .await
    }

    pub async fn list_orders(&self) -> Result<KiteApiResponse<Vec<Order>>> {
        self.client.get(&format!("/orders"), &self.backoff).await
    }

    pub async fn get_order_history(&self, order_id: &str) -> Result<KiteApiResponse<Vec<Order>>> {
        self.client
            .get(&format!("/orders/{}", order_id), &self.backoff)
            .await
    }

    pub async fn list_trades(&self) -> Result<KiteApiResponse<Vec<Trade>>> {
        self.client.get(&format!("/trades"), &self.backoff).await
    }

    pub async fn get_order_trades(&self, order_id: &str) -> Result<KiteApiResponse<Vec<Trade>>> {
        self.client
            .get(&format!("/orders/{}/trades", order_id), &self.backoff)
            .await
    }
}
