use crate::example_contracts::simple_ds_chief::simple_ds_chief::{Address, DSChief, SimpleDSChief__SimpleDSChief};
use crate::sui_std::table::table;
use crate::sui_std::table::table::{Table, Key};
use kani::Arbitrary;
use std::collections::{HashMap, HashSet};

fn bounded_any() -> u64 {
    kani::any_where(|x: &u64| *x == 0 || *x == 1 || *x == 2)
}

impl Arbitrary for Address {
    // Custom method to generate arbitrary `Address`
    fn any() -> Self {
        // Generate arbitrary `u64` value for `id`
        let id: u64 = bounded_any();
        Address {
            id: id,
        }
    }
}

pub fn arbitrary_hashmap<K, V>() -> HashMap<K, V>
where
    K: Arbitrary + Eq + std::hash::Hash + Clone,
    V: Arbitrary + Clone,
{
    let mut map = HashMap::new();
    let size: u8 = bounded_any() as u8;
    kani::assume(size < 3); // Limit the map size

    for _ in 0..size {
        let key: K = kani::any();
        let value: V = kani::any();
        map.insert(key, value);
    }

    map
}

impl<K: Key + Arbitrary + Clone, V: Arbitrary + Clone> Arbitrary for Table<K, V> {
    // Custom method to generate arbitrary `Address`
    fn any() -> Self {
        // Generate arbitrary `u64` value for `id`
        let map = arbitrary_hashmap();
        let size = map.len();
        let id = bounded_any() as u8;
        Table {
            id: id,
            map: map,
            size: size as u8, // Cast is safe as size is less than 10
        }
    }
}

impl Arbitrary for DSChief {
    // Custom method to generate arbitrary `Address`
    fn any() -> Self {
        let mut common_keys = HashSet::new();
        let size: u8 = kani::any_where(|x| *x > 0 && *x <= 5); // Prevent excessive keys

        for _ in 1..size { // No empty cases
            common_keys.insert(Address::any());
        }

        let mut votes = table::new();
        let mut approvals = table::new();
        let mut deposits = table::new();

        for key in &common_keys {
            table::add(&mut votes, key.clone(), bounded_any());
            table::add(&mut approvals, key.clone(), bounded_any());
            table::add(&mut deposits, key.clone(), bounded_any());
        }

        let id: u8 = bounded_any() as u8;

        // Generate arbitrary `u64` value for `id`
        let dschief = DSChief {
            id: id,
            slates: kani::any(),
            votes: votes,
            approvals: approvals,
            deposits: deposits,
        };
        for key in dschief.slates.map.keys() {
            kani::assume(*key == 0 || *key == 1 || *key == 2);
        };
        

        dschief
    }
}

pub fn get_random_key<K, V>(table: &Table<K, V>) -> K
where
    K: Key + Arbitrary + Clone, V: Arbitrary + Clone
{
    let keys: Vec<K> = table.map.keys().cloned().collect();
    let idx: usize = kani::any_where(|x| *x < keys.len()); // Ensure the index is valid

    keys[idx].clone()
}


//#[kani::unwind(3)]
#[kani::proof]
pub fn try_generic_dschief() {
    let mut dschief: DSChief = kani::any();
    
    run_loop_iteration(&mut dschief);
}


pub fn run_loop_iteration(dschief: &mut DSChief) {
    let address: Address = get_random_key(&dschief.deposits);
    let address2: Address = get_random_key(&dschief.deposits);
    let x: u8 = bounded_any() as u8;
    let wad: u64 = bounded_any();

    match x {
        0=> SimpleDSChief__SimpleDSChief::lock(dschief, &address, wad),
        1=> SimpleDSChief__SimpleDSChief::free(dschief, &address, wad),
        2=> {SimpleDSChief__SimpleDSChief::voteYays(dschief, &address, address2);},
        _=>{}
    }
}

// Need to assume this invariant, and possibly assume the sender is in the hashmaps.

//function checkAnInvariant() public {
//    bytes32 senderSlate = votes[msg.sender];
//    address option = slates[senderSlate];
//    uint256 senderDeposit = deposits[msg.sender];
//    assert(approvals[option] >= senderDeposit);
//}