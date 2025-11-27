use crate::calendar::Calendar;
use crate::date::Date;

pub mod exchange;
pub mod settlement;

use exchange::FR_EXCHANGE;
use settlement::FR_SETTLEMENT;

// France market variants (simplified QuantLib-style): Settlement and Exchange.
#[derive(Debug, Clone, Copy)]
pub enum FRMarket {
    Settlement,
    Exchange,
}

#[derive(Debug, Clone, Copy)]
pub struct France {
    pub market: FRMarket,
}

impl France {
    #[inline]
    pub fn new(market: FRMarket) -> Self {
        Self { market }
    }
}

impl Calendar for France {
    fn is_business_day(&self, date: Date) -> bool {
        match self.market {
            FRMarket::Settlement => FR_SETTLEMENT.is_business_day(date),
            FRMarket::Exchange => FR_EXCHANGE.is_business_day(date),
        }
    }
}
