use crate::enums::{CowBreed, Status};
use soroban_sdk::{contracttype, symbol_short, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CowStatus {
    pub status: Status,
    pub ledger: u32,
}

impl CowStatus {
    pub fn new(env: Env, return_status: Status) -> Self {
        Self {
            status: return_status,
            ledger: env.ledger().sequence(),
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct BuyCowResult {
    pub status: Status,
    pub cow_data: Vec<CowData>,
    pub ownership: Vec<String>,
}

impl BuyCowResult {
    pub fn new(env: Env, return_status: Status) -> Self {
        Self {
            status: return_status,
            cow_data: Vec::new(&env),
            ownership: Vec::new(&env),
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SellCowResult {
    pub status: Status,
    pub ownership: Vec<String>,
}

impl SellCowResult {
    pub fn new(env: Env, return_status: Status) -> Self {
        Self {
            status: return_status,
            ownership: Vec::new(&env),
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CowAppraisalResult {
    pub status: Status,
    pub price: i128,
}

impl CowAppraisalResult {
    pub fn new(return_status: Status) -> Self {
        Self {
            status: return_status,
            price: 0,
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct GetAllCowResult {
    pub status: Status,
    pub data: Vec<CowData>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CowData {
    pub id: String,
    pub name: Symbol,
    pub breed: CowBreed,
    pub born_ledger: u32,
    pub last_fed_ledger: u32,
    pub feeding_stats: CowFeedingStats,
    pub auction_id: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CowFeedingStats {
    pub on_time: u32,
    pub late: u32,
    pub forget: u32,
}

impl CowFeedingStats {
    pub fn new() -> Self {
        Self {
            on_time: 0,
            late: 0,
            forget: 0,
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CowEventDetails {
    pub id: String,
    pub name: Symbol,
    pub owner: Address,
    pub last_fed_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct AuctionData {
    pub auction_id: String,
    pub cow_id: String,
    pub cow_name: Symbol,
    pub cow_breed: CowBreed,
    pub cow_born_ledger: u32,
    pub owner: Address,
    pub start_price: i128,
    pub highest_bidder: Bidder,
    pub bid_history: Vec<Bidder>,
    pub auction_limit_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Bidder {
    pub user: Address,
    pub price: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct AuctionEventDetails {
    pub auction_id: String,
    pub bidder: Address,
    pub price: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct AuctionResult {
    pub status: Status,
    pub auction_data: Vec<AuctionData>,
}

impl AuctionResult {
    pub fn default(env: Env, return_status: Status) -> Self {
        Self {
            status: return_status,
            auction_data: Vec::new(&env),
        }
    }
}
