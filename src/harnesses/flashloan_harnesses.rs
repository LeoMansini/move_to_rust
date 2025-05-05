
use crate::example_contracts::flashloan::flashloan::flashloan__flashloan;
use crate::sui_std::coin::coin;
use crate::sui_std::balance::balance;

#[kani::proof]
#[kani::should_panic]
fn try_flashloan() {
    let mut pool = flashloan__flashloan::init();
    let mut b = balance::zero();
    let mut c = coin::from_balance(b);

    flashloan__flashloan::borrow(&mut pool, 100);
    
}