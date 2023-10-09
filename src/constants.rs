// User minimum balance
// here we set a rule that the user must have a minimum balance of 1.5 XLM.
// 1 XLM for Stellar network minimum requirement + 0.5 XLM (or more) for operation expenses.
// 1.5 XLM is equal to 15_000_000 stroops
pub const MINIMUM_USER_BALANCE: i128 = 15_000_000;

// Ledger approximate number for certain period.
// Assuming ledger closed time is 5 seconds/ledger.
//
pub const LEDGER_AMOUNT_IN_12_HOURS: u32 = 8640;
pub const LEDGER_AMOUNT_IN_24_HOURS: u32 = 17280;
pub const LEDGER_AMOUNT_IN_3_DAYS: u32 = 51840;
pub const LEDGER_AMOUNT_IN_1_WEEK: u32 = 120960;
pub const LEDGER_AMOUNT_IN_1_MONTH: u32 = 483840;

// Cow price based on breed (in XLM unit).
//
pub const JERSEY_PRICE: i128 = 1000;
pub const LIMOUSIN_PRICE: i128 = 1000;
pub const HALLIKAR_PRICE: i128 = 1000;
pub const HEREFORD_PRICE: i128 = 5000;
pub const HOLSTEIN_PRICE: i128 = 15000;
pub const SIMMENTAL_PRICE: i128 = 15000;

// Cow feeding stats multiplier, with 2 digit decimal precision.
// For every feeding event, it will give you:
// 0.5% rewards when ON_TIME -- 50 (0.5 x 100)
// 0.25% rewards when LATE -- 25 (0.25 x 100)
// 1% fines when FORGET -- 100 (1 x 100)
// 100% equivalent to 10_000
//
pub const ON_TIME_REWARD: i128 = 50;
pub const LATE_REWARD: i128 = 25;
pub const FORGET_FINE: i128 = 100;
pub const PRECISION_100_PERCENT: i128 = 10_000;

// Cow feeding ledger limit.
//
pub const WELL_FED: u32 = 4320;
pub const ON_TIME_FEED: u32 = 8640;
pub const LATE_FEED: u32 = 12960;
