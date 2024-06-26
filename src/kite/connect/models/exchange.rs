/// Exchange options.
///
/// This enum represents various exchange options available for trading.
/// Each variant corresponds to a specific exchange or market segment.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum Exchange {
    /// National Stock Exchange
    #[default]
    NSE,
    /// National Futures and Options
    NFO,
    /// Currency Derivatives Segment
    CDS,
    /// Bombay Stock Exchange
    BSE,
    /// Bombay Futures and Options
    BFO,
    /// Bombay Currency Derivatives
    BCD,
    /// Multi Commodity Exchange
    MCX,
    /// Multi Commodity Exchange Stock Exchange
    MCXSX,
    /// Stock Market Indices
    INDICES,
}

impl Exchange {
    /// Returns the divisor for the exchange.
    ///
    /// The divisor is used to normalize values based on the exchange.
    /// For `CDS` and `BCD`, specific divisors are returned, while a default divisor
    /// is used for other exchanges.
    ///
    /// # Returns
    ///
    /// A `f64` value representing the divisor.
    pub(crate) fn divisor(&self) -> f64 {
        match self {
            Self::CDS => 100_000_0.0,
            Self::BCD => 100_0.0,
            _ => 100.0,
        }
    }

    /// Determines if the exchange is tradable.
    ///
    /// The `INDICES` exchange is not tradable, while all other exchanges are.
    ///
    /// # Returns
    ///
    /// A `bool` indicating if the exchange is tradable.
    pub(crate) fn is_tradable(&self) -> bool {
        match self {
            Self::INDICES => false,
            _ => true,
        }
    }
}

impl From<usize> for Exchange {
    /// Creates an `Exchange` from a `usize`.
    ///
    /// Maps integer values to specific exchanges. Values outside the predefined range
    /// default to `NSE`.
    ///
    /// # Arguments
    ///
    /// * `value` - A `usize` representing the exchange.
    ///
    /// # Returns
    ///
    /// An `Exchange` variant corresponding to the input value.
    fn from(value: usize) -> Self {
        match value {
            9 => Self::INDICES,
            8 => Self::MCXSX,
            7 => Self::MCX,
            6 => Self::BCD,
            5 => Self::BFO,
            4 => Self::BSE,
            3 => Self::CDS,
            2 => Self::NFO,
            1 => Self::NSE,
            _ => Self::NSE,
        }
    }
}

impl From<&str> for Exchange {
    /// Creates an `Exchange` from a `&str`.
    ///
    /// Maps string representations of exchange names to specific exchanges. Unrecognized
    /// strings default to `NSE`.
    ///
    /// # Arguments
    ///
    /// * `value` - An `&str` representing the exchange.
    ///
    /// # Returns
    ///
    /// An `Exchange` variant corresponding to the input string.
    fn from(value: &str) -> Self {
        match value {
            "NSE" => Self::NSE,
            "NFO" => Self::NFO,
            "CDS" => Self::CDS,
            "BSE" => Self::BSE,
            "BFO" => Self::BFO,
            "BCD" => Self::BCD,
            "MCX" => Self::MCX,
            "MCXSX" => Self::MCXSX,
            "INDICES" => Self::INDICES,
            _ => Self::NSE,
        }
    }
}

impl From<Exchange> for &str {
    /// Converts an `Exchange` to a `&str`.
    ///
    /// Maps each `Exchange` variant to its string representation.
    ///
    /// # Arguments
    ///
    /// * `value` - An `Exchange` variant.
    ///
    /// # Returns
    ///
    /// An `&str` corresponding to the exchange.
    fn from(value: Exchange) -> Self {
        match value {
            Exchange::NSE => "NSE",
            Exchange::NFO => "NFO",
            Exchange::CDS => "CDS",
            Exchange::BSE => "BSE",
            Exchange::BFO => "BFO",
            Exchange::BCD => "BCD",
            Exchange::MCX => "MCX",
            Exchange::MCXSX => "MCXSX",
            Exchange::INDICES => "INDICES",
        }
    }
}
