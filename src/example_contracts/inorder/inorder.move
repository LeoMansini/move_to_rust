

module inorder::inorder {

    const EACallRequired: u64 = 0;
    const EBCallRequired: u64 = 1;
    const ECCallRequired: u64 = 2;
    const EInvariantBroken: u64 = 3;

    public struct CallRegistry has key {
        id: UID,
        num_calls: u64,
        a_called: bool,
        b_called: bool,
        c_called: bool,
    }

    fun init(ctx: &mut TxContext) {
        let call_registry = CallRegistry {
            id: object::new(ctx),
            num_calls: 0,
            a_called: false,
            b_called: false,
            c_called: false,
        };

        let veriman = call_registry.a_called;
        assert!(!veriman || call_registry.a_called, EInvariantBroken);

        transfer::share_object(call_registry);
    }

    public fun a(call_registry: &mut CallRegistry) {
        let veriman = call_registry.a_called;
        call_registry.a_called = true;
        call_registry.num_calls++;

        assert!(!veriman || call_registry.a_called, EInvariantBroken);
    }

    public fun b(call_registry: &mut CallRegistry) {
        let veriman = call_registry.a_called;
        assert!(call_registry.a_called, EACallRequired);
        
        if call_registry.b_called {
            call_registry.num_calls = 0;
            assert!(!veriman || call_registry.a_called, EInvariantBroken);
            return;
        }

        call_registry.b_called = true;
        call_registry.num_calls++;

        assert!(!veriman || call_registry.a_called, EInvariantBroken);
    }

    public fun c(call_registry: &mut CallRegistry): u64 {
        assert!(call_registry.a_called, EACallRequired);
        assert!(call_registry.b_called, EBCallRequired);
        
        call_registry.c_called = true;
        call_registry.num_calls++;

        return 3;
    }

    
}   