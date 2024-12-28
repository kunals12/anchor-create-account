# Create Account

We're going to create a Solana account.
   
This account is going to be a **system account** - meaning it will be owned by the System Program. In short, this means only the System Program will be allowed to modify it's data.   

In the test, we use two methods for creating the accounts. One of the methods uses Cross program invocation and the other calls the System Program directly. 

Cross program invocation means that we send the transaction to create the account first to our deployed Solana Program, which then calls the System Program. See [here](https://github.com/solana-developers/program-examples/tree/main/basics/cross-program-invocation) for more Cross Program Invocation examples. 

Calling the System Program directly means that the client sends the transaction to create the account directly to the Solana Program
   
In this example, this account will simply hold some SOL.

### Links:
- [Solana Cookbook - How to Create a System Account](https://solanacookbook.com/references/accounts.html#how-to-create-a-system-account)
- [Rust Docs - solana_program::system_instruction::create_account](https://docs.rs/solana-program/latest/solana_program/system_instruction/fn.create_account.html)


# Create System Account Program

This program demonstrates how to use the Solana system program's `create_account` function via a Cross-Program Invocation (CPI) to create a new account on the Solana blockchain.

## **Program Flow**

1. **User Invokes the `initialize` Instruction:**
   - The `initialize` instruction is invoked by the user through the program.

2. **Program Logs Messages:**
   - The program logs information, such as the public key of the new account that will be created.

3. **Calculate Required Lamports:**
   - The program uses the Solana **rent system** to calculate the minimum balance required to exempt the new account from being rent-deducted. This is done using `Rent::get()?.minimum_balance(0)`.

4. **Create a New Account Using CPI:**
   - The program calls the Solana **system program**'s `create_account` instruction through a Cross-Program Invocation (CPI) using the `create_account` function. 
   - The CPI creates a new system account using the provided public key (`new_account`) with the specified lamports and space (set to `0` here).

5. **Account is Created:**
   - The new account (`new_account`) is funded with lamports and initialized with zero space.

## **How Account Creation Works**

1. **Role of `new_account` in the Struct:**
   - The `new_account` field in the `CreateSystemAccount` struct represents the account that will be created.
   - It is marked as `mut`, meaning it can be modified (e.g., initialized with lamports during account creation).

2. **Account Address Determination:**
   - The program uses the public key of the `new_account` account provided by the user when invoking the instruction. The account creation does not derive a PDA but directly uses the provided key.

3. **System Program CPI:**
   - The `create_account` function is a **CPI to the system program** that creates a new system account with:
     - **From Account:** `user` (payer of lamports).
     - **To Account:** `new_account` (account being created).
     - **Lamports:** Rent-exemption balance.
     - **Space:** 0 bytes (no data allocated to the account).
     - **Owner Program:** The system program itself (`&ctx.accounts.system_program.key()`).

## **What Happens in `initialize`**

- The `create_account` function transfers lamports from `user` to the newly created account (`new_account`).
- The `new_account` is then registered as a valid account on the Solana blockchain with the specified lamports.

## **Role of `new_account` in the Struct**

- **Signer:**
  - The account must sign the transaction, proving it has been created and authorized by its private key.

- **Mutable (`mut`):**
  - This allows it to be modified during the execution of the instruction. In this case, its balance is updated and registered as a system account.

## **Why Do We Need `new_account` to Be a Signer?**

- **Keypair Generation:**
  - The user generates a new keypair locally (off-chain) for the `new_account`.
  - The public key of this keypair is passed as the `new_account` parameter when invoking the `initialize` instruction.

- **Signing Requirement:**
  - The `new_account` must sign the transaction to prove ownership and allow the program to create the account on-chain.

## **Code with Comments**

```rust
use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};

// Declare the program ID
declare_id!("GwTuckQCY4N2oWFggerXuYFvEhhjv4989YvmVjMwQSB");

#[program]
pub mod create_account {
    use super::*;

    /// Initializes the process to create a new system account.
    pub fn initialize(ctx: Context<CreateSystemAccount>) -> Result<()> {
        // Log a message indicating the program is invoked
        msg!("Program invoked. Creating a system account...");

        // Log the public key of the new account to be created
        msg!(
            "  New public key will be: {}",
            &ctx.accounts.new_account.key().to_string()
        );

        // Calculate the minimum lamports required for rent exemption
        let lamports = (Rent::get()?).minimum_balance(0);

        // CPI to the system program to create the account
        create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(), // Reference to the system program
                CreateAccount {
                    from: ctx.accounts.user.to_account_info(), // Account paying for the creation
                    to: ctx.accounts.new_account.to_account_info(), // Account being created
                },
            ),
            lamports, // Amount of lamports to fund the new account
            0,        // Space for the new account (0 bytes)
            &ctx.accounts.system_program.key(), // Owner of the new account
        )?;

        Ok(())
    }
}

/// Context for the `initialize` instruction
#[derive(Accounts)]
pub struct CreateSystemAccount<'info> {
    /// The user who will pay for creating the new account
    #[account(mut)]
    pub user: Signer<'info>,

    /// The new account to be created
    #[account(mut)]
    pub new_account: Signer<'info>,

    /// The system program responsible for creating accounts
    pub system_program: Program<'info, System>,
}
```

## **Final Notes**

1. **CPI to System Program:**
   - The `create_account` function delegates the responsibility of creating the new account to the system program, leveraging its inherent capability to initialize accounts on the Solana blockchain.

2. **Space = `0`:**
   - The account is initialized with no space (`0` bytes), meaning it cannot store any data. This is likely for demonstration or as a placeholder.

3. **Rent-Exemption:**
   - The calculated `lamports` ensure the account is rent-exempt, meaning it won’t be deleted due to insufficient funds over time.

---

Let me know if you have further questions or need additional clarifications!
