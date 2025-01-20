
module SimpleDSChief::SimpleDSChief {

    const EAddShouldBeGreater: u64 = 0;
    const ESubShouldBeSmaller: u64 = 1;

    use sui::table::Table;

    public struct Address {
        id: u64,
    }


    public struct DSChief has key {
        id: UID,
        slates: Table<u64, Address>,
        votes: Table<Address, u64>,
        approvals: Table<Address, u64>,
        deposits: Table<Address, u64>,
    }

    public fun lock(chief: &mut DSChief, sender: Address, wad: u64) {
        chief.deposits[sender] = add(chief.deposits[sender], wad);
        addWeight(chief, wad, chief.votes[sender]);
    }

    public fun free(chief: &mut DSChief, sender: Address, wad: u64) {
        chief.deposits[sender] = sub(chief.deposits[sender], wad);
        subWeight(chief, wad, chief.votes[sender]);
    }
    
    public fun voteYays(chief: &mut DSChief, sender: Address, yay: Address) -> u64 {
        let slate: u64 = etch(chief, yay);
        voteSlate(chief, slate);

        return slate
    }

    public fun etch(chief: &mut DSChief, yay: Address) -> u64 {
        let slate = yay.id; // way around hashing
        chief.slates[slate] = yay;
        return slate
    }

    public fun voteSlate(chief: &mut DSChief, sender: Address, slate: u64) {
        let weight: u64 = chief.deposits[sender];
        subWeight(chief, weighyt, chief.votes[sender]);
        chief.votes[sender] = slate;
        addWeight(chief, weight, chief.votes[sender]);
    }

    public fun addWeight(chief: &mut DSChief, weight: u64, slate: u64) {
        let yay: Address = chief.slates[slate];
        chief.approvals[yay] = add(chief.approvals[yay], weight);
    }

    public fun subWeight(chief: &mut DSChief, weight: u64, slate: u64) {
        let yay: Address = chief.slates[slate];
        chief.approvals[yay] = sub(chief.approvals[yay], weight);
    }

    public fun add(x: u64, y: u64) -> u64 {
        let z: u64 = x + y;
        assert!(z >= x, EAddShouldBeGreater);

        return z;
    }

    public fun sub(x: u64, y: u64) -> u64 {
        let z: u64 = x - y;
        assert!(z <= x, ESubShouldBeSmaller);

        return z;
    }


}