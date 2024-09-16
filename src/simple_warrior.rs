    pub struct Warrior {
        id: u8,
        sword: Option<Sword>,
    }

    pub struct Sword {
        id: u8,
        strength: u8,
    }


pub struct IdGetter {
    current_id: u32,
}

impl IdGetter {
    pub fn new() -> Self {
        IdGetter { current_id: 0 }
    }
    
    pub fn get_new_id(&mut self) -> u32 {
        self.current_id += 1;
        self.current_id
    }
}

// Create a global instance (for single-threaded or non-global use)
pub static mut ID_GETTER: IdGetter = IdGetter::new();

// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Demonstrates wrapping objects using the `Option` type.
pub struct simple_warrior__example {}
impl simple_warrior__example {
    /// Warrior already has a Sword equipped.
    const EAlreadyEquipped: u64 = 0;

    /// Warrior does not have a sword equipped.
    const ENotEquipped: u64 = 1;

    pub fn new_sword(strength: u8, ctx: &mut TxContext) -> Sword {
        Sword { id: ID_GETTER.get_new_id(), strength }
    }

    pub fn new_warrior(ctx: &mut TxContext) -> Warrior {
        Warrior { id: ID_GETTER.get_new_id(), sword: option::none() }
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