use soroban_sdk::{Address, BytesN, Env, String, Symbol};

use crate::enums::*;
use crate::types::*;

pub trait CowContractTrait {
    /// [CowContract::init]
    /// Initialize the Cowchain Farm contract
    fn init(env: Env, admin: Address, native_token: Address, message: String) -> Status;

    /// [CowContract::upgrade]
    /// Upgrade the Cowchain Farm contract
    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Status;

    /// [CowContract::bump_instance]
    /// Bump the Cowchain Farm instance storage
    fn bump_instance(env: Env, ledger_amount: u32) -> Status;

    /// [CowContract::health_check]
    /// Health check for Cowchain Farm contract
    fn health_check(env: Env) -> CowStatus;

    /// [CowContract::buy_cow]
    /// Buy Cow from supplier
    fn buy_cow(
        env: Env,
        user: Address,
        cow_name: Symbol,
        cow_id: String,
        cow_breed: CowBreed,
    ) -> BuyCowResult;

    /// [CowContract::sell_cow]
    /// Sell Cow to supplier
    fn sell_cow(env: Env, user: Address, cow_id: String) -> SellCowResult;

    /// [CowContract::cow_appraisal]
    /// Cow appraisal to get market value
    fn cow_appraisal(env: Env, cow_id: String) -> CowAppraisalResult;

    /// [CowContract::feed_the_cow]
    /// Feed the cow in Cowchain Farm
    fn feed_the_cow(env: Env, user: Address, cow_id: String) -> CowStatus;

    /// [CowContract::get_all_cow]
    /// Retrieve all cow data listed in ownership
    fn get_all_cow(env: Env, user: Address) -> GetAllCowResult;
}
