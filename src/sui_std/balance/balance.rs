
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

/// A storable handler for Balances in general. Is used in the `Coin`
/// module to allow balance operations and can be used to implement
/// custom coins with `Supply` and `Balance`s.

/// For when trying to destroy a non-zero balance.
const ENonZero: u64 = 0;
/// For when an overflow is happening on Supply operations.
const EOverflow: u64 = 1;
/// For when trying to withdraw more than there is.
const ENotEnough: u64 = 2;
/// Sender is not @0x0 the system address.
const ENotSystemAddress: u64 = 3;
/// System operation performed for a coin other than SUI
const ENotSUI: u64 = 4;

/// A Supply of T. Used for minting and burning.
/// Wrapped into a `TreasuryCap` in the `Coin` module.
pub struct Supply {
    value: u64,
}

/// Storable balance - an inner struct of a Coin type.
/// Can be used to store coins which don't need the key ability.
pub struct Balance {
    value: u64,
}

/// Get the amount stored in a `Balance`.
pub fn value(b: &Balance) -> u64 {
    b.value
}

/// Get the `Supply` value.
pub fn supply_value(supply: &Supply) -> u64 {
    supply.value
}

/// Create a new supply for type T.
pub fn create_supply<T>(_: T) -> Supply {
    Supply { value: 0 }
}

/// Increase supply by `value` and create a new `Balance` with this value.
pub fn increase_supply(s: &mut Supply, value: u64) -> Balance {
    assert!(value < (18446744073709551615u64 - s.value), "{}", EOverflow);
    s.value = s.value + value;
    Balance { value }
}

/// Burn a Balance and decrease Supply.
pub fn decrease_supply(s: &mut Supply, balance: Balance) -> u64 {
    let Balance { value } = balance;
    assert!(s.value >= value, "{}", EOverflow);
    s.value = s.value - value;
    value
}

/// Create a zero `Balance` for type `T`.
pub fn zero() -> Balance {
    Balance { value: 0 }
}

/// Join two balances together.
pub fn join(b: &mut Balance, balance: Balance) -> u64 {
    let Balance { value } = balance;
    b.value = b.value + value;
    b.value
}

/// Split a `Balance` and take a sub balance from it.
pub fn split(b: &mut Balance, value: u64) -> Balance {
    assert!(b.value >= value, "{}", ENotEnough);
    b.value = b.value - value;
    Balance { value }
}

/// Withdraw all balance. After this the remaining balance must be 0.
pub fn withdraw_all(b: &mut Balance) -> Balance {
    let value = b.value;
    split(b, value)
}

/// Destroy a zero `Balance`.
pub fn destroy_zero(balance: Balance) {
    assert!(balance.value == 0, "{}", ENonZero);
    let Balance { value: _ } = balance;
}

/// Destroy a `Supply` preventing any further minting and burning.
pub fn destroy_supply(s: Supply) -> u64 {
    let Supply { value } = s;
    value
}