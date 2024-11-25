use crate::sui_std::balance::balance;
use balance::Balance;
use crate::sui_std::coin::coin;
use coin::Coin;
use crate::sui_std::transfer::transfer;

pub struct NFT {
    id: u8,
    price: Balance,
}

pub struct Loan {
    amount: u64,
}

pub struct LoanPool {
    id: u8,
    amount: Balance,
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

// Copyright (c) Sui Foundation, Inc.
// SPDX-License-Identifier: Apache-2.0

/// For when the loan amount exceed the pool amount
const ELoanAmountExceedPool: u64 = 0;
/// For when the repay amount do not match the initial loan amount
const ERepayAmountInvalid: u64 = 1;

pub struct flashloan__flashloan {}
impl flashloan__flashloan {
    // === Imports ===
    
    // === Errors ===

    

    // === Structs ===

    /// A "shared" loan pool.
    /// For demonstration purpose, we assume the loan pool only allows SUI.
    /// A loan position.
    /// This is a hot potato struct, it enforces the users
    /// to repay the loan in the end of the transaction or within the same PTB.
    /// A dummy NFT to represent the flashloan fnctionality
    pub fn init() -> LoanPool {
        LoanPool { 
            id: ID_GETTER.get_new_id(), 
            amount: balance::zero() 
        }
    }
    // === Public-Mutative Functions ===

    /// Deposit money into loan pool
    pub fn deposit_pool(pool: &mut LoanPool, deposit: Coin) {
        balance::join(&mut pool.amount, coin::into_balance(deposit));
    }

    /// Function allows users to borrow from the loan pool.
    /// It returns the borrowed [`Coin`] and the [`Loan`] position 
    /// enforcing users to fulfill before the PTB ends.
    pub fn borrow(pool: &mut LoanPool, amount: u64, ) -> (Coin, Loan) {
        assert!(amount <= balance::value(&pool.amount), "{}", ELoanAmountExceedPool);

        (
            coin::from_balance(balance::split(&mut pool.amount, amount)),
            Loan {
                amount
            }
        )
    }

    /// Repay the loan
    /// Users must execute this fnction to ensure the loan is repaid before the transaction ends.
    pub fn repay(pool: &mut LoanPool, loan: Loan, payment: Coin) {
        let Loan { amount } = loan;
        assert!(coin::value(&payment) == amount, "{}", ERepayAmountInvalid);

        balance::join(&mut pool.amount, coin::into_balance(payment));
    }

    /// Mint NFT
    pub fn mint_nft(payment: Coin, ) -> NFT {
        NFT {
            id: ID_GETTER.get_new_id(),
            price: coin::into_balance(payment),
        }
    }

    /// Sell NFT
    pub fn sell_nft(nft: NFT, ) -> Coin {
        let NFT {id, price} = nft;
        coin::from_balance(price)
    }
}   