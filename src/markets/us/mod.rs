//! United States market calendars (Settlement, NYSE, Fed, etc.).

use chrono::Weekday;

use crate::calendar::Calendar;
use crate::date::Date;

pub mod fed;
pub mod gov_bond;
pub mod nerc;
pub mod nyse;
pub mod settlement;
pub mod sofr;

use fed::US_FED;
use gov_bond::US_GOVERNMENT_BOND;
use nerc::US_NERC;
use nyse::US_NYSE;
use settlement::US_SETTLEMENT;
use sofr::US_SOFR;

//
//  United States markets
//

/// US market variants supported by `UnitedStates`.
#[derive(Debug, Clone, Copy)]
pub enum USMarket {
    Settlement,
    LiborImpact,
    NYSE,
    GovernmentBond,
    SOFR,
    NERC,
    FederalReserve,
}

/// United States calendar wrapper.
/// Uses underlying `StaticCalendar`s and adds a bit of market-specific logic.
#[derive(Debug, Clone, Copy)]
pub struct UnitedStates {
    pub market: USMarket,
}

impl UnitedStates {
    #[inline]
    pub fn new(market: USMarket) -> Self {
        Self { market }
    }
}

impl Calendar for UnitedStates {
    fn is_business_day(&self, date: Date) -> bool {
        match self.market {
            USMarket::Settlement => US_SETTLEMENT.is_business_day(date),

            USMarket::LiborImpact => {
                // LiborImpact: same as Settlement, but since 2015
                // Independence Day only impacts Libor if the actual
                // July 4 falls on a weekday.
                let m = date.month();
                let d = date.day();
                let y = date.year();
                let w = date.weekday();

                // if it's the Friday/Monday *around* July 4 and >=2015,
                // treat as business day even if Settlement says holiday.
                if y >= 2015
                    && m == 7
                    && ((d == 5 && w == Weekday::Mon) || (d == 3 && w == Weekday::Fri))
                {
                    return true;
                }

                US_SETTLEMENT.is_business_day(date)
            }

            USMarket::NYSE => US_NYSE.is_business_day(date),

            USMarket::GovernmentBond => US_GOVERNMENT_BOND.is_business_day(date),

            USMarket::SOFR => US_SOFR.is_business_day(date),

            USMarket::NERC => US_NERC.is_business_day(date),

            USMarket::FederalReserve => US_FED.is_business_day(date),
        }
    }
}
