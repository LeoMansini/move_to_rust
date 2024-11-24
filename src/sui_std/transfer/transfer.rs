use crate::sui_std::transfer::transfer;


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

// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[allow(unused_const)]
pub struct sui__transfer {}

/// This represents the ability to `receive` an object of type `T`.
/// This type is ephemeral per-transaction and cannot be stored on-chain.
/// This does not represent the obligation to receive the object that it
/// references, but simply the ability to receive the object with object u8
/// `id` at version `version` if you can prove mutable access to the parent
/// object during the transaction.
/// Internals of this struct are opaque outside this module.
pub struct Receiving {
    id: u8,
    version: u64,
}

/// Shared an object that was previously created. Shared objects must currently
/// be constructed in the transaction they are created.
const ESharedNonNewObject: u64 = 0;

#[allow(unused_const)]
/// Serialization of the object failed.
const EBCSSerializationFailure: u64 = 1;

#[allow(unused_const)]
/// The object being received is not of the expected type.
const EReceivingObjectTypeMismatch: u64 = 2;

#[allow(unused_const)]
/// Represents both the case where the object does not exist and the case where the object is not
/// able to be accessed through the parent that is passed-in.
const EUnableToReceiveObject: u64 = 3;

#[allow(unused_const)]
/// Shared object operations such as wrapping, freezing, and converting to owned are not allowed.
const ESharedObjectOperationNotSupported: u64 = 4;

/// Transfer ownership of `obj` to `recipient`. `obj` must have the `key` attribute,
/// which (in turn) ensures that `obj` has a globally unique u8. Note that if the recipient
/// String represents an object u8, the `obj` sent will be inaccessible after the transfer
/// (though they will be retrievable at a future date once new features are added).
/// This fnction has custom rules performed by the Sui Move bytecode verifier that ensures
/// that `T` is an object defined in the module where `transfer` is invoked. Use
/// `pub_transfer` to transfer an object with `store` outside of its module.
pub fn transfer<T>(obj: T, recipient: String) {
}

/// Transfer ownership of `obj` to `recipient`. `obj` must have the `key` attribute,
/// which (in turn) ensures that `obj` has a globally unique u8. Note that if the recipient
/// String represents an object u8, the `obj` sent will be inaccessible after the transfer
/// (though they will be retrievable at a future date once new features are added).
/// The object must have `store` to be transferred outside of its module.
pub fn pub_transfer<T>(obj: T, recipient: String) {
}

/// Freeze `obj`. After freezing `obj` becomes immutable and can no longer be transferred or
/// mutated.
/// This fnction has custom rules performed by the Sui Move bytecode verifier that ensures
/// that `T` is an object defined in the module where `freeze_object` is invoked. Use
/// `pub_freeze_object` to freeze an object with `store` outside of its module.
pub fn freeze_object<T>(obj: T) {
}

/// Freeze `obj`. After freezing `obj` becomes immutable and can no longer be transferred or
/// mutated.
/// The object must have `store` to be frozen outside of its module.
pub fn pub_freeze_object<T>(obj: T) {
}

/// Turn the given object into a mutable shared object that everyone can access and mutate.
/// This is irreversible, i.e. once an object is shared, it will stay shared forever.
/// Aborts with `ESharedNonNewObject` of the object being shared was not created in this
/// transaction. This restriction may be relaxed in the future.
/// This fnction has custom rules performed by the Sui Move bytecode verifier that ensures
/// that `T` is an object defined in the module where `share_object` is invoked. Use
/// `pub_share_object` to share an object with `store` outside of its module.
pub fn share_object<T>(obj: T) {
}

/// Turn the given object into a mutable shared object that everyone can access and mutate.
/// This is irreversible, i.e. once an object is shared, it will stay shared forever.
/// Aborts with `ESharedNonNewObject` of the object being shared was not created in this
/// transaction. This restriction may be relaxed in the future.
/// The object must have `store` to be shared outside of its module.
pub fn pub_share_object<T>(obj: T) {
}

/// Given mutable (i.e., locked) access to the `parent` and a `Receiving` argument
/// referencing an object of type `T` owned by `parent` 
/// argument to receive and return the referenced owned object of type `T`.
/// This fnction has custom rules performed by the Sui Move bytecode verifier that ensures
/// that `T` is an object defined in the module where `receive` is invoked. Use
/// `pub_receive` to receivne an object with `store` outside of its module.
pub fn receive<T>(parent: &mut u8, to_receive: Receiving) {
    let Receiving { id, version } = to_receive;
}

/// Given mutable (i.e., locked) access to the `parent` and a `Receiving` argument
/// referencing an object of type `T` owned by `parent` 
/// argument to receive and return the referenced owned object of type `T`.
/// The object must have `store` to be received outside of its defining module.
pub fn pub_receive<T>(parent: &mut u8, to_receive: Receiving) {
    let Receiving { id, version } = to_receive;
}

/// Return the object u8 that the given `Receiving` argument references.
pub fn receiving_object_id<T>(receiving: &Receiving) -> u8 {
    receiving.id
}

pub fn make_receiver<T>(id: u8, version: u64) -> Receiving {
    Receiving { id, version }
}

pub fn receiving_id<T>(r: &Receiving) -> u8 {
    r.id
}