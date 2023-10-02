#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short, token, Address, BytesN, Env, String, Symbol, Vec,
};

use crate::constants::*;
use crate::enums::*;
use crate::interface::*;
use crate::types::*;

mod constants;
mod enums;
mod interface;
mod types;

#[contract]
pub struct CowContract;

#[contractimpl]
impl CowContractTrait for CowContract {
    fn init(env: Env, admin: Address, native_token: Address, message: String) -> Status {
        // check for initialization password.
        // you must set your own unique password other than "9p2Vx4Dr8wp365n7C5rB42xN9".
        // you can use the Deployer contract instead for this check.
        // the main purpose is to prevent other people from initializing your contract.
        let internal_password = String::from_slice(&env, "9p2Vx4Dr8wp365n7C5rB42xN9");
        if message.ne(&internal_password) {
            return Status::TryAgain;
        }
        // check admin key in storage.
        let is_admin_exist = env.storage().instance().has(&DataKey::Admin);
        if is_admin_exist {
            // if admin key exist, means that contract has been initialized.
            return Status::AlreadyInitialized;
        }
        // check admin authorization
        admin.require_auth();
        // save admin, native token, and record current initialization Ledger.
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::NativeToken, &native_token);
        env.storage()
            .instance()
            .set(&DataKey::InitializedLedger, &env.ledger().sequence());
        // bump storage instance lifetime to 1 month
        env.storage()
            .instance()
            .bump(LEDGER_AMOUNT_IN_1_MONTH, LEDGER_AMOUNT_IN_1_MONTH);
        Status::Ok
    }

    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Status {
        // check Admin key in storage.
        // if Admin key not exist, contract has not been initialized.
        let is_admin_exist = env.storage().instance().has(&DataKey::Admin);
        if !is_admin_exist {
            return Status::NotInitialized;
        }

        // load the Admin address and get its authorization.
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        // update the Cowchain Farm contract.
        env.deployer().update_current_contract_wasm(new_wasm_hash);
        Status::Upgraded
    }

    fn bump_instance(env: Env, ledger_amount: u32) -> Status {
        // check Admin key in storage.
        // if Admin key not exist, contract has not been initialized.
        let is_admin_exist = env.storage().instance().has(&DataKey::Admin);
        if !is_admin_exist {
            return Status::NotInitialized;
        }

        // load the Admin address and get its authorization.
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        // bump instance storage
        env.storage().instance().bump(ledger_amount, ledger_amount);
        Status::Bumped
    }

    fn health_check(env: Env) -> CowStatus {
        CowStatus {
            status: Status::Ok,
            ledger: env.ledger().sequence(),
        }
    }

    fn open_donation(env: Env, from: Address, amount: i128) -> Status {
        // check Admin key in storage.
        // if Admin key not exist, contract has not been initialized.
        let is_admin_exist = env.storage().instance().has(&DataKey::Admin);
        if !is_admin_exist {
            return Status::Fail;
        }

        // if Native Token key not exist, contract has not been initialized.
        let is_native_token_exist = env.storage().instance().has(&DataKey::NativeToken);
        if !is_native_token_exist {
            return Status::Fail;
        }

        from.require_auth();

        // initiate native token client.
        let native_token: Address = env.storage().instance().get(&DataKey::NativeToken).unwrap();
        let native_token_client = token::Client::new(&env, &native_token);

        // transfer native token from user to contract.
        let donation_amount = amount * 10_000_000;
        native_token_client.transfer(&from, &env.current_contract_address(), &donation_amount);

        Status::Ok
    }

    fn buy_cow(
        env: Env,
        user: Address,
        cow_name: Symbol,
        cow_id: String,
        cow_breed: CowBreed,
    ) -> BuyCowResult {
        // ensures that user has authorized invocation of this contract.
        user.require_auth();

        // if Native Token key not exist, contract has not been initialized.
        let is_native_token_exist = env.storage().instance().has(&DataKey::NativeToken);
        if !is_native_token_exist {
            return BuyCowResult::default(env, Status::NotInitialized);
        }

        // initiate native token client.
        let native_token: Address = env.storage().instance().get(&DataKey::NativeToken).unwrap();
        let native_token_client = token::Client::new(&env, &native_token);

        // get user balance.
        let user_native_token_balance: i128 = native_token_client.balance(&user);

        // here we set a rule that the user must have a minimum balance of 1.5 XLM.
        // 1 XLM for Stellar network minimum requirement + 0.5 XLM (or more) for operation expenses.
        // 1.5 XLM is equal to 15_000_000 stroops
        let minimum_user_balance: i128 = 15_000_000;

        // get cow price based on their breed (the price will be in stroops unit).
        let cow_price_in_stroops: i128 = get_cow_base_price_in_stroops(&cow_breed);

        // find out the approximate user balance after transaction.
        let user_balance_after_tx: i128 =
            user_native_token_balance - minimum_user_balance - cow_price_in_stroops.clone();

        // cancel the transaction if user balance after transaction equal or less than zero.
        if user_balance_after_tx <= 0 {
            return BuyCowResult::default(env, Status::InsufficientFund);
        }

        // transfer native token to supplier to complete the buying process.
        native_token_client.transfer(
            &user,
            &env.current_contract_address(),
            &cow_price_in_stroops,
        );

        // new cow data.
        let new_cow_data = CowData {
            id: cow_id.clone(),
            name: cow_name,
            breed: cow_breed,
            born_ledger: env.ledger().sequence(),
            last_fed_ledger: env.ledger().sequence(),
            feeding_stats: CowFeedingStats::default(),
            auction_id: String::from_slice(&env, "-"),
        };
        let mut cow_ownership_list: Vec<String> = Vec::new(&env);

        // if ownership data exist, append the data to ownership list.
        let is_owner_exist = env.storage().persistent().has(&user);
        if is_owner_exist {
            // get current ownership data.
            let ownership_data: Vec<String> = env.storage().persistent().get(&user).unwrap();
            cow_ownership_list.append(&ownership_data);
        }

        // save ownership data & bump lifetime to 1 week.
        cow_ownership_list.push_back(cow_id.clone());
        env.storage().persistent().set(&user, &cow_ownership_list);
        env.storage()
            .persistent()
            .bump(&user, LEDGER_AMOUNT_IN_1_WEEK, LEDGER_AMOUNT_IN_1_WEEK);

        // save cow data & bump lifetime to 24 hours.
        env.storage().temporary().set(&cow_id, &new_cow_data);
        env.storage().temporary().bump(
            &cow_id,
            LEDGER_AMOUNT_IN_24_HOURS,
            LEDGER_AMOUNT_IN_24_HOURS,
        );

        // publish Cowchain Farm BUY event
        let new_cow_event = CowEventDetails {
            id: new_cow_data.id.clone(),
            name: new_cow_data.name.clone(),
            owner: user,
            last_fed_ledger: env.ledger().sequence(),
        };
        env.events().publish((symbol_short!("buy"),), new_cow_event);

        BuyCowResult {
            status: Status::Ok,
            cow_data: new_cow_data,
            ownership: cow_ownership_list,
        }
    }

    fn sell_cow(env: Env, user: Address, cow_id: String) -> SellCowResult {
        // ensures that user has authorized invocation of this contract.
        user.require_auth();

        // if Native Token key not exist, contract has not been initialized.
        let is_native_token_exist = env.storage().instance().has(&DataKey::NativeToken);
        if !is_native_token_exist {
            return SellCowResult::default(env, Status::NotInitialized);
        }

        // check if cow still alive.
        let is_cow_alive = env.storage().temporary().has(&cow_id);
        if !is_cow_alive {
            return SellCowResult::default(env, Status::NotFound);
        }

        // check if ownership data exist.
        let is_ownership_exist = env.storage().persistent().has(&user);
        if !is_ownership_exist {
            return SellCowResult::default(env, Status::MissingOwnership);
        }

        // get cow data.
        let cow_data: CowData = env.storage().temporary().get(&cow_id).unwrap();

        // here we check the age of the cow.
        // a cow can only be sold after it has been alive for 3 days.
        let current_ledger: u32 = env.ledger().sequence();
        let cow_age: u32 = current_ledger - cow_data.born_ledger;
        if cow_age < LEDGER_AMOUNT_IN_3_DAYS {
            return SellCowResult::default(env, Status::Underage);
        }

        // calculate cow selling price.
        let cow_base_price: i128 = get_cow_base_price_in_stroops(&cow_data.breed);
        let cow_selling_price = get_cow_appraisal_price(&cow_data, cow_base_price);

        // initiate native token client.
        let native_token: Address = env.storage().instance().get(&DataKey::NativeToken).unwrap();
        let native_token_client = token::Client::new(&env, &native_token);

        // get current contract balance.
        let contract_native_token_balance: i128 =
            native_token_client.balance(&env.current_contract_address());
        if contract_native_token_balance < cow_selling_price {
            return SellCowResult::default(env, Status::InsufficientFund);
        }

        // transfer native token to user to complete the selling process.
        native_token_client.transfer(&env.current_contract_address(), &user, &cow_selling_price);

        // get ownership list and remove Cow ID.
        let mut cow_ownership_list: Vec<String> = env.storage().persistent().get(&user).unwrap();
        let index = cow_ownership_list.first_index_of(&cow_id).unwrap();
        cow_ownership_list.remove_unchecked(index);

        // save new ownership data & bump lifetime to 1 week.
        env.storage().persistent().set(&user, &cow_ownership_list);
        env.storage()
            .persistent()
            .bump(&user, LEDGER_AMOUNT_IN_1_WEEK, LEDGER_AMOUNT_IN_1_WEEK);

        // remove cow data from storage.
        env.storage().temporary().remove(&cow_id);

        // publish Cowchain Farm SELL event
        let new_cow_event = CowEventDetails {
            id: cow_data.id.clone(),
            name: cow_data.name.clone(),
            owner: user,
            last_fed_ledger: cow_data.last_fed_ledger.clone(),
        };
        env.events()
            .publish((symbol_short!("sell"),), new_cow_event);

        SellCowResult {
            status: Status::Ok,
            ownership: cow_ownership_list,
        }
    }

    fn cow_appraisal(env: Env, cow_id: String) -> CowAppraisalResult {
        // check if cow still alive.
        let is_cow_alive = env.storage().temporary().has(&cow_id);
        if !is_cow_alive {
            return CowAppraisalResult::default(Status::NotFound);
        }

        // get cow price based on their breed (the price will be in XLM unit).
        let cow_data: CowData = env.storage().temporary().get(&cow_id).unwrap();
        let cow_base_price: i128 = get_cow_base_price_in_stroops(&cow_data.breed);

        // get cow appraisal price.
        let cow_price_appraisal = get_cow_appraisal_price(&cow_data, cow_base_price);

        CowAppraisalResult {
            status: Status::Ok,
            price: cow_price_appraisal,
        }
    }

    fn feed_the_cow(env: Env, user: Address, cow_id: String) -> CowStatus {
        // check if cow still alive.
        let is_cow_alive = env.storage().temporary().has(&cow_id);
        if !is_cow_alive {
            return CowStatus::new(env, Status::NotFound);
        }

        // check if ownership data exist.
        let is_ownership_exist = env.storage().persistent().has(&user);
        if !is_ownership_exist {
            return CowStatus::new(env, Status::MissingOwnership);
        }

        // get cow data from storage.
        let mut cow_data: CowData = env.storage().temporary().get(&cow_id).unwrap();

        // so in 24 hours there are approximately 17280 ledger.
        // we have 4 feeding time zone, that is every 4320 ledger.
        //
        // the time zones are:
        // 1st 4320 ledger -> FULL
        // 2nd 4320 ledger -> ON TIME
        // 3rd 4320 ledger -> LATE
        // 4th 4320 ledger -> FORGET
        //
        // the basic rule in Cowchain Farm are:
        // if feed distance are less than 4320 ledger, the cow won't eat, still full.
        // if feed distance are more than 17280 ledger, the cow will die.

        // find out feeding distance.
        let current_ledger: u32 = env.ledger().sequence();
        let last_fed_ledger: u32 = cow_data.last_fed_ledger;
        let feed_distance: u32 = current_ledger - last_fed_ledger;

        // when the cow is still full, no bump operation will be made to its data.
        if feed_distance <= WELL_FED {
            return CowStatus::new(env, Status::FullStomach);
        }

        // calculate feeding stats.
        let mut on_time = cow_data.feeding_stats.on_time;
        let mut late = cow_data.feeding_stats.late;
        let mut forget = cow_data.feeding_stats.forget;

        if feed_distance > WELL_FED && feed_distance <= ON_TIME_FEED {
            on_time = on_time + 1;
        }
        if feed_distance > ON_TIME_FEED && feed_distance <= LATE_FEED {
            late = late + 1;
        }
        if feed_distance > LATE_FEED {
            forget = forget + 1;
        }

        // update cow data.
        cow_data.last_fed_ledger = env.ledger().sequence();
        cow_data.feeding_stats = CowFeedingStats {
            on_time,
            late,
            forget,
        };

        // save updated cow data & bump lifetime to 24 hours.
        env.storage().temporary().set(&cow_id, &cow_data);
        env.storage().temporary().bump(
            &cow_id,
            LEDGER_AMOUNT_IN_24_HOURS,
            LEDGER_AMOUNT_IN_24_HOURS,
        );

        // bump user lifetime to 1 week.
        env.storage()
            .persistent()
            .bump(&user, LEDGER_AMOUNT_IN_1_WEEK, LEDGER_AMOUNT_IN_1_WEEK);

        // publish Cowchain Farm FEED event
        let new_cow_event = CowEventDetails {
            id: cow_data.id.clone(),
            name: cow_data.name.clone(),
            owner: user,
            last_fed_ledger: cow_data.last_fed_ledger.clone(),
        };
        env.events()
            .publish((symbol_short!("feed"),), new_cow_event);

        CowStatus {
            status: Status::Ok,
            ledger: cow_data.last_fed_ledger,
        }
    }

    fn get_all_cow(env: Env, user: Address) -> GetAllCowResult {
        // ensures that user has authorized invocation of this contract.
        user.require_auth();

        // check if ownership data exist.
        let is_ownership_exist = env.storage().persistent().has(&user);
        if !is_ownership_exist {
            return GetAllCowResult {
                status: Status::Fail,
                data: Vec::new(&env),
            };
        }

        // get ownership data.
        let ownership_data: Vec<String> = env.storage().persistent().get(&user).unwrap();

        // get all cow data.
        let mut cow_data_list: Vec<CowData> = Vec::new(&env);
        for cow_id in ownership_data {
            let is_cow_alive = env.storage().temporary().has(&cow_id);
            if !is_cow_alive {
                continue;
            }
            let cow_data: CowData = env.storage().temporary().get(&cow_id).unwrap();
            cow_data_list.push_back(cow_data);
        }

        GetAllCowResult {
            status: Status::Ok,
            data: cow_data_list,
        }
    }

    // ! TESTNET DEVELOPMENT
    // ! ----------------------------------------------------------------------------
    fn pub_auction(env: Env, user: Address) -> u32 {
        // new cow event
        let new_cow_event = CowEventDetails {
            id: String::from_slice(&env, "qKRoefgNAp321"),
            name: symbol_short!("mainland"),
            owner: user,
            last_fed_ledger: env.ledger().sequence(),
        };
        // publish cow event
        env.events()
            .publish((symbol_short!("auction"),), new_cow_event);
        // give success result
        env.ledger().sequence()
    }

    fn register_auction(
        env: Env,
        user: Address,
        cow_id: String,
        auction_id: String,
        price: u32,
    ) -> Status {
        // check Admin key in storage.
        // if Admin key not exist, contract has not been initialized.
        let is_admin_exist = env.storage().instance().has(&DataKey::Admin);
        if !is_admin_exist {
            return Status::NotInitialized;
        }

        // ensures that user has authorized invocation of this contract.
        user.require_auth();

        // check if cow still alive.
        let is_cow_alive = env.storage().temporary().has(&cow_id);
        if !is_cow_alive {
            return Status::NotFound;
        }

        // Set CowData's auction ID to indicate that this cow is being auctioned.
        let mut cow_data: CowData = env.storage().temporary().get(&cow_id).unwrap();
        cow_data.auction_id = auction_id.clone();

        let new_auction_data = AuctionData {
            auction_id: auction_id.clone(),
            cow_id,
            cow_name: cow_data.name.clone(),
            cow_breed: cow_data.breed.clone(),
            cow_born_ledger: cow_data.born_ledger.clone(),
            owner: user.clone(),
            start_price: price as i128,
            highest_bidder: Bidder {
                user: user.clone(),
                price: price as i128,
            },
            bid_history: Vec::new(&env),
            auction_limit_ledger: LEDGER_AMOUNT_IN_24_HOURS,
        };

        // Create and/or append auction list
        let mut auction_list: Vec<String> = Vec::new(&env);
        // if auction data exist, append the data to auction list.
        let is_list_exist = env.storage().persistent().has(&DataKey::AuctionList);
        if is_list_exist {
            // get current ownership data.
            let stored_auction_list: Vec<String> = env
                .storage()
                .persistent()
                .get(&DataKey::AuctionList)
                .unwrap();
            auction_list.append(&stored_auction_list);
        }

        // save auction list & bump lifetime to 1 month.
        auction_list.push_back(auction_id.clone());
        env.storage()
            .persistent()
            .set(&DataKey::AuctionList, &auction_list);
        env.storage().persistent().bump(
            &DataKey::AuctionList,
            LEDGER_AMOUNT_IN_1_MONTH,
            LEDGER_AMOUNT_IN_1_MONTH,
        );

        // save auction data & bump lifetime to 48 hours.
        env.storage()
            .temporary()
            .set(&auction_id, &new_auction_data);
        env.storage().temporary().bump(
            &auction_id,
            LEDGER_AMOUNT_IN_48_HOURS,
            LEDGER_AMOUNT_IN_48_HOURS,
        );

        // save updated cow data & bump lifetime to 24 hours.
        env.storage().temporary().set(&cow_id, &cow_data);
        env.storage().temporary().bump(
            &cow_id,
            LEDGER_AMOUNT_IN_24_HOURS,
            LEDGER_AMOUNT_IN_24_HOURS,
        );

        // bump user lifetime to 1 week.
        env.storage()
            .persistent()
            .bump(&user, LEDGER_AMOUNT_IN_1_WEEK, LEDGER_AMOUNT_IN_1_WEEK);

        Status::Ok
    }
}

fn get_cow_base_price_in_stroops(breed: &CowBreed) -> i128 {
    // get cow price based on their breed (the price will be in XLM unit).
    let cow_price_in_native_token = match breed {
        CowBreed::Jersey => JERSEY_PRICE,
        CowBreed::Limousin => LIMOUSIN_PRICE,
        CowBreed::Hallikar => HALLIKAR_PRICE,
        CowBreed::Hereford => HEREFORD_PRICE,
        CowBreed::Holstein => HOLSTEIN_PRICE,
        CowBreed::Simmental => SIMMENTAL_PRICE,
    };

    // we need to convert the price from XLM to stroops unit.
    // this is because Soroban uses the smallest unit for its operation.
    // convert price to stroops unit by multiplying XLM unit with 10 million.
    cow_price_in_native_token * 10_000_000
}

fn get_cow_appraisal_price(cow_data: &CowData, cow_base_price: i128) -> i128 {
    // calculate appraisal multiplier.
    let on_time_rewards: i128 = (cow_data.feeding_stats.on_time as i128) * ON_TIME_REWARD;
    let late_rewards: i128 = (cow_data.feeding_stats.late as i128) * LATE_REWARD;
    let forget_fines: i128 = (cow_data.feeding_stats.forget as i128) * FORGET_FINE;
    let mut rewards_fines_multiplier: i128 = on_time_rewards + late_rewards - forget_fines;
    if rewards_fines_multiplier < -PRECISION_100_PERCENT {
        // rewards_or_fines cannot less than 0.
        rewards_fines_multiplier = -PRECISION_100_PERCENT;
    }
    let rewards_or_fines: i128 =
        (cow_base_price * rewards_fines_multiplier) / PRECISION_100_PERCENT;

    // calculate cow appraisal price.
    cow_base_price + rewards_or_fines
}
