const ENotEquipped: u64 = 1;

const EAlreadyEquipped: u64 = 0;

pub struct Warrior {
    id: u8,
    sword: Option<Sword>,
}

pub struct Sword {
    id: u8,
    strength: u8,
}

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

/// Demonstrates wrapping objects using the `Option` type.
pub struct simple_warrior__example {}
impl simple_warrior__example {
    /// Warrior already has a Sword equipped.

    /// Warrior does not have a sword equipped.

    pub fn new_sword(strength: u8, ) -> Sword {
        Sword { id: ID_GETTER.get_new_id(), strength }
    }

    pub fn new_warrior() -> Warrior {
        Warrior { id: ID_GETTER.get_new_id(), sword: None }
    }

    pub fn equip(warrior: &mut Warrior, sword: Sword) {
        assert!(warrior.sword.is_none(), "{}", EAlreadyEquipped);
        warrior.sword.replace(sword).unwrap();
    }

    pub fn unequip(warrior: &mut Warrior) -> Sword {
        assert!(warrior.sword.is_some(), "{}", ENotEquipped);
        warrior.sword.take().unwrap()
    }

}