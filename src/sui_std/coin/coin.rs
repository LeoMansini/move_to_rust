use crate::sui_std::balance::balance;
use balance::{Balance, Supply};
use crate::sui_std::transfer::transfer;
use std::sync::LazyLock;

pub struct IdGetter {
    current_id: std::sync::Mutex<u8>,
}

impl IdGetter {
    pub fn new() -> Self {
        IdGetter {
            current_id: std::sync::Mutex::new(0),
        }
    }

    pub fn get_new_id(&self) -> u8 {
        let mut id = self.current_id.lock().unwrap();
        *id += 1;
        *id
    }
}

// Use LazyLock to initialize ID_GETTER
pub static ID_GETTER: LazyLock<IdGetter> = LazyLock::new(|| IdGetter::new());

// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Defines the `Coin` type - platform wide representation of fngible
/// tokens and coins. `Coin` can be described as a secure wrapper around
/// `Balance` type.
pub struct sui__coin {}

// Allows calling `.split_vec(amounts, ctx)` on `coin`

// Allows calling `.join_vec(coins)` on `coin`

// Allows calling `.split_and_transfer(amount, recipient, ctx)` on `coin`

// Allows calling `.divide_and_keep(n, ctx)` on `coin`

/// A type passed to create_supply is not a one-time witness.
const EBadWitness: u64 = 0;
/// Invalid arguments are passed to a fnction.
const EInvalidArg: u64 = 1;
/// Trying to split a coin more times than its balance allows.
const ENotEnough: u64 = 2;
// #[error]
// const EGlobalPauseNotAllowed: Vec<u8> =
//    b"Kill switch was not allowed at the creation of the DenyCapV2";
const EGlobalPauseNotAllowed: u64 = 3;

/// A coin of type `T` worth `value`. Transferable and storable
pub struct Coin {
    id: u8,
    balance: Balance,
}

/// Each Coin type T created through `create_currency` fnction will have a
/// unique instance of CoinMetadata that stores the metadata for this coin type.
pub struct CoinMetadata {
    id: u8,
    /// Number of decimal places the coin uses.
    /// A coin with `value ` N and `decimals` D should be shown as N / 10^D
    /// E.g., a coin with `value` 7002 and decimals 3 should be displayed as 7.002
    /// This is metadata for display usage only.
    decimals: u8,
    /// Name for the token
    name: String,
    /// Symbol for the token
    symbol: String,
    /// Description of the token
    description: String,
    /// URL for the token logo
    icon_url: Option<String>,
}

/// Similar to CoinMetadata, but created only for regulated coins that 
/// This object is always immutable.
pub struct RegulatedCoinMetadata {
    id: u8,
    /// The u8 of the coin's CoinMetadata object.
    coin_metadata_object: u8,
    /// The u8 of the coin's DenyCap object.
    deny_cap_object: u8,
}

/// Capability allowing the bearer to mint and burn
/// coins of type `T`. Transferable
pub struct TreasuryCap {
    id: u8,
    total_supply: Supply,
}

/// Capability allowing the bearer to deny Stringes from using the currency's coins--
/// immediately preventing those Stringes from interacting with the coin as an input to a
/// transaction and at the start of the next preventing them from receiving the coin.
/// If `allow_global_pause` is true, the bearer can enable a global pa
/// all Stringes were added to the deny list.
pub struct DenyCapV2 {
    id: u8,
    allow_global_pause: bool,
}

// === Supply <-> TreasuryCap morphing and accessors  ===

/// Return the total number of `T`'s in circulation.
pub fn total_supply(cap: &TreasuryCap) -> u64 {
    balance::supply_value(&cap.total_supply)
}

/// Unwrap `TreasuryCap` getting the `Supply`.
///
/// Operation is irreversible. Supply cannot be converted into a `TreasuryCap` due
/// to different security guarantees (TreasuryCap can be created only once for a type)
pub fn treasury_into_supply(treasury: TreasuryCap) -> Supply {
    let TreasuryCap { id, total_supply } = treasury;
    total_supply
}

/// Get immutable reference to the treasury's `Supply`.
pub fn supply_immut(treasury: &TreasuryCap) -> &Supply {
    &treasury.total_supply
}

/// Get mutable reference to the treasury's `Supply`.
pub fn supply_mut(treasury: &mut TreasuryCap) -> &mut Supply {
    &mut treasury.total_supply
}

// === Balance <-> Coin accessors and type morphing ===

impl Coin{}
/// Public getter for the coin's value
pub fn value(c: &Coin) -> u64 {
    balance::value(&c.balance)
}

/// Get immutable reference to the balance of a coin.
pub fn balance(c: &Coin) -> &Balance {
    &c.balance
}

/// Get a mutable reference to the balance of a coin.
pub fn balance_mut(c: &mut Coin) -> &mut Balance {
    &mut c.balance
}

/// Put a `Coin` to the `Balance`.
pub fn put(balance: &mut Balance, coin: Coin) {
    balance::join(balance, into_balance(coin));
}

// === Base Coin fnctionality ===

/// Split coin `self` to two coins, one with balance `split_amount`,
/// and the remaining balance is left is `self`.
pub fn split(c: &mut Coin, split_amount: u64, ) -> Coin {
    take(&mut c.balance, split_amount)
}

/// Split coin `self` into `n - 1` coins with equal balances. The remainder is left in
/// `self`. Return newly created coins.
pub fn divide_into_n(c: &mut Coin, n: u64, ) -> Vec<Coin> {
    assert!(n > 0, "{}", EInvalidArg);
    assert!(n <= value(c), "{}", ENotEnough);

    let mut vec = Vec::new();
    let mut i = 0;
    let split_amount = value(c) / n;
    while (i < n - 1) {
        vec.push(split(c, split_amount));
        i = i + 1;
    };
    vec
}

/// Make any Coin with a zero value. Useful for placeholding
/// bids/payments or preemptively making empty balances.
pub fn zero() -> Coin {
    Coin { id: ID_GETTER.get_new_id(), balance: balance::zero() }
}

/// Destroy a coin with value zero
pub fn destroy_zero(c: Coin) {
    let Coin { id, balance } = c;
    balance::destroy_zero(balance)
}

/// Consume the coin `c` and add its value to `self`.
/// Aborts if `c.value + self.value > U64_MAX`
pub fn join(this: &mut Coin, c: Coin) {
    let Coin { id, balance } = c;
    balance::join(&mut this.balance, balance);
}

/// Take a `Coin` worth of `value` from `Balance`.
/// Aborts if `value > balance.value`
pub fn take(balance: &mut Balance, value: u64, ) -> Coin {
    Coin {
        id: ID_GETTER.get_new_id(),
        balance: balance::split(balance, value),
    }
}

/// Wrap a balance into a Coin to make it transferable.
pub fn from_balance(balance: Balance, ) -> Coin {
    Coin { id: ID_GETTER.get_new_id(), balance }
}

/// Destruct a Coin wrapper and keep the balance.
pub fn into_balance(c: Coin) -> Balance {
    let Coin { id, balance } = c;
    balance
}

// === Registering new coin types and managing the coin supply ===

/// Create a new currency type `T` as and return the `TreasuryCap` for
/// `T` to the caller. Can only be called with a `one-time-witness`
/// type, ensuring that there's only one `TreasuryCap` per `T`.
pub fn create_currency<T>(
    witness: T,
    decimals: u8,
    symbol: Vec<u8>,
    name: Vec<u8>,
    description: Vec<u8>,
    icon_url: Option<String>,
    
) -> (TreasuryCap, CoinMetadata) {
    // Make sure there's only one instance of the type T
    (
        TreasuryCap {
            id: ID_GETTER.get_new_id(),
            total_supply: balance::create_supply(witness),
        },
        CoinMetadata {
            id: ID_GETTER.get_new_id(),
            decimals,
            name: format!("{:?}", name),
            symbol:format!("{:?}", symbol),
            description: format!("{:?}", description),
            icon_url,
        },
    )
}

/// Create a coin worth `value` and increase the total supply
/// in `cap` accordingly.
pub fn mint(cap: &mut TreasuryCap, value: u64, ) -> Coin {
    Coin {
        id: ID_GETTER.get_new_id(),
        balance: cap.total_supply.increase_supply(value),
    }
}

/// Mint some amount of T as a `Balance` and increase the total
/// supply in `cap` accordingly.
/// Aborts if `value` + `cap.total_supply` >= U64_MAX
pub fn mint_balance(cap: &mut TreasuryCap, value: u64) -> Balance {
    cap.total_supply.increase_supply(value)
}

/// Destroy the coin `c` and decrease the total supply in `cap`
/// accordingly.
pub fn burn(cap: &mut TreasuryCap, c: Coin) -> u64 {
    let Coin { id, balance } = c;
    cap.total_supply.decrease_supply(balance)
}

// === Entrypoints ===

/// Mint `amount` of `Coin` and send it to `recipient`. Invokes `mint()`.
pub fn mint_and_transfer(
    c: &mut TreasuryCap,
    amount: u64,
    recipient: String,
    
) {
    transfer::pub_transfer(mint(c, amount), recipient)
}

// === Update coin metadata ===

/// Update name of the coin in `CoinMetadata`
pub fn update_name(
    _treasury: &TreasuryCap,
    metadata: &mut CoinMetadata,
    name: String,
) {
    metadata.name = name;
}

/// Update the symbol of the coin in `CoinMetadata`
pub fn update_symbol(
    _treasury: &TreasuryCap,
    metadata: &mut CoinMetadata,
    symbol: String,
) {
    metadata.symbol = symbol;
}

/// Update the description of the coin in `CoinMetadata`
pub fn update_description(
    _treasury: &TreasuryCap,
    metadata: &mut CoinMetadata,
    description: String,
) {
    metadata.description = description;
}

/// Update the url of the coin in `CoinMetadata`
pub fn update_icon_url(
    _treasury: &TreasuryCap,
    metadata: &mut CoinMetadata,
    url: String,
) {
    metadata.icon_url = Option::Some(url);
}

// === Get coin metadata fields for on-chain consumption ===

pub fn get_decimals(metadata: &CoinMetadata) -> u8 {
    metadata.decimals
}

pub fn get_name(metadata: &CoinMetadata) -> String {
    metadata.name.clone()
}

pub fn get_symbol(metadata: &CoinMetadata) -> String {
    metadata.symbol.clone()
}

pub fn get_description(metadata: &CoinMetadata) -> String {
    metadata.description.clone()
}

pub fn get_icon_url(metadata: &CoinMetadata) -> Option<String> {
    metadata.icon_url.clone()
}