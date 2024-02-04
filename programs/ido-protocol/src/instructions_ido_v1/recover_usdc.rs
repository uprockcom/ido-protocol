use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{ IdoPoolAccount, WhitelistAccount, ADMIN_PUBKEY};

pub fn recover_usdc(ctx: Context<RecoverUsdcContext>) -> Result<()> {
    assert_eq!(
        ADMIN_PUBKEY,
        ctx.accounts.admin.key.to_string().as_bytes()
    );

    if ctx.accounts.whitelist_account.total_deposit == 0 {
        return Ok(());
    }

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
        ctx.accounts.whitelist_account.total_deposit,
    )?;

    //life saver
    ctx.accounts.whitelist_account.total_deposit = 0;

    Ok(())
}


#[derive(Accounts)]
pub struct RecoverUsdcContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut, constraint = whitelist_account.pool == pool_account.key())]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: authority is checked, no worries
    #[account(mut)]
    pub participant: AccountInfo<'info>,

    #[account(
    mut,
    seeds = [pool_account.key().as_ref(), b"quote_token_account".as_ref(), quote_mint.key().as_ref()],
    bump
    )]
    pub quote_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_token_user_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}