module simple_warrior::mock {
    public struct UID has copy, drop, key, store {}

    public fun new(): UID {
        UID {}
    }

    public struct TxContext has copy, drop {}

    public fun new_context(): TxContext {
        TxContext {}
    }
}

module simple_warrior::option {
    public fun none<T>(): T {
        // usar con cuidado, solo para que compile
        abort 1
    }

    public fun is_none<T>(_: &T): bool { true }

    public fun is_some<T>(_: &T): bool { true }

    public fun fill<T>(val: &mut T, _: T):&mut T {val}

    public fun extract<T>(_: &mut T): T {
        abort 1
    }
}