use crate::example_contracts::simple_ds_chief::simple_ds_chief::{Address, DSChief, SimpleDSChief__SimpleDSChief};
use crate::sui_std::table::table;
use crate::sui_std::table::table::{Table, Key};
use kani::Arbitrary;
use std::collections::{HashMap, HashSet};

impl Arbitrary for Address {
    // Custom method to generate arbitrary `Address`
    fn any() -> Self {
        // Generate arbitrary `u64` value for `id`
        Address {
            id: kani::any(),
        }
    }
}

pub fn arbitrary_hashmap<K, V>() -> HashMap<K, V>
where
    K: Arbitrary + Eq + std::hash::Hash + Clone,
    V: Arbitrary + Clone,
{
    let mut map = HashMap::new();
    let size: u8 = kani::any();
    kani::assume(size < 10); // Limit the map size

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
        Table {
            id: kani::any(),
            map: map,
            size: size as u8, // Cast is safe as size is less than 10
        }
    }
}

impl Arbitrary for DSChief {
    // Custom method to generate arbitrary `Address`
    fn any() -> Self {
        let mut common_keys = HashSet::new();
        let size: u8 = kani::any();
        kani::assume(size > 0 && size <= 5); // Prevent excessive keys

        for _ in 1..size { // No empty cases
            common_keys.insert(Address::any());
        }

        let mut votes = table::new();
        let mut approvals = table::new();
        let mut deposits = table::new();

        for key in &common_keys {
            table::add(&mut votes, key.clone(), kani::any());
            table::add(&mut approvals, key.clone(), kani::any());
            table::add(&mut deposits, key.clone(), kani::any());
        }

        // Generate arbitrary `u64` value for `id`
        DSChief {
            id: kani::any(),
            slates: kani::any(),
            votes: votes,
            approvals: approvals,
            deposits: deposits,
        }
    }
}

pub fn get_random_key<K, V>(table: &Table<K, V>) -> K
where
    K: Key + Arbitrary + Clone, V: Arbitrary + Clone
{
    let keys: Vec<K> = table.map.keys().cloned().collect();
    let idx: usize = kani::any();
    kani::assume(idx < keys.len()); // Ensure the index is valid

    keys[idx].clone()
}


#[kani::proof]
#[kani::unwind(3)]
pub fn try_generic_dschief() {
    let mut dschief: DSChief = kani::any();
    
    loop {
        let address: Address = get_random_key(&dschief.deposits);
        let address2: Address = get_random_key(&dschief.deposits);
        let x: u8 = kani::any();
        kani::assume(x < 3);
        match x {
            0=> SimpleDSChief__SimpleDSChief::lock(&mut dschief, &address, kani::any()),
            1=> SimpleDSChief__SimpleDSChief::free(&mut dschief, &address, kani::any()),
            2=> {SimpleDSChief__SimpleDSChief::voteYays(&mut dschief, &address, address2);},
            _=>{}
        }
    }

}

// Need to assume this invariant, and possibly assume the sender is in the hashmaps.

//function checkAnInvariant() public {
//    bytes32 senderSlate = votes[msg.sender];
//    address option = slates[senderSlate];
//    uint256 senderDeposit = deposits[msg.sender];
//    assert(approvals[option] >= senderDeposit);
//}