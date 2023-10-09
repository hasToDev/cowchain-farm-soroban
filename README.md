![Stellar](https://img.shields.io/badge/Stellar-7D00FF?style=for-the-badge&logo=Stellar&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

<br>
<div align="center">
<img src="cowchain_farm.png" alt="Cowchain" width="533">
<p align="center">Smart contract-based Cow Farming web app built with Flutter and Soroban</p>
</div>

## About Cowchain Farm

Cowchain Farm combines Rust-based smart contracts built with [Soroban](https://soroban.stellar.org) on Stellar
blockchain, and a web app client to
access the smart contracts functionality built with one of the most popular cross-platform
frameworks, [Flutter](https://flutter.dev).

Cowchain Farm smart contract will cover several capabilities of Soroban that exist in
the [Preview 11 release](https://soroban.stellar.org/docs/reference/releases), which include:

1. Authentication and authorization
2. Error handling
3. Custom types
4. Contract initialization
5. Contract upgrading
6. Payment transfer
7. Data storage expiration
8. Events

While the Cowchain Farm web app will cover the following:

1. Calling Soroban smart contract function using [Flutter Stellar SDK](https://pub.dev/packages/stellar_flutter_sdk)
2. Communication with the [Freighter](https://www.freighter.app) browser extension

And the latest addition is the Cowchain Farm notification service, which includes:

1. Ingesting events from Soroban smart contracts using [Dart CLI](https://dart.dev/tutorials/server/get-started)
   with [Flutter Stellar SDK](https://pub.dev/packages/stellar_flutter_sdk)
2. Sending notifications to Cowchain Farm users using [OneSignal](https://onesignal.com)

## Get Started

This article is specifically about the Soroban smart contract for Cowchain Farm.

Discussion of the Cowchain Farm web app is in
the [Cowchain Farm App repository](https://github.com/hasToDev/cowchain-farm-app),
and the discussion for Cowchain Farm Dart CLI notification service is in
the [Cowchain Farm Alert repository](https://github.com/hasToDev/cowchain-farm-alert).

The Cowchain Farm smart contract in this repository was developed using `Rust version 1.73.0-nightly`
and `Soroban CLI 20.0.0-rc4`

## Install Rust and Soroban CLI

The first step you have to do is install Rust. You can follow the steps to install Rust in the following article:

- [Install Rust - Rust Programming Language](https://www.rust-lang.org/tools/install)
- [Walkthrough: Installing Rust on Windows](https://www.alpharithms.com/installing-rust-on-windows-403718/)
- [How To Install Rust on Ubuntu 20.04](https://www.digitalocean.com/community/tutorials/install-rust-on-ubuntu-linux)

Next we install Soroban CLI:

```shell
cargo install --locked --version 20.0.0-rc4 soroban-cli
```

Confirm that both Rust and Soroban CLI are installed by running `rustc --version` and `soroban --version`.

You should receive a result that is more or less similar to:

```text
rustc 1.73.0-nightly (32303b219 2023-07-29)

soroban 20.0.0-rc4 (bce5e56ba16ba977200b022c91f3eaf6282f47eb)
soroban-env 20.0.0-rc2 (8c63bff68a15d79aca3a705ee6916a68db57b7e6)
soroban-env interface version 85899345977
stellar-xdr 20.0.0-rc1 (d5ce0c9e7aa83461773a6e81662067f35d39e4c1)
xdr curr (9ac02641139e6717924fdad716f6e958d0168491)
```

## Clone, Build, and Deploy

1. Clone the repository:
    ```shell
    git clone https://github.com/hasToDev/cowchain-farm-soroban.git
    ```

2. Build the contract:
   ```shell
   cargo build --target wasm32-unknown-unknown --release
   ```

3. Deploy the contract to Testnet:
   ```shell
   soroban contract deploy \
   --wasm target/wasm32-unknown-unknown/release/cowchain-farm-soroban.wasm \
   --rpc-url https://soroban-testnet.stellar.org:443 \
   --network-passphrase 'Test SDF Network ; September 2015'
   ```
   After the deployment is complete, you will receive a **Contract Address**. Save that address to be used in calling
   the contract functions.

   The form of **Contract Address** will be similar
   to `CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8`.<br><br>

4. (optional) Install the contract to Testnet:
   ```shell
   soroban contract install \
   --wasm target/wasm32-unknown-unknown/release/cowchain-farm-soroban.wasm \
   --rpc-url https://soroban-testnet.stellar.org:443 \
   --network-passphrase 'Test SDF Network ; September 2015'
   ```
   After the installation process is complete, you will receive a **Contract ID**.

   The form of **Contract ID** will be similar
   to `900c8b247d0acz41befcf6a441ebddf6f5pf5cKe10d79a5ef88Sa315665bf926`.<br>

   <br>You can use this Contract ID as a **WASM Hash** argument that will be needed when using a contract deployer
   or when you want to upgrade your current contract.

## Prerequisites

### Accounts

Before calling Cowchain Farm smart contract function, make sure you have at least 2 **Stellar TESTNET account**. You
can create the account using Stellar
Laboratory [here](https://laboratory.stellar.org/#account-creator?network=test).

The first account will be used as an administrator account, while the second account will be used as a user account.

For example, we will use the two accounts below as administrator and user:

| Description | ADMINISTRATOR                                            | 
|-------------|----------------------------------------------------------|
| Public Key  | GCMEOWWTRG6QD2S5F2V66CJTT7EG4MDPL7U523SGTLOHZPPUAJFGNIS6 |
| Secret Key  | SBNESSDQWIDIO7NYDAHM2STHSVZPIIPM3OGT6PB56DL2EE4XXIHECYYP |

| Description | USER                                                     | 
|-------------|----------------------------------------------------------|
| Public Key  | GCK2IJZ3XTVRZWX27YITE2DBIDSHDIVNIICLJ63P6XXFAFHVFFWS52UY |
| Secret Key  | SDM6DSM6Y3KZ3AN5FW632FYRW3RND6K42LSMEKUGCIP6FPSBHL5RJFDE |

### Initialization Password

When we initialize this contract, we will need the initialization password, which we have previously hard-coded into the
<u>init</u> function. You can find this password in the [lib.rs](src/lib.rs) file.

**IMPORTANT:** Be sure to replace the password in the <u>init</u> function with your own before deploying.

The primary purpose of using this password is to prevent others from initializing your contract. Instead, you can use
the deployer contract. the tutorial is [here](https://soroban.stellar.org/docs/basic-tutorials/deployer).

### Stellar Native Asset Contract Address

When we initialize this contract, we will need the address of Stellar native asset token. You can find the address using
Soroban CLI:

```shell
soroban lab token id \
--asset native \
--rpc-url https://soroban-testnet.stellar.org:443 \
--network-passphrase 'Test SDF Network ; September 2015'
```

The Stellar native asset token contract address is `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`.

## Calling Smart Contract Function

1. Contract Initialization
   <br> After deployment, the first function we should call is initialization.
   <br> Required auth: <u>ADMIN account authorization</u>.
   <br> Required arguments: <u>ADMIN account
   address</u>, <u>native token address</u>, and <u>messsage</u> AKA <u>password</u>.
   ```shell
   soroban contract invoke \
   --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
   --source SBNESSDQWIDIO7NYDAHM2STHSVZPIIPM3OGT6PB56DL2EE4XXIHECYYP \
   --rpc-url https://soroban-testnet.stellar.org:443 \
   --network-passphrase 'Test SDF Network ; September 2015' \
   --fee 12345678 \
   -- \
   init \
   --admin GCMEOWWTRG6QD2S5F2V66CJTT7EG4MDPL7U523SGTLOHZPPUAJFGNIS6 \
   --native_token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
   --message "kBfn7v17QdX4kD856bX2mBj1Y"
   ```
2. Contract Upgrade
   <br> Required auth: <u>ADMIN account authorization</u>.
   <br> Required arguments: <u>WASM Hash</u> from *soroban contract install*.
   ```shell
   soroban contract invoke \
   --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
   --source SBNESSDQWIDIO7NYDAHM2STHSVZPIIPM3OGT6PB56DL2EE4XXIHECYYP \
   --rpc-url https://soroban-testnet.stellar.org:443 \
   --network-passphrase 'Test SDF Network ; September 2015' \
   --fee 12345678 \
   -- \
   upgrade \
   --new_wasm_hash 900c8b247d0acz41befcf6a441ebddf6f5pf5cKe10d79a5ef88Sa315665bf926
   ```

3. Extend or Bump Contract Instance Storage Lifetime
   <br> This will bump your contract instance storage lifetime to the N ledger after the current ledger sequence.
   <br> Required arguments: <u>Ledger amount</u>.
   ```shell
   soroban contract invoke \
   --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
   --rpc-url https://soroban-testnet.stellar.org:443 \
   --network-passphrase 'Test SDF Network ; September 2015' \
   --fee 12345678 \
   -- \
   bump_instance \
   --ledger_amount 1234
   ```

4. Contract Health Check
   ```shell
   soroban contract invoke \
   --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
   --rpc-url https://soroban-testnet.stellar.org:443 \
   --network-passphrase 'Test SDF Network ; September 2015' \
   --fee 12345678 \
   -- \
   health_check
   ```

5. Cow Purchase
   <br> Required auth: <u>USER account authorization</u>.
   <br> Required arguments: <u>USER account
   address</u>, <u>cow name</u>, <u>cow id</u>, and <u>cow breed</u>.
   ```shell
   soroban contract invoke \
   --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
   --source SDM6DSM6Y3KZ3AN5FW632FYRW3RND6K42LSMEKUGCIP6FPSBHL5RJFDE \
   --rpc-url https://soroban-testnet.stellar.org:443 \
   --network-passphrase 'Test SDF Network ; September 2015' \
   --fee 12345678 \
   -- \
   buy_cow \
   --user GCK2IJZ3XTVRZWX27YITE2DBIDSHDIVNIICLJ63P6XXFAFHVFFWS52UY \
   --cow_name supercattle \
   --cow_id 8e6bbeyd144a4fjY753r80c286d781c7074fb371 \
   --cow_breed 4
   ```

6. Cow Sale
   <br> Required auth: <u>USER account authorization</u>.
   <br> Required arguments: <u>USER account address</u>, and <u>cow id</u>.
   ```shell
   soroban contract invoke \
   --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
   --source SDM6DSM6Y3KZ3AN5FW632FYRW3RND6K42LSMEKUGCIP6FPSBHL5RJFDE \
   --rpc-url https://soroban-testnet.stellar.org:443 \
   --network-passphrase 'Test SDF Network ; September 2015' \
   --fee 12345678 \
   -- \
   sell_cow \
   --user GCK2IJZ3XTVRZWX27YITE2DBIDSHDIVNIICLJ63P6XXFAFHVFFWS52UY \
   --cow_id 8e6bbeyd144a4fjY753r80c286d781c7074fb371
   ```

7. Cow Price Appraisal
   <br> Required arguments: <u>cow id</u>.
   ```shell
   soroban contract invoke \
   --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
   --rpc-url https://soroban-testnet.stellar.org:443 \
   --network-passphrase 'Test SDF Network ; September 2015' \
   --fee 12345678 \
   -- \
   cow_appraisal \
   --cow_id 8e6bbeyd144a4fjY753r80c286d781c7074fb371
   ```

8. Cow Feeding
   <br> Required arguments: <u>USER account address</u>, and <u>cow id</u>.
   ```shell
   soroban contract invoke \
   --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
   --rpc-url https://soroban-testnet.stellar.org:443 \
   --network-passphrase 'Test SDF Network ; September 2015' \
   --fee 12345678 \
   -- \
   feed_the_cow \
   --user GCK2IJZ3XTVRZWX27YITE2DBIDSHDIVNIICLJ63P6XXFAFHVFFWS52UY \
   --cow_id 8e6bbeyd144a4fjY753r80c286d781c7074fb371
   ```

9. Retrieve All User's Cow Data
   <br> Required auth: <u>USER account authorization</u>.
   <br> Required arguments: <u>USER account address</u>.
   ```shell
   soroban contract invoke \
   --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
   --source SDM6DSM6Y3KZ3AN5FW632FYRW3RND6K42LSMEKUGCIP6FPSBHL5RJFDE \
   --rpc-url https://soroban-testnet.stellar.org:443 \
   --network-passphrase 'Test SDF Network ; September 2015' \
   --fee 12345678 \
   -- \
   get_all_cow \
   --user GCK2IJZ3XTVRZWX27YITE2DBIDSHDIVNIICLJ63P6XXFAFHVFFWS52UY
   ```

10. Register Cow Auction
    <br> Required auth: <u>USER account authorization</u>.
    <br> Required arguments: <u>USER account address</u>, <u>cow id</u>, <u>auction id</u>, and <u>start price</u>.
    ```shell
    soroban contract invoke \
    --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
    --source SDM6DSM6Y3KZ3AN5FW632FYRW3RND6K42LSMEKUGCIP6FPSBHL5RJFDE \
    --rpc-url https://soroban-testnet.stellar.org:443 \
    --network-passphrase 'Test SDF Network ; September 2015' \
    --fee 12345678 \
    -- \
    register_auction \
    --user GCK2IJZ3XTVRZWX27YITE2DBIDSHDIVNIICLJ63P6XXFAFHVFFWS52UY \
    --cow_id 8e6bbeyd144a4fjY753r80c286d781c7074fb371 \
    --auction_id UP30JKm637DgL7xnjrywp3hDpdBxAfeLPjbJ27ss \
    --price 1245
    ```

11. Bidding Cow Auction
    <br> Required auth: <u>USER account authorization</u>.
    <br> Required arguments: <u>USER account address</u>, <u>auction id</u>, and <u>bid price</u>.
    ```shell
    soroban contract invoke \
    --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
    --source SDM6DSM6Y3KZ3AN5FW632FYRW3RND6K42LSMEKUGCIP6FPSBHL5RJFDE \
    --rpc-url https://soroban-testnet.stellar.org:443 \
    --network-passphrase 'Test SDF Network ; September 2015' \
    --fee 12345678 \
    -- \
    bidding \
    --user GCK2IJZ3XTVRZWX27YITE2DBIDSHDIVNIICLJ63P6XXFAFHVFFWS52UY \
    --auction_id UP30JKm637DgL7xnjrywp3hDpdBxAfeLPjbJ27ss \
    --bid_price 3467
    ```

12. Finalize or Close Cow Auction
    <br> Required arguments: <u>auction id</u>.
    ```shell
    soroban contract invoke \
    --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
    --source SDM6DSM6Y3KZ3AN5FW632FYRW3RND6K42LSMEKUGCIP6FPSBHL5RJFDE \
    --rpc-url https://soroban-testnet.stellar.org:443 \
    --network-passphrase 'Test SDF Network ; September 2015' \
    --fee 12345678 \
    -- \
    finalize_auction \
    --auction_id UP30JKm637DgL7xnjrywp3hDpdBxAfeLPjbJ27ss
    ```

13. Retrieve All Auction Data
    ```shell
    soroban contract invoke \
    --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
    --source SDM6DSM6Y3KZ3AN5FW632FYRW3RND6K42LSMEKUGCIP6FPSBHL5RJFDE \
    --rpc-url https://soroban-testnet.stellar.org:443 \
    --network-passphrase 'Test SDF Network ; September 2015' \
    --fee 12345678 \
    -- \
    get_all_auction
    ```

14. Give XLM Donation to Contract
    <br> Required auth: <u>USER account authorization</u>.
    <br> Required arguments: <u>USER account address</u>, and <u>donation amount</u>.
    ```shell
    soroban contract invoke \
    --id CB7UCV29SYKUFRZNEIMKVW5XKSJCGTMBCSJFN5OJ2SSXBTPRXO42XGT8 \
    --source SDM6DSM6Y3KZ3AN5FW632FYRW3RND6K42LSMEKUGCIP6FPSBHL5RJFDE \
    --rpc-url https://soroban-testnet.stellar.org:443 \
    --network-passphrase 'Test SDF Network ; September 2015' \
    --fee 12345678 \
    -- \
    open_donation \
    --from GCK2IJZ3XTVRZWX27YITE2DBIDSHDIVNIICLJ63P6XXFAFHVFFWS52UY \
    --amount 25750
    ```

## State Expiration

The Cowchain Farm contract, upon initialization, will have its INSTANCE storage lifetime bumped to 4 weeks.

User registration data has 1 week of lifetime in PERSISTENT storage, and it will be bumped every time
function **buy_cow**, **sell_cow**, **feed_the_cow**, **register_auction**, and **bidding** is called.

While each cow only has 24 hours of lifetime in TEMPORARY storage. It must be bumped by calling the **feed_the_cow**
function before the time is up. Otherwise, the data will be lost.

Cow unique names have the same lifetime of cow data. Every time a cow's lifetime is bumped by the  **feed_the_cow**
function, the Cow unique name will get bumped too.

Every time we register for Cow Auction, although the duration for the auction is 12 hours, the auction data will be
created with 24 hours lifetime.

## Events & Notification Service

There are several events that will be emitted every time a particular function is called.

This event will be ingested by a notification service that will notify each user who installs the [Cowchain Farm
notification app](https://www.dropbox.com/scl/fi/q6qksqguoi30dgfo3t5c2/cowchain_farm_200.apk?rlkey=7n0xadf3j6r01hp65jxsdo0fo&raw=1).
And they will have to register the Stellar wallet account ID or Public Key that they use for Cowchain
Farm in the app.

The event in Cowchain Farm smart contract includes:

1. buy
2. sell
3. feed
4. register
5. refund
6. auction

The Cowchain Farm notification service will send notifications when:

1. Your cow starts to feel hungry.
2. You win an auction.
3. Your funds are refunded because someone outbid you at an auction.

Apart from sending notifications to users, the notification service will also finalize or close the ongoing auction
after the auction has reached its ledger limit. That way, the user won't have to worry about completing their auction.

## License

The Cowchain Farm is distributed under an MIT license. See the [LICENSE](LICENSE) for more information.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Contact

[Hasto](https://github.com/hasToDev) - [@HasToDev](https://twitter.com/HasToDev)
