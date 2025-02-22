use crate::example_contracts::simple_ds_chief::simple_ds_chief::{Address, DSChief, SimpleDSChief__SimpleDSChief};
use crate::sui_std::table::table::{Table, Key};
use kani::Arbitrary;
use std::collections::HashMap;

impl Arbitrary for Address {
    // Custom method to generate arbitrary `Address`
    fn arbitrary() -> Self {
        // Generate arbitrary `u64` value for `id`
        Address {
            id: kani::any(),
        }
    }
}

impl<K, V> Arbitrary for HashMap<K, V>
where
    K: Arbitrary + Eq + std::hash::Hash,
    V: Arbitrary,
{
    fn arbitrary() -> Self {
        let mut map = HashMap::new();
        let size = kani::any::<u32>() % 10; // Limit the number of entries to 10

        for _ in 0..size {
            let key = K::arbitrary(); // Generate arbitrary key
            let value = V::arbitrary(); // Generate arbitrary value
            map.insert(key, value);
        }

        map
    }
}

impl<K: Key, V> Arbitrary for Table<K, V> {
    // Custom method to generate arbitrary `Address`
    fn arbitrary() -> Self {
        // Generate arbitrary `u64` value for `id`
        let map: HashMap<K, V> = kani::any();
        Table {
            id: kani::any(),
            map: map,
            size: map.len(),
        }
    }
}

impl Arbitrary for DSChief {
    // Custom method to generate arbitrary `Address`
    fn arbitrary() -> Self {
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
#[kani::unwind(5)]
pub fn try_generic_dschief() {
    let mut dschief: DSChief = kani::any();
    while true {
        let x: u8 = kani::any();
        kani::assume(x < 3);
        match x {
            0=> SimpleDSChief__SimpleDSChief::lock(&mut dschief, kani::any(), kani::any()),
            1=> SimpleDSChief__SimpleDSChief::free(&mut dschief, kani::any(), kani::any()),
            2=> {SimpleDSChief__SimpleDSChief::voteYays(&mut dschief, kani::any(), kani::any());},
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