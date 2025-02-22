use crate::example_contracts::simple_ds_chief::simple_ds_chief::{Address, DSChief, SimpleDSChief__SimpleDSChief};
use crate::sui_std::table::table::{Table, Key};
use kani::Arbitrary;
use std::collections::HashMap;

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
        // Generate arbitrary `u64` value for `id`
        DSChief {
            id: kani::any(),
            slates: kani::any(),
            votes: kani::any(),
            approvals: kani::any(),
            deposits: kani::any(),
        }
    }
}


#[kani::proof]
#[kani::unwind(3)]
pub fn try_generic_dschief() {
    let mut dschief: DSChief = kani::any();
    while true {
        let x: u8 = kani::any();
        kani::assume(x < 3);
        match x {
            0=> SimpleDSChief__SimpleDSChief::lock(&mut dschief, &kani::any(), kani::any()),
            1=> SimpleDSChief__SimpleDSChief::free(&mut dschief, &kani::any(), kani::any()),
            2=> {SimpleDSChief__SimpleDSChief::voteYays(&mut dschief, &kani::any(), kani::any());},
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