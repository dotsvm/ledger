
# ledger program (anchor)

a simple solana program built using anchor that implements a basic ledger system.

## overview

this program allows:

* creating user accounts with balances
* transferring balance between users
* validating ownership and signer authorization

the goal of this project is to understand how anchor works on top of the solana runtime while building a real program.

---

## architecture

the project follows a modular anchor structure:

```
programs/ledger/src/
├── lib.rs
├── state/
│   ├── mod.rs
│   └── user_account.rs
├── instructions/
│   ├── mod.rs
│   ├── initialize.rs
│   └── transfer.rs
```

### state

`user_account` defines on-chain data:

* owner: pubkey
* balance: u64

account size:

* 8 bytes discriminator
* 32 bytes pubkey
* 8 bytes balance
* total: 48 bytes

---

### instructions

#### initialize

creates a new user account.

* allocates space (48 bytes)
* payer funds rent
* sets owner = signer
* sets balance = 0

#### transfer

transfers balance from one user to another.

validations:

* signer must match sender.owner
* sender must have enough balance

state changes:

* sender.balance -= amount
* receiver.balance += amount

---

## key concepts

### accounts vs program

* program: executable code deployed on solana
* accounts: store all state

the program is stateless. all data lives in accounts.

---

### anchor abstractions

anchor simplifies:

* account validation
* serialization and deserialization
* instruction decoding

mapping:

* account<T> → wraps accountinfo + deserialization
* signer → checks is_signer
* #[account(mut)] → ensures writable

---

### system program

used for account creation.

anchor internally performs a cpi to the system program when using:

```
#[account(init, payer = signer, space = ...)]
```

---

### rent

accounts must hold enough lamports to be rent-exempt.

* rent is based on account size
* payer funds the account during creation

---

## security

the critical check in transfer:

```
sender.owner == signer.key()
```

without this:

* any user could transfer funds from any account

---

## testing

tests are written using anchor (typescript).

flow:

* create user accounts
* call initialize
* call transfer
* fetch and verify balances

run tests:

```
anchor test
```

---

## notes

* solana programs do not store state internally
* all state must be explicitly passed as accounts
* account size is fixed at creation
* incorrect space allocation can break accounts
* ownership validation is required for security

---

## future improvements

* add deposit instruction
* add withdrawal logic
* add events for transfers
* handle account upgrades and resizing
* implement custom error types using anchor

---

## learning goals

this project demonstrates:

* how anchor maps to low-level solana concepts
* how accounts are created and managed
* how instruction handlers work
* how to write secure on-chain logic

---
