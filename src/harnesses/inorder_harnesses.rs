use crate::example_contracts::inorder::inorder::inorder__inorder;

#[kani::proof]
#[kani::unwind(5)]
pub fn try_generic_inorder() {
    let mut call_registry = inorder__inorder::init();
    while true {
        let x: u8 = kani::any();
        kani::assume(x < 3);
        match x {
            0=> inorder__inorder::a(&mut call_registry),
            1=> inorder__inorder::b(&mut call_registry),
            2=> {inorder__inorder::c(&mut call_registry);},
            _=>{}
        }
    }

}