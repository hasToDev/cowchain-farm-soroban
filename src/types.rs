use crate::enums::{CowBreed, Status};
use soroban_sdk::{contracttype, symbol_short, Env, String, Symbol, Vec};

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
    pub cow_data: CowData,
    pub ownership: Vec<String>,
}

impl BuyCowResult {
    pub fn default(env: Env, return_status: Status) -> Self {
        Self {
            status: return_status,
            cow_data: CowData::default(env.clone()),
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
    pub fn default(env: Env, return_status: Status) -> Self {
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
    pub fn default(return_status: Status) -> Self {
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
}

impl CowData {
    pub fn default(env: Env) -> Self {
        Self {
            id: String::from_slice(&env, ""),
            name: symbol_short!(""),
            breed: CowBreed::Jersey,
            born_ledger: 0,
            last_fed_ledger: 0,
            feeding_stats: CowFeedingStats::default(),
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CowFeedingStats {
    pub on_time: u32,
    pub late: u32,
    pub forget: u32,
}

impl CowFeedingStats {
    pub fn default() -> Self {
        Self {
            on_time: 0,
            late: 0,
            forget: 0,
        }
    }
}
