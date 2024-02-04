use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{Err, IdoPoolAccount, WhitelistAccount};

pub fn deposit_ticket(ctx: Context<DepositTicketContext>, amount: u64) -> Result<()> {
    //first time deposit? ok increase unique
    if ctx.accounts.whitelist_account.total_deposit == 0 {
        ctx.accounts.pool_account.pool_stats.unique_contributor = ctx
            .accounts
            .pool_account
            .pool_stats
            .unique_contributor
            .checked_add(1)
            .unwrap();

        ctx.accounts
            .pool_account
            .pool_stats
            .ticket_unique_contributor = ctx
            .accounts
            .pool_account
            .pool_stats
            .ticket_unique_contributor
            .checked_add(1)
            .unwrap();
    }

    ctx.accounts.whitelist_account.total_deposit = ctx
        .accounts
        .whitelist_account
        .total_deposit
        .checked_add(amount)
        .unwrap();

    if ctx.accounts.whitelist_account.total_deposit < ctx.accounts.pool_account.wallet_min_cap {
        msg!("min cap has not been satisfied");
        return err!(Err::ErrPermissionDenied);
    }

    //successfully added, now check if we exceeded
    if ctx.accounts.whitelist_account.settled_allocation
        < ctx.accounts.whitelist_account.total_deposit
    {
        return err!(Err::ErrMaxAllocationExceeded);
    }

    msg!(
        "your deposit/allocation is : {}/{}",
        ctx.accounts.whitelist_account.total_deposit,
        ctx.accounts.whitelist_account.settled_allocation
    );

    //update pool stats
    ctx.accounts
        .pool_account
        .pool_stats
        .ticket_total_contribution = ctx
        .accounts
        .pool_account
        .pool_stats
        .ticket_total_contribution
        .checked_add(amount)
        .unwrap();

    ctx.accounts.pool_account.pool_stats.total_contribution = ctx
        .accounts
        .pool_account
        .pool_stats
        .total_contribution
        .checked_add(amount)
        .unwrap();

    if ctx.accounts.pool_account.pool_stats.total_contribution
        >= ctx.accounts.pool_account.pool_hard_cap
    {
        return err!(Err::ErrSoldOut);
    }

    // do the transfer
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Transfer {
                from: ctx
                    .accounts
                    .quote_token_user_account
                    .to_account_info()
                    .clone(),
                to: ctx.accounts.quote_token_account.to_account_info().clone(),
                authority: ctx.accounts.participant.to_account_info().clone(),
            },
        ),
        amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct DepositTicketContext<'info> {
    #[account(mut,
    constraint = quote_token_account.mint == pool_account.quote_mint,
    constraint = whitelist_account.pool == pool_account.key(),
    constraint = quote_token_user_account.mint == pool_account.quote_mint,
    constraint = quote_mint.key() == pool_account.quote_mint
    )]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut,
    constraint = whitelist_account.owner == participant.key()
    )]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

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

    #[account(mut)]
    pub participant: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
