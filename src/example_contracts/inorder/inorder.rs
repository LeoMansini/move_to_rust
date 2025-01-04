use crate::sui_std::transfer::transfer;

pub struct CallRegistry {
    id: u8,
    numCalls: u64,
    a_called: bool,
    b_called: bool,
    c_called: bool,
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

pub struct inorder__inorder {}
impl inorder__inorder {

    const EACallRequired: u64 = 0;
    const EBCallRequired: u64 = 1;
    const ECCallRequired: u64 = 2;
    const EInvariantBroken: u64 = 3;

    fn init() {
        let call_registry = CallRegistry {
            id: ID_GETTER.get_new_id(),
            numCalls: 0,
            a_called: false,
            b_called: false,
            c_called: false,
        };

        let veriman = call_registry.a_called;
        assert!(!veriman || call_registry.a_called, "{}", EInvariantBroken);

        transfer::share_object(call_registry);
    }

    pub fn a(call_registry: &mut CallRegistry) {
        let veriman = call_registry.a_called;
        call_registry.a_called = true;
        call_registry.numCalls+=1;

        assert!(!veriman || call_registry.a_called, "{}", EInvariantBroken);
    }

    pub fn b(call_registry: &mut CallRegistry) {
        let veriman = call_registry.a_called;
        assert!(call_registry.a_called, "{}", EACallRequired);
        
        if call_registry.b_called {
            call_registry.numCalls = 0;
            assert!(!veriman || call_registry.a_called, "{}", EInvariantBroken);
            return;
        }

        call_registry.b_called = true;
        call_registry.numCalls+=1;

        assert!(!veriman || call_registry.a_called, "{}", EInvariantBroken);
    }

    pub fn c(call_registry: &mut CallRegistry) {
        assert!(call_registry.a_called, "{}", EACallRequired);
        assert!(call_registry.b_called, "{}", EBCallRequired);
        
        call_registry.c_called = true;
        call_registry.numCalls+=1;

        return 3;
    }

}   