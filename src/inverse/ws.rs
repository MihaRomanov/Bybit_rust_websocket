use crate::callback::{Callback, On};
use crate::error::Result;
use crate::{future_ping, run, Credentials};

use serde::{Deserialize, Serialize};
use serde_json;

const MAINNET: &str = "wss://stream.bybit.com/realtime";
const TESTNET: &str = "wss://stream-testnet.bybit.com/realtime";

#[derive(Deserialize, Debug)]
pub struct PingRequest<'a> {
    pub op: &'a str,
}

/// Also handles subscription-acknowledgment messages
#[derive(Deserialize, Debug)]
pub struct Pong<'a> {
    pub success: bool,
    pub ret_msg: &'a str,
    pub conn_id: &'a str,
    pub request: PingRequest<'a>,
}

#[derive(Deserialize, Debug)]
pub struct BaseResponse<'a, D> {
    pub topic: &'a str,
    pub data: D,
}

#[derive(Deserialize, Debug)]
pub struct BaseResponseWithTimestamp<'a, D> {
    pub topic: &'a str,
    pub data: D,
    pub timestamp_e6: u64,
}

#[derive(Deserialize, Debug)]
pub struct Response<'a, D> {
    pub topic: &'a str,
    #[serde(alias = "type")]
    pub res_type: &'a str,
    pub data: D,
    pub cross_seq: u64,
    pub timestamp_e6: u64,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookItem<'a> {
    pub price: &'a str,
    pub symbol: &'a str,
    pub id: u64,
    pub side: &'a str,
    pub size: u64,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookDeleteItem<'a> {
    pub price: &'a str,
    pub symbol: &'a str,
    pub id: u64,
    pub side: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookDelta<'a> {
    #[serde(borrow)]
    pub delete: Vec<OrderBookDeleteItem<'a>>,
    pub update: Vec<OrderBookItem<'a>>,
    pub insert: Vec<OrderBookItem<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct Trade<'a> {
    // Symbol
    pub symbol: &'a str,
    // Direction of price change
    pub tick_direction: &'a str,
    // Order price
    pub price: f64,
    // Position qty
    pub size: u64,
    // UTC time
    pub timestamp: &'a str,
    // Millisecond timestamp
    pub trade_time_ms: u64,
    // Direction of taker
    pub side: &'a str,
    // Trade ID
    pub trade_id: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct Insurance<'a> {
    // Symbol
    pub currency: &'a str,
    // UTC time
    pub timestamp: &'a str,
    // Wallet balance
    pub wallet_balance: i64,
}

#[derive(Deserialize, Debug)]
pub struct PerpetualInstrumentInfoSnapshot<'a> {
    // id
    pub id: u64,
    // Symbol
    pub symbol: &'a str,
    pub last_price_e4: u64,
    // Latest transaction price
    pub last_price: &'a str,
    pub bid1_price_e4: u64,
    // Best bid price
    pub bid1_price: &'a str,
    pub ask1_price_e4: u64,
    // Best ask price
    pub ask1_price: &'a str,
    // Direction of price change
    pub last_tick_direction: &'a str,
    pub prev_price_24h_e4: u64,
    // Price of 24 hours ago
    pub prev_price_24h: &'a str,
    // Percentage change of market price relative to 24h * 10^4
    pub price_24h_pcnt_e6: i64,
    pub high_price_24h_e4: u64,
    // The highest price in the last 24 hours
    pub high_price_24h: &'a str,
    pub low_price_24h_e4: u64,
    // Lowest price in the last 24 hours
    pub low_price_24h: &'a str,
    pub prev_price_1h_e4: u64,
    // Hourly market price an hour ago
    pub prev_price_1h: &'a str,
    pub price_1h_pcnt_e6: i64,
    pub mark_price_e4: u64,
    // Mark price
    pub mark_price: &'a str,
    pub index_price_e4: u64,
    // Index_price
    pub index_price: &'a str,
    // Open interest. The update is not immediate - slowest update is 1 minute
    pub open_interest: u64,
    pub open_value_e8: u64,
    // Total turnover
    pub total_turnover_e8: u64,
    // Turnover for 24h * 10^8
    pub turnover_24h_e8: u64,
    // Total volume * 10^8
    pub total_volume: u64,
    // Trading volume in the last 24 hours
    pub volume_24h: u64,
    // Funding rate * 10^6
    pub funding_rate_e6: i32,
    // Predicted funding rate * 10^6
    pub predicted_funding_rate_e6: i32,
    // Cross sequence (internal value)
    pub cross_seq: u64,
    // Creation time (when the order_status was Created)
    pub created_at: &'a str,
    // Update time
    pub updated_at: &'a str,
    // Next settlement time of capital cost
    pub next_funding_time: &'a str,
    // Countdown of settlement capital cost
    pub countdown_hour: u32,
    // funding rate time interval, unit hour
    pub funding_rate_interval: u32,
    pub settle_time_e9: u64,
    pub delisting_status: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct PerpetualInstrumentInfoDeltaItem<'a> {
    // id
    pub id: u64,
    // Symbol
    pub symbol: &'a str,
    pub last_tick_direction: Option<&'a str>,
    pub last_price_e4: Option<u64>,
    pub last_price: Option<&'a str>,
    pub bid1_price_e4: Option<u64>,
    pub bid1_price: Option<&'a str>,
    pub ask1_price_e4: Option<u64>,
    pub ask1_price: Option<&'a str>,
    pub price_24h_pcnt_e6: Option<i64>,
    pub price_1h_pcnt_e6: Option<i64>,
    pub mark_price_e4: Option<u64>,
    pub mark_price: Option<&'a str>,
    pub total_turnover_e8: Option<u64>,
    pub turnover_24h_e8: Option<u64>,
    pub total_volume: Option<u64>,
    pub volume_24h: Option<u64>,
    pub cross_seq: u64,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct PerpetualInstrumentInfoDelta<'a> {
    #[serde(borrow)]
    pub delete: Vec<PerpetualInstrumentInfoDeltaItem<'a>>,
    pub update: Vec<PerpetualInstrumentInfoDeltaItem<'a>>,
    pub insert: Vec<PerpetualInstrumentInfoDeltaItem<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct FuturesInstrumentInfoSnapshot<'a> {
    // id
    pub id: u64,
    // Symbol
    pub symbol: &'a str,
    // Symbol name alias
    pub symbol_name: &'a str,
    // The year of symbol
    pub symbol_year: u32,
    // Contract type
    pub contract_type: &'a str,
    // Coin type
    pub coin: &'a str,
    // Quote symbol
    pub quote_symbol: &'a str,
    // Position Mode. 0: One-Way Mode; 3: Hedge Mode
    pub mode: &'a str,
    // Support float profit open position or not
    pub is_up_borrowable: i32,
    // Symbol import timestamp * 10^9
    pub import_time_e9: u64,
    // Enable trading timestamp for symbol * 10^9
    pub start_trading_time_e9: u64,
    // Rest time until settled in seconds
    pub time_to_settle: u64,
    // Delivery timestamp * 10^9
    pub settle_time_e9: u64,
    // Delivery fee rate * 10^8
    pub settle_fee_rate_e8: i32,
    // Contract status
    pub contract_status: &'a str,
    // Quantity of subsidy from trading platform in BTC * 10^8
    pub system_subsidy_e8: u64,
    // Latest transaction price
    pub last_price_e4: u64,
    // Latest transaction price
    pub last_price: &'a str,
    // Direction of price change
    pub last_tick_direction: &'a str,
    pub bid1_price_e4: u64,
    // Best bid price
    pub bid1_price: &'a str,
    pub ask1_price_e4: u64,
    // Best ask price
    pub ask1_price: &'a str,
    pub prev_price_24h_e4: u64,
    // Price of 24 hours ago
    pub prev_price_24h: &'a str,
    // Percentage change of market price relative to 24h * 10^4
    pub price_24h_pcnt_e6: i64,
    pub high_price_24h_e4: u64,
    // The highest price in the last 24 hours
    pub high_price_24h: &'a str,
    pub low_price_24h_e4: u64,
    // Lowest price in the last 24 hours
    pub low_price_24h: &'a str,
    pub prev_price_1h_e4: u64,
    // Hourly market price an hour ago
    pub prev_price_1h: &'a str,
    pub price_1h_pcnt_e6: i64,
    pub mark_price_e4: u64,
    // Mark price
    pub mark_price: &'a str,
    pub index_price_e4: u64,
    // Index_price
    pub index_price: &'a str,
    // Open interest. The update is not immediate - slowest update is 1 minute
    pub open_interest: u64,
    pub open_value_e8: u64,
    // Total turnover
    pub total_turnover_e8: u64,
    // Turnover for 24h * 10^8
    pub turnover_24h_e8: u64,
    pub fair_basis_e8: u64,
    pub fair_basis_rate_e8: u64,
    pub basis_in_year_e8: u64,
    pub expect_price_e4: u64,
    // Total volume * 10^8
    pub total_volume: u64,
    // Trading volume in the last 24 hours
    pub volume_24h: u64,
    // Cross sequence (internal value)
    pub cross_seq: u64,
    // Creation time (when the order_status was Created)
    pub created_at_e9: u64,
    // Update time
    pub updated_at_e9: u64,
}

#[derive(Deserialize, Debug)]
pub struct FuturesInstrumentInfoDeltaItem<'a> {
    // id
    pub id: u64,
    // Symbol
    pub symbol: &'a str,
    pub symbol_name: &'a str,
    pub symbol_year: u32,
    pub contract_type: &'a str,
    pub coin: &'a str,
    pub quote_symbol: &'a str,
    pub mode: &'a str,
    pub start_trading_time_e9: u64,
    pub time_to_settle: u64,
    pub settle_time_e9: u64,

    pub is_up_borrowable: Option<i32>,
    pub settle_fee_rate_e8: Option<i32>,
    pub import_time_e9: Option<u64>,
    pub contract_status: Option<&'a str>,
    pub system_subsidy_e8: Option<u64>,
    pub last_price_e4: Option<u64>,
    // Latest transaction price
    pub last_price: Option<&'a str>,
    // Direction of price change
    pub last_tick_direction: Option<&'a str>,
    pub bid1_price_e4: Option<u64>,
    // Best bid price
    pub bid1_price: Option<&'a str>,
    pub ask1_price_e4: Option<u64>,
    // Best ask price
    pub ask1_price: Option<&'a str>,
    pub prev_price_24h_e4: Option<u64>,
    // Price of 24 hours ago
    pub prev_price_24h: Option<&'a str>,
    // Percentage change of market price relative to 24h * 10^4
    pub price_24h_pcnt_e6: Option<i64>,
    pub high_price_24h_e4: Option<u64>,
    // The highest price in the last 24 hours
    pub high_price_24h: Option<&'a str>,
    pub low_price_24h_e4: Option<u64>,
    // Lowest price in the last 24 hours
    pub low_price_24h: Option<&'a str>,
    pub prev_price_1h_e4: Option<u64>,
    // Hourly market price an hour ago
    pub prev_price_1h: Option<&'a str>,
    pub price_1h_pcnt_e6: Option<i64>,
    pub mark_price_e4: Option<u64>,
    // Mark price
    pub mark_price: Option<&'a str>,
    pub index_price_e4: Option<u64>,
    // Index_price
    pub index_price: Option<&'a str>,
    // Open interest. The update is not immediate - slowest update is 1 minute
    pub open_interest: Option<u64>,
    pub open_value_e8: Option<u64>,
    // Total turnover
    pub total_turnover_e8: Option<u64>,
    // Turnover for 24h * 10^8
    pub turnover_24h_e8: Option<u64>,
    pub fair_basis_e8: Option<u64>,
    pub fair_basis_rate_e8: Option<u64>,
    pub basis_in_year_e8: Option<u64>,
    pub expect_price_e4: Option<u64>,
    // Total volume * 10^8
    pub total_volume: Option<u64>,
    // Trading volume in the last 24 hours
    pub volume_24h: Option<u64>,
    // Cross sequence (internal value)
    pub cross_seq: Option<u64>,
    // Creation time (when the order_status was Created)
    pub created_at_e9: Option<u64>,
    // Update time
    pub updated_at_e9: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct FuturesInstrumentInfoDelta<'a> {
    #[serde(borrow)]
    pub update: Vec<FuturesInstrumentInfoDeltaItem<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct Kline {
    // Start timestamp point for result, in seconds
    pub start: u64,
    // End timestamp point for result, in seconds
    pub end: u64,
    // Starting price
    pub open: f64,
    // Closing price
    pub close: f64,
    // Maximum price
    pub high: f64,
    // Minimum price
    pub low: f64,
    // Trading volume
    pub volume: f64,
    // Turnover
    pub turnover: f64,
    // Is confirm
    pub confirm: bool,
    // Cross sequence (internal value)
    pub cross_seq: u64,
    // End timestamp point for result, in seconds
    pub timestamp: u64,
}

#[derive(Deserialize, Debug)]
pub struct Liquidation<'a> {
    // Symbol
    pub symbol: &'a str,
    // Liquidated position's side
    pub side: &'a str,
    // Bankruptcy price
    pub price: &'a str,
    // Order quantity
    pub qty: &'a str,
    // Millisecond timestamp
    pub time: u64,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PublicResponse<'a> {
    #[serde(borrow)]
    OrderBookL2Snapshot(BaseResponseWithTimestamp<'a, Vec<OrderBookItem<'a>>>),
    OrderBookL2Delta(BaseResponseWithTimestamp<'a, OrderBookDelta<'a>>),
    Trade(BaseResponse<'a, Vec<Trade<'a>>>),
    Insurance(BaseResponse<'a, Vec<Insurance<'a>>>),
    PerpetualInstrumentInfoSnapshot(Response<'a, PerpetualInstrumentInfoSnapshot<'a>>),
    PerpetualInstrumentInfoDelta(Response<'a, PerpetualInstrumentInfoDelta<'a>>),
    FuturesInstrumentInfoSnapshot(Response<'a, FuturesInstrumentInfoSnapshot<'a>>),
    FuturesInstrumentInfoDelta(Response<'a, FuturesInstrumentInfoDelta<'a>>),
    Kline(BaseResponseWithTimestamp<'a, Vec<Kline>>),
    Liquidation(BaseResponse<'a, Liquidation<'a>>),
}

pub struct OnPublicResponse;
impl<'a> On<'a> for OnPublicResponse {
    type Arg = PublicResponse<'a>;
}

pub struct PublicWebSocketApiClient {
    pub uri: String,
    subscriptions: Vec<String>,
}

#[derive(Serialize, Debug)]
struct Subscription {
    op: String,
    args: Vec<String>,
}

pub struct PublicWebSocketApiClientBuilder {
    uri: String,
}

impl Default for PublicWebSocketApiClientBuilder {
    fn default() -> Self {
        Self {
            uri: MAINNET.to_owned(),
        }
    }
}

impl PublicWebSocketApiClientBuilder {
    pub fn testnet(mut self) -> Self {
        self.uri = TESTNET.to_owned();
        self
    }

    pub fn uri<S: AsRef<str>>(mut self, uri: S) -> Self {
        self.uri = uri.as_ref().to_owned();
        self
    }

    pub fn build(self) -> PublicWebSocketApiClient {
        PublicWebSocketApiClient {
            uri: self.uri,
            subscriptions: Vec::new(),
        }
    }
}

impl PublicWebSocketApiClient {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> PublicWebSocketApiClientBuilder {
        PublicWebSocketApiClientBuilder::default()
    }

    pub fn subscribe_order_book_l2_25<S: AsRef<str>>(&mut self, symbols: &[S]) {
        self.subscribe("orderBookL2_25", symbols);
    }

    pub fn subscribe_order_book_l2_200<S: AsRef<str>>(&mut self, symbols: &[S]) {
        self.subscribe("orderBook_200.100ms", symbols);
    }

    pub fn subscribe_trade<S: AsRef<str>>(&mut self, symbols: &[S]) {
        self.subscribe("trade", symbols);
    }

    pub fn subscribe_insurance<S: AsRef<str>>(&mut self, symbols: &[S]) {
        self.subscribe("insurance", symbols);
    }

    pub fn subscribe_instrument_info<S: AsRef<str>>(&mut self, symbols: &[S]) {
        self.subscribe("instrument_info.100ms", symbols);
    }

    pub fn subscribe_kline<S: AsRef<str>>(&mut self, symbols: &[S], interval: &str) {
        let topic = format!("klineV2.{}", interval);
        self.subscribe(&topic, symbols);
    }

    pub fn subscribe_liquidation<S: AsRef<str>>(&mut self, symbols: &[S]) {
        self.subscribe("liquidation", symbols);
    }

    pub fn run<T: Callback<OnPublicResponse>>(&self, callback: T) -> Result<()> {
        run(&self.uri, None, &self.subscriptions, future_ping, callback)
    }

    fn subscribe<S>(&mut self, topic: &str, symbols: &[S])
    where
        S: AsRef<str>,
    {
        let args = symbols
            .iter()
            .map(|symbol| format!("{}.{}", topic, symbol.as_ref()))
            .collect();
        let subscription = Subscription {
            op: "subscribe".into(),
            args,
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }
}

#[derive(Deserialize, Debug)]
pub struct BasePrivateResponse<'a, D> {
    pub topic: &'a str,
    pub data: Vec<D>,
}

#[derive(Deserialize, Debug)]
pub struct Position<'a> {
    /// UserID
    pub user_id: u32,
    /// Symbol
    pub symbol: &'a str,
    /// Position qty
    pub size: u32,
    /// Side
    // TODO: improve by parsing to an enum
    pub side: &'a str,
    /// Position value
    pub position_value: &'a str,
    /// Average entry price
    pub entry_price: &'a str,
    /// Liquidation price
    pub liq_price: &'a str,
    /// Bankruptcy price
    pub bust_price: &'a str,
    /// In Isolated Margin mode, the value is set by the user. In Cross Margin mode, the value is the max leverage at current risk level
    pub leverage: &'a str,
    /// Pre-occupied order margin
    pub order_margin: &'a str,
    /// Position margin
    pub position_margin: &'a str,
    /// Available balance = wallet balance - used margin
    pub available_balance: &'a str,
    /// Take profit price
    pub take_profit: &'a str,
    /// Stop loss price
    pub stop_loss: &'a str,
    /// Today's realised pnl
    pub realised_pnl: &'a str,
    /// Trailing stop (the distance from the current price)
    pub trailing_stop: &'a str,
    /// Trailing stop active price
    pub trailing_active: &'a str,
    /// Wallet balance
    pub wallet_balance: &'a str,
    /// [Risk ID](https://bybit-exchange.github.io/docs/inverse/#t-getrisklimit)
    pub risk_id: u32,
    /// Position closing fee occupied (your opening fee + expected maximum closing fee)
    pub occ_closing_fee: &'a str,
    /// Pre-occupied funding fee: calculated from position qty and current funding fee
    pub occ_funding_fee: &'a str,
    /// Whether or not auto-margin replenishment is enabled
    pub auto_add_margin: u32,
    /// Accumulated realised pnl (all-time total)
    pub cum_realised_pnl: &'a str,
    /// Position status: `Normal`, `Liq`, `Adl`
    pub position_status: &'a str,
    /// Position sequence
    pub position_seq: u32,
    /// true means isolated margin mode; false means cross margin mode
    #[serde(alias = "Isolated")]
    pub isolated: bool,
    /// Position mode
    pub mode: u32,
    /// 0-One-Way Mode, 1-Buy side of both side mode, 2-Sell side of both side mode (Perpetual)
    pub position_idx: u32,
    /// TrailingProfit or StopLoss mode Full or Partial
    pub tp_sl_mode: &'a str,
    /// The qty of take profit orders
    pub tp_order_num: u32,
    /// The qty of stop loss orders
    pub sl_order_num: u32,
    /// The available size to set take profit
    pub tp_free_size_x: u32,
    /// The available size to set stop loss
    pub sl_free_size_x: u32,
}

#[derive(Deserialize, Debug)]
pub struct Execution<'a> {
    /// Symbol
    pub symbol: &'a str,
    /// Side    
    pub side: &'a str,
    /// Order ID    
    pub order_id: &'a str,
    /// Transaction ID    
    pub exec_id: &'a str,
    /// Unique user-set order ID. Maximum length of 36 characters    
    pub order_link_id: &'a str,
    /// Transaction price    
    pub price: &'a str,
    /// Order qty    
    pub order_qty: u32,
    /// Execution type (cannot be Funding)    
    pub exec_type: &'a str,
    /// Transaction qty    
    pub exec_qty: u32,
    /// Transaction fee    
    pub exec_fee: &'a str,
    /// Number of unfilled contracts from the order's size    
    pub leaves_qty: u32,
    /// Is maker    
    pub is_maker: bool,
    /// Transaction timestamp    
    pub trade_time: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct Order<'a> {
    /// Order ID
    pub order_id: &'a str,
    /// Unique user-set order ID. Maximum length of 36 characters
    pub order_link_id: &'a str,
    /// Symbol
    pub symbol: &'a str,
    /// Side
    pub side: &'a str,
    /// Conditional order type
    pub order_type: &'a str,
    /// Order price
    pub price: &'a str,
    /// Transaction qty
    pub qty: u32,
    /// Time in force
    pub time_in_force: &'a str,
    /// Trigger scenario for single action
    pub create_type: &'a str,
    /// Trigger scenario for cancel operation
    pub cancel_type: &'a str,
    /// Order status
    pub order_status: &'a str,
    /// Number of unfilled contracts from the order's size
    pub leaves_qty: u32,
    /// Cumulative qty of trading
    pub cum_exec_qty: u32,
    /// Cumulative value of trading
    pub cum_exec_value: &'a str,
    /// Cumulative trading fees
    pub cum_exec_fee: &'a str,
    /// Timestamp (when the order_status was New)
    pub timestamp: &'a str,
    /// Take profit price
    pub take_profit: &'a str,
    /// Take profit trigger price type
    pub tp_trigger_by: Option<&'a str>,
    /// Stop loss price
    pub stop_loss: &'a str,
    /// Stop loss trigger price type
    pub sl_trigger_by: Option<&'a str>,
    /// Trailing stop (the distance from the current price)
    pub trailing_stop: &'a str,
    /// Last execution price
    pub last_exec_price: &'a str,
    /// What is a reduce-only order? True means your position can only reduce in size if this order is triggered
    pub reduce_only: bool,
    /// What is a close on trigger order? For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.
    pub close_on_trigger: bool,
}

#[derive(Deserialize, Debug)]
pub struct StopOrder<'a> {
    /// Order ID
    pub order_id: &'a str,
    /// Unique user-set order ID. Maximum length of 36 characters
    pub order_link_id: &'a str,
    /// UserID
    pub user_id: u32,
    /// Symbol
    pub symbol: &'a str,
    /// Order type
    pub order_type: &'a str,
    /// Side
    pub side: &'a str,
    /// Order price
    pub price: &'a str,
    /// Order quantity in USD
    pub qty: u32,
    /// Time in force
    pub time_in_force: &'a str,
    /// Trigger scenario for single action
    pub create_type: &'a str,
    /// Trigger scenario for cancel operation
    pub cancel_type: &'a str,
    /// Order status
    pub order_status: &'a str,
    /// Conditional order type
    pub stop_order_type: &'a str,
    /// Trigger price type. Default LastPrice
    pub trigger_by: &'a str,
    /// If stop_order_type is TrailingProfit, this field is the trailing stop active price.
    pub trigger_price: &'a str,
    /// What is a close on trigger order? For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.
    pub close_on_trigger: bool,
    /// UTC time
    pub timestamp: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct Wallet<'a> {
    /// User ID
    pub user_id: u32,
    /// Coin type
    pub coin: &'a str,
    /// Wallet balance
    pub wallet_balance: &'a str,
    /// In Isolated Margin Mode:
    ///
    /// ````
    /// available_balance = wallet_balance - (position_margin + occ_closing_fee + occ_funding_fee + order_margin)
    /// ````
    ///
    /// In Cross Margin Mode:
    ///
    /// ````
    /// if unrealised_pnl > 0:
    ///      available_balance = wallet_balance - (position_margin + occ_closing_fee + occ_funding_fee + order_margin)
    ///  if unrealised_pnl < 0:
    ///      available_balance = wallet_balance - (position_margin + occ_closing_fee + occ_funding_fee + order_margin) + unrealised_pnl
    /// ````
    pub available_balance: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PrivateResponse<'a> {
    #[serde(borrow)]
    Position(BasePrivateResponse<'a, Position<'a>>),
    Execution(BasePrivateResponse<'a, Execution<'a>>),
    Order(BasePrivateResponse<'a, Order<'a>>),
    StopOrder(BasePrivateResponse<'a, StopOrder<'a>>),
    Wallet(BasePrivateResponse<'a, Wallet<'a>>),
}

pub struct OnPrivateResponse;
impl<'a> On<'a> for OnPrivateResponse {
    type Arg = PrivateResponse<'a>;
}

#[derive(Serialize)]
struct AuthReq<'a> {
    op: &'a str,
    args: [&'a str; 3],
}

pub struct PrivateWebSocketApiClientBuilder {
    uri: String,
}

impl Default for PrivateWebSocketApiClientBuilder {
    fn default() -> Self {
        Self {
            uri: MAINNET.to_owned(),
        }
    }
}

impl PrivateWebSocketApiClientBuilder {
    pub fn testnet(mut self) -> Self {
        self.uri = TESTNET.to_owned();
        self
    }

    pub fn uri<S: AsRef<str>>(mut self, uri: S) -> Self {
        self.uri = uri.as_ref().to_owned();
        self
    }

    pub fn build_with_credentials<S: AsRef<str>>(
        self,
        api_key: S,
        secret: S,
    ) -> PrivateWebSocketApiClient {
        PrivateWebSocketApiClient {
            uri: self.uri,
            subscriptions: Vec::new(),
            api_key: api_key.as_ref().to_owned(),
            secret: secret.as_ref().to_owned(),
        }
    }
}

pub struct PrivateWebSocketApiClient {
    pub uri: String,
    pub api_key: String,
    pub secret: String,
    subscriptions: Vec<String>,
}

impl PrivateWebSocketApiClient {
    pub fn new<S: AsRef<str>>(api_key: S, secret: S) -> Self {
        Self::builder()
            .build_with_credentials(api_key.as_ref().to_owned(), secret.as_ref().to_owned())
    }

    pub fn builder() -> PrivateWebSocketApiClientBuilder {
        PrivateWebSocketApiClientBuilder::default()
    }

    pub fn subscribe_position(&mut self) {
        let subscription = Subscription {
            op: "subscribe".into(),
            args: vec!["position".into()],
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_execution(&mut self) {
        let subscription = Subscription {
            op: "subscribe".into(),
            args: vec!["execution".into()],
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_order(&mut self) {
        let subscription = Subscription {
            op: "subscribe".into(),
            args: vec!["order".into()],
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_stop_order(&mut self) {
        let subscription = Subscription {
            op: "subscribe".into(),
            args: vec!["stop_order".into()],
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_wallet(&mut self) {
        let subscription = Subscription {
            op: "subscribe".into(),
            args: vec!["wallet".into()],
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn run<T: Callback<OnPrivateResponse>>(&self, callback: T) -> Result<()> {
        let credentials = Credentials {
            api_key: self.api_key.clone(),
            secret: self.secret.clone(),
        };
        run(
            &self.uri,
            Some(&credentials),
            &Vec::new(),
            future_ping,
            callback,
        )
    }
}
