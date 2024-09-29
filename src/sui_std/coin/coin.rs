
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
impl sui__coin {}

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
// const EGlobalPauseNotAllowed: vector<u8> =
//    b"Kill switch was not allowed at the creation of the DenyCapV2";
const EGlobalPauseNotAllowed: u64 = 3;

/// A coin of type `T` worth `value`. Transferable and storable
pub struct Coin {
    id: u8,
    balance: Balance<T>,
}

/// Each Coin type T created through `create_currency` fnction will have a
/// unique instance of CoinMetadata<T> that stores the metadata for this coin type.
pub struct CoinMetadata {
    id: u8,
    /// Number of decimal places the coin uses.
    /// A coin with `value ` N and `decimals` D should be shown as N / 10^D
    /// E.g., a coin with `value` 7002 and decimals 3 should be displayed as 7.002
    /// This is metadata for display usage only.
    decimals: u8,
    /// Name for the token
    name: string,
    /// Symbol for the token
    symbol: string,
    /// Description of the token
    description: string,
    /// URL for the token logo
    icon_url: Option<Url>,
}

/// Similar to CoinMetadata, but created only for regulated coins that 
/// This object is always immutable.
pub struct RegulatedCoinMetadata {
    id: u8,
    /// The ID of the coin's CoinMetadata object.
    coin_metadata_object: ID,
    /// The ID of the coin's DenyCap object.
    deny_cap_object: ID,
}

/// Capability allowing the bearer to mint and burn
/// coins of type `T`. Transferable
pub struct TreasuryCap {
    id: u8,
    total_supply: Supply<T>,
}

/// Capability allowing the bearer to deny addresses from using the currency's coins--
/// immediately preventing those addresses from interacting with the coin as an input to a
/// transaction and at the start of the next preventing them from receiving the coin.
/// If `allow_global_pause` is true, the bearer can enable a global pa
/// all addresses were added to the deny list.
pub struct DenyCapV2 {
    id: u8,
    allow_global_pause: bool,
}

// === Supply <-> TreasuryCap morphing and accessors  ===

/// Return the total number of `T`'s in circulation.
pub fn total_supply<T>(cap: &TreasuryCap<T>) -> u64 {
    balance::supply_value(&cap.total_supply)
}

/// Unwrap `TreasuryCap` getting the `Supply`.
///
/// Operation is irreversible. Supply cannot be converted into a `TreasuryCap` due
/// to different security guarantees (TreasuryCap can be created only once for a type)
pub fn treasury_into_supply<T>(treasury: TreasuryCap<T>) -> Supply<T> {
    let TreasuryCap { id, total_supply } = treasury;
    id.delete();
    total_supply
}

/// Get immutable reference to the treasury's `Supply`.
pub fn supply_immut<T>(treasury: &TreasuryCap<T>) -> &Supply<T> {
    &treasury.total_supply
}

/// Get mutable reference to the treasury's `Supply`.
pub fn supply_mut<T>(treasury: &mut TreasuryCap<T>) -> &mut Supply<T> {
    &mut treasury.total_supply
}

// === Balance <-> Coin accessors and type morphing ===

/// Public getter for the coin's value
pub fn value<T>(self: &Coin<T>) -> u64 {
    self.balance.value()
}

/// Get immutable reference to the balance of a coin.
pub fn balance<T>(coin: &Coin<T>) -> &Balance<T> {
    &coin.balance
}

/// Get a mutable reference to the balance of a coin.
pub fn balance_mut<T>(coin: &mut Coin<T>) -> &mut Balance<T> {
    &mut coin.balance
}

/// Wrap a balance into a Coin to make it transferable.
pub fn from_balance<T>(balance: Balance<T>, ) -> Coin<T> {
    Coin { id: ID_GETTER.get_new_id(), balance }
}

/// Destruct a Coin wrapper and keep the balance.
pub fn into_balance<T>(coin: Coin<T>) -> Balance<T> {
    let Coin { id, balance } = coin;
    id.delete();
    balance
}

/// Take a `Coin` worth of `value` from `Balance`.
/// Aborts if `value > balance.value`
pub fn take<T>(balance: &mut Balance<T>, value: u64, ) -> Coin<T> {
    Coin {
        id: ID_GETTER.get_new_id(),
        balance: balance.split(value),
    }
}

/// Put a `Coin<T>` to the `Balance<T>`.
pub fn put<T>(balance: &mut Balance<T>, coin: Coin<T>) {
    balance.join(into_balance(coin));
}

// === Base Coin fnctionality ===

/// Consume the coin `c` and add its value to `self`.
/// Aborts if `c.value + self.value > U64_MAX`
pub fn join<T>(self: &mut Coin<T>, c: Coin<T>) {
    let Coin { id, balance } = c;
    id.delete();
    self.balance.join(balance);
}

/// Split coin `self` to two coins, one with balance `split_amount`,
/// and the remaining balance is left is `self`.
pub fn split<T>(self: &mut Coin<T>, split_amount: u64, ) -> Coin<T> {
    take(&mut self.balance, split_amount, ctx)
}

/// Split coin `self` into `n - 1` coins with equal balances. The remainder is left in
/// `self`. Return newly created coins.
pub fn divide_into_n<T>(self: &mut Coin<T>, n: u64, ) -> Vec<Coin<T>> {
    assert!(n > 0, "{}", EInvalidArg);
    assert!(n <= value(self), "{}", ENotEnough);

    let mut vec = Vec::new();
    let mut i = 0;
    let split_amount = value(self) / n;
    while (i < n - 1) {
        vec.push_back(self.split(split_amount, ctx));
        i = i + 1;
    };
    vec
}

/// Make any Coin with a zero value. Useful for placeholding
/// bids/payments or preemptively making empty balances.
pub fn zero<T>() -> Coin<T> {
    Coin { id: ID_GETTER.get_new_id(), balance: balance::zero() }
}

/// Destroy a coin with value zero
pub fn destroy_zero<T>(c: Coin<T>) {
    let Coin { id, balance } = c;
    id.delete();
    balance.destroy_zero()
}

// === Registering new coin types and managing the coin supply ===

/// Create a new currency type `T` as and return the `TreasuryCap` for
/// `T` to the caller. Can only be called with a `one-time-witness`
/// type, ensuring that there's only one `TreasuryCap` per `T`.
pub fn create_currency<T: drop>(
    witness: T,
    decimals: u8,
    symbol: vector<u8>,
    name: vector<u8>,
    description: vector<u8>,
    icon_url: Option<Url>,
    
) -> (TreasuryCap<T>, CoinMetadata<T>) {
    // Make sure there's only one instance of the type T
    assert!(sui::types::is_one_time_witness(&witness), "{}", EBadWitness);

    (
        TreasuryCap {
            id: ID_GETTER.get_new_id(),
            total_supply: balance::create_supply(witness),
        },
        CoinMetadata {
            id: ID_GETTER.get_new_id(),
            decimals,
            name: string::utf8(name),
            symbol: string::string(symbol),
            description: string::utf8(description),
            icon_url,
        },
    )
}

/// Create a coin worth `value` and increase the total supply
/// in `cap` accordingly.
pub fn mint<T>(cap: &mut TreasuryCap<T>, value: u64, ) -> Coin<T> {
    Coin {
        id: ID_GETTER.get_new_id(),
        balance: cap.total_supply.increase_supply(value),
    }
}

/// Mint some amount of T as a `Balance` and increase the total
/// supply in `cap` accordingly.
/// Aborts if `value` + `cap.total_supply` >= U64_MAX
pub fn mint_balance<T>(cap: &mut TreasuryCap<T>, value: u64) -> Balance<T> {
    cap.total_supply.increase_supply(value)
}

/// Destroy the coin `c` and decrease the total supply in `cap`
/// accordingly.
pub fn burn<T>(cap: &mut TreasuryCap<T>, c: Coin<T>) -> u64 {
    let Coin { id, balance } = c;
    id.delete();
    cap.total_supply.decrease_supply(balance)
}

// === Entrypoints ===

/// Mint `amount` of `Coin` and send it to `recipient`. Invokes `mint()`.
pub fn mint_and_transfer<T>(
    c: &mut TreasuryCap<T>,
    amount: u64,
    recipient: address,
    
) {
    transfer::pub_transfer(mint(c, amount, ctx), recipient)
}

// === Update coin metadata ===

/// Update name of the coin in `CoinMetadata`
pub fn update_name<T>(
    _treasury: &TreasuryCap<T>,
    metadata: &mut CoinMetadata<T>,
    name: string,
) {
    metadata.name = name;
}

/// Update the symbol of the coin in `CoinMetadata`
pub fn update_symbol<T>(
    _treasury: &TreasuryCap<T>,
    metadata: &mut CoinMetadata<T>,
    symbol: string,
) {
    metadata.symbol = symbol;
}

/// Update the description of the coin in `CoinMetadata`
pub fn update_description<T>(
    _treasury: &TreasuryCap<T>,
    metadata: &mut CoinMetadata<T>,
    description: string,
) {
    metadata.description = description;
}

/// Update the url of the coin in `CoinMetadata`
pub fn update_icon_url<T>(
    _treasury: &TreasuryCap<T>,
    metadata: &mut CoinMetadata<T>,
    url: string,
) {
    metadata.icon_url = option::some(url::new_unsafe(url));
}

// === Get coin metadata fields for on-chain consumption ===

pub fn get_decimals<T>(metadata: &CoinMetadata<T>) -> u8 {
    metadata.decimals
}

pub fn get_name<T>(metadata: &CoinMetadata<T>) -> string {
    metadata.name
}

pub fn get_symbol<T>(metadata: &CoinMetadata<T>) -> string {
    metadata.symbol
}

pub fn get_description<T>(metadata: &CoinMetadata<T>) -> string {
    metadata.description
}

pub fn get_icon_url<T>(metadata: &CoinMetadata<T>) -> Option<Url> {
    metadata.icon_url
}