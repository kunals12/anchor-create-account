use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};

declare_id!("GwTuckQCY4N2oWFggerXuYFvEhhjv4989YvmVjMwQSB");

#[program]
pub mod create_account {
    use super::*;

    pub fn initialize(ctx: Context<CreateSystemAccount>) -> Result<()> {
        msg!("Program invoked. Creating a system account...");
        msg!(
            "  New public key will be: {}",
            &ctx.accounts.new_account.key().to_string()
        );
        let lamports = (Rent::get()?).minimum_balance(0);
        create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.new_account.to_account_info(),
                },
            ),
            lamports,
            0,
            &ctx.accounts.system_program.key(),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateSystemAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub new_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}
