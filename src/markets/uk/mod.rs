//! UK market calendars (LSE, UK settlement, etc.).

use crate::calendar::Calendar;
use crate::date::Date;

mod settlement;
mod exchange;

pub use settlement::UK_SETTLEMENT;
pub use exchange::UK_EXCHANGE;

// United Kingdom market variants: Settlement, Exchange (LSE-style), and Metals.
#[derive(Debug, Clone, Copy)]
pub enum UKMarket {
    Settlement,
    Exchange,
    Metals,
}

#[derive(Debug, Clone, Copy)]
pub struct UnitedKingdom {
    pub market: UKMarket,
}

impl UnitedKingdom {
    #[inline]
    pub fn new(market: UKMarket) -> Self {
        Self { market }
    }
}


impl Calendar for UnitedKingdom {
    fn is_business_day(&self, date: Date) -> bool {
        match self.market {
            UKMarket::Settlement => UK_SETTLEMENT.is_business_day(date),
            UKMarket::Exchange => UK_EXCHANGE.is_business_day(date),
            // In QuantLib, the UK Metals calendar shares the same rules as the
            // ExchangeImpl. We mirror that here by reusing `UK_EXCHANGE`.
            UKMarket::Metals => UK_EXCHANGE.is_business_day(date),
        }
    }
}
