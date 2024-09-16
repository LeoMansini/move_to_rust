// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Demonstrates wrapping objects using the `Option` type.
impl simple_warrior__example {
    pub struct Sword {
        id: u8,
        strength: u8,
    }

    pub struct Warrior {
        id: u8,
        sword: Option<Sword>,
    }

    /// Warrior already has a Sword equipped.
    const EAlreadyEquipped: u64 = 0;

    /// Warrior does not have a sword equipped.
    const ENotEquipped: u64 = 1;

    pub fn new_sword(strength: u8, ctx: &mut TxContext) -> Sword {
        Sword { id: object::new(ctx), strength }
    }

    pub fn new_warrior(ctx: &mut TxContext) -> Warrior {
        Warrior { id: object::new(ctx), sword: option::none() }
    }

    pub fn equip(warrior: &mut Warrior, sword: Sword) {
        assert!(option::is_none(&warrior.sword), EAlreadyEquipped);
        option::fill(&mut warrior.sword, sword);
    }

    pub fn unequip(warrior: &mut Warrior) -> Sword {
        assert!(option::is_some(&warrior.sword), ENotEquipped);
        option::extract(&mut warrior.sword)
    }

}