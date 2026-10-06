// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! OKX-specific custom data types.
//!
//! These types carry OKX domain data through the Nautilus data engine as
//! [`CustomData`](nautilus_model::data::CustomData).

use nautilus_core::UnixNanos;
use nautilus_model::{custom_data, identifiers::InstrumentId};
use rust_decimal::Decimal;

#[cfg(feature = "arrow")]
use nautilus_serialization::arrow_custom_data;

use crate::websocket::messages::OKXTickerMsg;

/// OKX 24-hour ticker snapshot from the `tickers` channel.
#[cfg_attr(feature = "arrow", arrow_custom_data)]
#[custom_data]
pub struct OKXTicker {
    /// The instrument for these 24-hour statistics.
    pub instrument_id: InstrumentId,
    /// Last traded price.
    pub last_price: Decimal,
    /// Size of the last trade, in contracts.
    pub last_size: Decimal,
    /// 24-hour opening price.
    pub open_price: Decimal,
    /// 24-hour highest price.
    pub high_price: Decimal,
    /// 24-hour lowest price.
    pub low_price: Decimal,
    /// 24-hour trading volume in contracts.
    pub volume: Decimal,
    /// 24-hour trading volume in the base currency.
    pub volume_currency: Decimal,
    /// Best bid price.
    pub bid_price: Decimal,
    /// Best ask price.
    pub ask_price: Decimal,
    /// Best bid size.
    pub bid_size: Decimal,
    /// Best ask size.
    pub ask_size: Decimal,
    /// UNIX timestamp (nanoseconds) when the ticker event occurred.
    pub ts_event: UnixNanos,
    /// UNIX timestamp (nanoseconds) when the instance was initialized.
    pub ts_init: UnixNanos,
}

impl OKXTicker {
    /// Parses an [`OKXTicker`] from an OKX ticker message.
    ///
    /// Empty decimal fields (venues occasionally omit values for fresh
    /// listings) are treated as zero.
    ///
    /// # Errors
    ///
    /// Returns an error if a non-empty decimal field cannot be parsed.
    pub fn from_msg(
        msg: &OKXTickerMsg,
        instrument_id: InstrumentId,
        ts_init: UnixNanos,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            instrument_id,
            last_price: parse_decimal(&msg.last_px)?,
            last_size: parse_decimal(&msg.last_sz)?,
            open_price: parse_decimal(&msg.open24h)?,
            high_price: parse_decimal(&msg.high24h)?,
            low_price: parse_decimal(&msg.low24h)?,
            volume: parse_decimal(&msg.vol24h)?,
            volume_currency: parse_decimal(&msg.vol_ccy_24h)?,
            bid_price: parse_decimal(&msg.bid_px)?,
            ask_price: parse_decimal(&msg.ask_px)?,
            bid_size: parse_decimal(&msg.bid_sz)?,
            ask_size: parse_decimal(&msg.ask_sz)?,
            ts_event: UnixNanos::from(msg.ts.saturating_mul(1_000_000)),
            ts_init,
        })
    }
}

fn parse_decimal(value: &str) -> anyhow::Result<Decimal> {
    if value.is_empty() {
        return Ok(Decimal::ZERO);
    }
    value
        .parse::<Decimal>()
        .map_err(|e| anyhow::anyhow!("invalid decimal `{value}`: {e}"))
}

/// Registers OKX custom data types.
///
/// Safe to call multiple times (idempotent via internal `Once` guards).
pub fn register_okx_custom_data() {
    #[cfg(feature = "arrow")]
    {
        nautilus_serialization::ensure_custom_data_registered::<OKXTicker>();
    }

    #[cfg(not(feature = "arrow"))]
    {
        let _ = nautilus_model::data::ensure_custom_data_json_registered::<OKXTicker>();
    }
}
