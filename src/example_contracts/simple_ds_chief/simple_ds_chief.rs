use crate::sui_std::table::table::Table;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Address {
    pub id: u64,
}

pub struct DSChief {
    pub id: u8,
    pub slates: Table<u64, Address>,
    pub votes: Table<Address, u64>,
    pub approvals: Table<Address, u64>,
    pub deposits: Table<Address, u64>,
}

const ESubShouldBeSmaller: u64 = 1;

const EAddShouldBeGreater: u64 = 0;

use std::{ops::Add, sync::LazyLock};

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

pub struct SimpleDSChief__SimpleDSChief {}
impl SimpleDSChief__SimpleDSChief {


    pub fn lock(chief: &mut DSChief, sender: &Address, wad: u64) {
        chief.deposits[sender] = Self::add(chief.deposits[sender], wad);
        Self::addWeight(chief, wad, chief.votes[sender]);
    }

    pub fn free(chief: &mut DSChief, sender: &Address, wad: u64) {
        chief.deposits[sender] = Self::sub(chief.deposits[sender], wad);
        Self::subWeight(chief, wad, chief.votes[sender]);
    }
    
    pub fn voteYays(chief: &mut DSChief, sender: &Address, yay: Address) -> u64 {
        let slate: u64 = Self::etch(chief, yay);
        Self::voteSlate(chief, sender, slate);

        slate
    }

    pub fn etch(chief: &mut DSChief, yay: Address) -> u64 {
        let slate = yay.id; // way around hashing
        chief.slates[&slate] = yay;
        slate
    }

    pub fn voteSlate(chief: &mut DSChief, sender: &Address, slate: u64) {
        let weight: u64 = chief.deposits[sender];
        Self::subWeight(chief, weight, chief.votes[sender]);
        chief.votes[sender] = slate;
        Self::addWeight(chief, weight, chief.votes[sender]);
    }

    pub fn addWeight(chief: &mut DSChief, weight: u64, slate: u64) {
        let yay: Address = chief.slates[&slate];
        chief.approvals[&yay] = Self::add(chief.approvals[&yay], weight);
    }

    pub fn subWeight(chief: &mut DSChief, weight: u64, slate: u64) {
        let yay: Address = chief.slates[&slate];
        chief.approvals[&yay] = Self::sub(chief.approvals[&yay], weight);
    }

    pub fn add(x: u64, y: u64) -> u64 {
        let z: u64 = x + y;
        assert!(z >= x, "{}", EAddShouldBeGreater);

        z
    }

    pub fn sub(x: u64, y: u64) -> u64 {
        let z: u64 = x - y;
        assert!(z <= x, "{}", ESubShouldBeSmaller);

        z
    }

}