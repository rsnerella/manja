use std::fmt;

use serde::{Deserialize, Serialize};

/// Represents the variety of an order.
///
/// This enum contains several constant values used for placing different types of orders.
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderVariety {
    /// Regular order.
    #[serde(rename = "regular")]
    Regular,

    /// After Market Order.
    #[serde(rename = "amo")]
    AfterMarket,

    /// Cover Order.
    #[serde(rename = "co")]
    Cover,

    /// Iceberg Order.
    #[serde(rename = "iceberg")]
    Iceberg,

    /// Auction Order.
    #[serde(rename = "auction")]
    Auction,
}

impl fmt::Display for OrderVariety {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            // NOTE: String representation is primarily used for routing to the
            // particulart API endpoint for placing orders
            OrderVariety::Regular => "regular",
            OrderVariety::AfterMarket => "amo",
            OrderVariety::Cover => "co",
            OrderVariety::Iceberg => "iceberg",
            OrderVariety::Auction => "auction",
        };
        write!(f, "{}", display_str)
    }
}

/// Represents the various statuses an order can have during its lifecycle.
///
/// The status field in the order response shows the current state of the order. The most common statuses are OPEN, COMPLETE, CANCELLED, and REJECTED.
/// An order can traverse through several interim and temporary statuses during its lifetime. For example, when an order is first placed or modified,
/// it instantly passes through several stages before reaching its end state. Some of these are highlighted below.
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderStatus {
    /// The order has been placed and is currently open.
    #[serde(rename = "OPEN")]
    Open,

    /// The order has been completely filled.
    #[serde(rename = "COMPLETE")]
    Complete,

    /// The order has been cancelled.
    #[serde(rename = "CANCELLED")]
    Cancelled,

    /// The order has been rejected.
    #[serde(rename = "REJECTED")]
    Rejected,

    /// Order request has been received by the backend.
    #[serde(rename = "PUT ORDER REQ RECEIVED")]
    PutOrderReqReceived,

    /// Order pending validation by the RMS (Risk Management System).
    #[serde(rename = "VALIDATION PENDING")]
    ValidationPending,

    /// Order is pending registration at the exchange.
    #[serde(rename = "OPEN PENDING")]
    OpenPending,

    /// Order's modification values are pending validation by the RMS.
    #[serde(rename = "MODIFY VALIDATION PENDING")]
    ModifyValidationPending,

    /// Order's modification values are pending registration at the exchange.
    #[serde(rename = "MODIFY PENDING")]
    ModifyPending,

    /// Order's placed but the fill is pending based on a trigger price.
    #[serde(rename = "TRIGGER PENDING")]
    TriggerPending,

    /// Order's cancellation request is pending registration at the exchange.
    #[serde(rename = "CANCEL PENDING")]
    CancelPending,

    /// Same as `PUT ORDER REQ RECEIVED`, but for AMOs (After Market Orders).
    #[serde(rename = "AMO REQ RECEIVED")]
    AmoReqReceived,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            OrderStatus::Open => "OPEN",
            OrderStatus::Complete => "COMPLETE",
            OrderStatus::Cancelled => "CANCELLED",
            OrderStatus::Rejected => "REJECTED",
            OrderStatus::PutOrderReqReceived => "PUT ORDER REQ RECEIVED",
            OrderStatus::ValidationPending => "VALIDATION PENDING",
            OrderStatus::OpenPending => "OPEN PENDING",
            OrderStatus::ModifyValidationPending => "MODIFY VALIDATION PENDING",
            OrderStatus::ModifyPending => "MODIFY PENDING",
            OrderStatus::TriggerPending => "TRIGGER PENDING",
            OrderStatus::CancelPending => "CANCEL PENDING",
            OrderStatus::AmoReqReceived => "AMO REQ RECEIVED",
        };
        write!(f, "{}", display_str)
    }
}

/// Represents the type of an order.
///
/// This enum contains several constant values used for placing different types of orders.
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    /// Market order.
    #[serde(rename = "MARKET")]
    Market,

    /// Limit order.
    #[serde(rename = "LIMIT")]
    Limit,

    /// Stoploss order.
    #[serde(rename = "SL")]
    Stoploss,

    /// Stoploss-market order.
    #[serde(rename = "SL-M")]
    StoplossMarket,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
            OrderType::Stoploss => "SL",
            OrderType::StoplossMarket => "SL-M",
        };
        write!(f, "{}", display_str)
    }
}

/// Represents the product type for an order.
///
/// This enum contains several constant values used for specifying the product type.
#[derive(Debug, Serialize, Deserialize)]
pub enum ProductType {
    /// Cash & Carry for equity.
    #[serde(rename = "CNC")]
    CashAndCarry,

    /// Normal for futures and options.
    #[serde(rename = "NRML")]
    Normal,

    /// Margin Intraday Squareoff for futures and options.
    #[serde(rename = "MIS")]
    MarginIntradaySquareoff,
}

impl fmt::Display for ProductType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            ProductType::CashAndCarry => "CNC",
            ProductType::Normal => "NRML",
            ProductType::MarginIntradaySquareoff => "MIS",
        };
        write!(f, "{}", display_str)
    }
}

/// Represents the validity of an order.
///
/// This enum contains several constant values used for specifying the order validity.
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderValidity {
    /// Regular order.
    #[serde(rename = "DAY")]
    Day,

    /// Immediate or Cancel.
    #[serde(rename = "IOC")]
    ImmediateOrCancel,

    /// Order validity in minutes.
    #[serde(rename = "TTL")]
    TimeToLive,
}

impl fmt::Display for OrderValidity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            OrderValidity::Day => "DAY",
            OrderValidity::ImmediateOrCancel => "IOC",
            OrderValidity::TimeToLive => "TTL",
        };
        write!(f, "{}", display_str)
    }
}

/// Represents the validity of an order.
///
/// This enum contains several constant values used for specifying the order validity.
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    /// Buy.
    #[serde(rename = "BUY")]
    BUY,

    /// Sell.
    #[serde(rename = "SELL")]
    SELL,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            TransactionType::BUY => "BUY",
            TransactionType::SELL => "SELL",
        };
        write!(f, "{}", display_str)
    }
}
