use backoff::ExponentialBackoff;

use crate::kite::connect::api::create_backoff_policy;
use crate::kite::connect::{
    client::HTTPClient,
    models::{Auction, Holding, KiteApiResponse, Position, PositionConversionRequest},
};
use crate::kite::error::Result;

/// ## Exiting holdings and positions
///
/// There are no special API calls for exiting instruments from holdings and
/// positions portfolios. The way to do it is to place an opposite `BUY` or
/// `SELL` order depending on whether the position is a long or a short
/// (`MARKET` order for an immediate exit). It is important to note that the
/// exit order should carry the same product as the existing position. If the
/// exit order is of a different margin product, it may be treated as a new
/// position in the portfolio.
pub struct Portfolio<'c> {
    /// Reference to the HTTP client used for making API requests.
    pub client: &'c HTTPClient,
    /// Backoff policy for retrying API requests.
    backoff: ExponentialBackoff,
}

impl<'c> Portfolio<'c> {
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

    // ===== [ KiteConnect API endpoints ] =====

    /// Retrieve the list of long term equity holdings
    ///
    /// Holdings contain the user's portfolio of long term equity delivery
    /// stocks. An instrument in a holdings portfolio remains there
    /// indefinitely until its sold or is delisted or changed by the exchanges.
    /// Underneath it all, instruments in the holdings reside in the user's
    /// DEMAT account, as settled by exchanges and clearing institutions.
    pub async fn get_holdings(&self) -> Result<KiteApiResponse<Vec<Holding>>> {
        self.client
            .get(&format!("/portfolio/holdings"), &self.backoff)
            .await
    }

    /// Retrieve the list of short term positions
    ///
    /// Positions contain the user's portfolio of short to medium term derivatives
    /// (futures and options contracts) and intraday equity stocks. Instruments
    /// in the positions portfolio remain there until they're sold, or until
    /// expiry, which, for derivatives, is typically three months. Equity positions
    /// carried overnight move to the holdings portfolio the next day.
    ///
    /// The positions API returns two sets of positions, `net` and `day`. `net`
    /// is the actual, current net position portfolio, while `day` is a snapshot
    /// of the buying and selling activity for that particular day. This is
    /// useful for computing intraday profits and losses for trading strategies.
    pub async fn get_positions(&self) -> Result<KiteApiResponse<Vec<Position>>> {
        self.client
            .get(&format!("/portfolio/positions"), &self.backoff)
            .await
    }

    /// Retrieve the list of auctions that are currently being held
    ///
    /// This API returns a list of auctions that are currently being held,
    /// along with details about each auction such as the auction number,
    /// the security being auctioned, the last price of the security, and
    /// the quantity of the security being offered. Only the stocks that
    /// you hold in your demat account will be shown in the auctions list.
    pub async fn get_auctions(&self) -> Result<KiteApiResponse<Vec<Auction>>> {
        self.client
            .get(&format!("/portfolio/holdings/auctions"), &self.backoff)
            .await
    }

    /// Convert the margin product of an open position
    ///
    /// All positions held are of specific margin products such as NRML, MIS
    /// etc. A position can have one and only one margin product. These
    /// products affect how the user's margin usage and free cash values are
    /// computed, and a user may want to covert or change a position's margin
    /// product from time to time. More on [margin policies](https://zerodha.com/z-connect/general/zerodha-margin-policies).
    pub async fn convert_position(
        &self,
        request: PositionConversionRequest,
    ) -> Result<KiteApiResponse<bool>> {
        self.client
            .put(&format!("/portfolio/positions"), request, &self.backoff)
            .await
    }

    // TODO!
    // Initiating authorisation
    //
    // curl --request POST https://api.kite.trade/portfolio/holdings/authorise
    // -H "X-Kite-Version: 3" \
    // -H "Authorization: token api_key:access_token" \
    // -d "isin=INE002A01018" -d "quantity=50" \
    // -d "isin=INE009A01021" -d "quantity=50"
    //
    // {
    // "status": "success",
    // "data": {
    // "request_id": "na8QgCeQm05UHG6NL9sAGRzdfSF64UdB"
    // }
    // }
}
