use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{IdoPoolAccount, WhitelistAccount, ADMIN_PUBKEY};

pub fn recover_nft(ctx: Context<RecoverNftContext>) -> Result<()> {
    assert_eq!(
        ADMIN_PUBKEY,
        ctx.accounts.admin.key.to_string().as_bytes()
    );

    //return back nft
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.nft_lock_account.to_account_info().clone(),
                to: ctx.accounts.nft_user_account.to_account_info().clone(),
                authority: ctx.accounts.nft_lock_account.to_account_info().clone(),
            },
            &[&[
                ctx.accounts.pool_account.key().as_ref(),
                b"nft_lock_account".as_ref(),
                ctx.accounts.nft_lock_account.mint.as_ref(),
                &[*ctx.bumps.get("nft_lock_account").unwrap()],
            ]],
        ),
        ctx.accounts.nft_lock_account.amount,
    )?;


    //close the account we lock nft
    anchor_spl::token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info().clone(),
        anchor_spl::token::CloseAccount {
            account: ctx.accounts.nft_lock_account.to_account_info().clone(),
            destination: ctx.accounts.participant.to_account_info().clone(),
            authority: ctx.accounts.nft_lock_account.to_account_info().clone(),
        },
        &[&[
            ctx.accounts.pool_account.key().as_ref(),
            b"nft_lock_account".as_ref(),
            ctx.accounts.nft_lock_account.mint.as_ref(),
            &[*ctx.bumps.get("nft_lock_account").unwrap()],
        ]],
    ))?;


    ctx.accounts.whitelist_account.locked_nft_mint = Pubkey::default();
    Ok(())
}


#[derive(Accounts)]
pub struct RecoverNftContext<'info> {
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
    seeds = [pool_account.key().as_ref(), b"nft_lock_account".as_ref(), nft_mint.key().as_ref()],
    bump
    )]
    pub nft_lock_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = nft_user_account.mint == nft_mint.key())]
    pub nft_user_account: Account<'info, TokenAccount>,

    // #[account(mut, constraint = nft_user_account.mint == nft_mint.key())]
    // pub nft_user_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}