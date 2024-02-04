use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{Err, IdoPoolAccount, WhitelistAccount};

pub fn deposit_nft(ctx: Context<DepositNftContext>, amount: u64) -> Result<()> {
    //transfer did not happen, reject deposit
    if ctx.accounts.nft_lock_account.amount == 0 {
        msg!("there is no locked nft, needs to reject here");
        return err!(Err::ErrPermissionDenied);
    }

    //first time deposit? ok increase unique
    if ctx.accounts.whitelist_account.total_deposit == 0 {
        ctx.accounts.pool_account.pool_stats.unique_contributor = ctx
            .accounts
            .pool_account
            .pool_stats
            .unique_contributor
            .checked_add(1)
            .unwrap();

        ctx.accounts.pool_account.pool_stats.nft_unique_contributor = ctx
            .accounts
            .pool_account
            .pool_stats
            .nft_unique_contributor
            .checked_add(1)
            .unwrap();
    }

    ctx.accounts.whitelist_account.total_deposit = (ctx.accounts.whitelist_account.total_deposit
        as u128)
        .checked_add(amount as u128)
        .unwrap() as u64;

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
    ctx.accounts.pool_account.pool_stats.nft_total_contribution =
        (ctx.accounts.pool_account.pool_stats.nft_total_contribution as u128)
            .checked_add(amount as u128)
            .unwrap() as u64;

    ctx.accounts.pool_account.pool_stats.total_contribution =
        (ctx.accounts.pool_account.pool_stats.total_contribution as u128)
            .checked_add(amount as u128)
            .unwrap() as u64;

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
pub struct DepositNftContext<'info> {
    #[account(mut,
    constraint = quote_token_account.mint == pool_account.quote_mint,
    constraint = quote_token_user_account.mint == pool_account.quote_mint,
    constraint = quote_mint.key() == pool_account.quote_mint,
    )]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut,
    constraint = whitelist_account.pool == pool_account.key(),
    constraint = whitelist_account.owner == participant.key(),
    )]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

    #[account(
    mut,
    seeds = [pool_account.key().as_ref(), b"nft_lock_account".as_ref(), nft_mint.key().as_ref()],
    bump
    )]
    pub nft_lock_account: Box<Account<'info, TokenAccount>>,

    // #[account(mut, constraint = nft_user_account.mint == nft_mint.key())]
    // pub nft_user_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = nft_mint.key() == whitelist_account.locked_nft_mint)]
    pub nft_mint: Account<'info, Mint>,

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
