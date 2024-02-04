use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{ IdoPoolAccount };

pub fn force_raise(ctx: Context<ForceRaiseContext>) -> Result<()> {
    if ctx.accounts.quote_token_account.amount > 0 {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info().clone(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.quote_token_account.to_account_info().clone(),
                    to: ctx
                        .accounts
                        .quote_token_user_account
                        .to_account_info()
                        .clone(),
                    authority: ctx.accounts.quote_token_account.to_account_info().clone(),
                },
                &[&[
                    ctx.accounts.pool_account.key().as_ref(),
                    b"quote_token_account".as_ref(),
                    ctx.accounts.pool_account.quote_mint.as_ref(),
                    &[*ctx.bumps.get("quote_token_account").unwrap()],
                ]],
            ),
            ctx.accounts.quote_token_account.amount,
        )?;
    }

    ctx.accounts.pool_account.is_raised = true;
    Ok(())
}


#[derive(Accounts)]
pub struct ForceRaiseContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
    seeds = [pool_account.key().as_ref(), b"quote_token_account".as_ref(), pool_account.quote_mint.as_ref()],
    bump,
    constraint = quote_token_account.mint == pool_account.quote_mint,
    )]
    pub quote_token_account: Account<'info, TokenAccount>,

    #[account(mut,
    constraint = quote_token_user_account.mint == pool_account.quote_mint,
    )]
    pub quote_token_user_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}