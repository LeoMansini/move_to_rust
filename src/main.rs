mod example_contracts;
mod sui_std;
use example_contracts::simple_warrior::simple_warrior::simple_warrior__example;
use example_contracts::flashloan::flashloan::flashloan__flashloan;
use sui_std::coin::coin;
use sui_std::balance::balance;

//////////////////// SIMPLE WARRIOR

#[kani::proof]
fn try_warrior_succeeds() {
    let mut w = simple_warrior__example::new_warrior();
    let mut s = simple_warrior__example::new_sword(1);
    simple_warrior__example::equip(&mut w, s);
    simple_warrior__example::unequip(&mut w);
}

#[kani::proof]
#[kani::should_panic]
fn try_warrior_unequips_empty() {
    let mut w = simple_warrior__example::new_warrior();
    let mut s = simple_warrior__example::new_sword(1);
    simple_warrior__example::unequip(&mut w);
}

#[kani::proof]
#[kani::should_panic]
fn try_warrior_equips_twice() {
    let mut w = simple_warrior__example::new_warrior();
    let mut s1 = simple_warrior__example::new_sword(1);
    let mut s2 = simple_warrior__example::new_sword(1);
    simple_warrior__example::equip(&mut w, s1);
    simple_warrior__example::equip(&mut w, s2);
}

#[kani::proof]
#[kani::unwind(5)]
fn try_generic() {
    let mut w = simple_warrior__example::new_warrior();
    let mut equip: bool = false;
    while true {
        let x: u8 = kani::any();
        kani::assume(x < 2);
        if x == 0 {
            if !equip {
                equip = true;
                let mut s1 = simple_warrior__example::new_sword(1);
                simple_warrior__example::equip(&mut w, s1);
            }
        }
        else {
            if equip {
                simple_warrior__example::unequip(&mut w);
            }
        }
    }
}


////////////////////// FLASHLOAN

#[kani::proof]
#[kani::should_panic]
fn try_flashloan() {
    let mut pool = flashloan__flashloan::init();
    let mut b = balance::zero();
    let mut c = coin::from_balance(b);

    flashloan__flashloan::borrow(&mut pool, 100);
    
}

fn main(){}