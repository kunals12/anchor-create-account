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