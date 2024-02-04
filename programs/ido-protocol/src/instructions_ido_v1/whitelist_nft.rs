use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::anchor_metaplex::{check_metadata, get_metadata_account};
use crate::{Err, IdoPoolAccount, WhitelistAccount};

pub fn whitelist_nft(ctx: Context<WhitelistNftContext>) -> Result<()> {
    // assert_eq!(
    //     ctx.accounts.whitelist_account.pool.key().as_ref(),
    //     ctx.accounts.pool_account.key().as_ref()
    // );

    if ctx.accounts.whitelist_account.locked_nft_mint != Pubkey::default() {
        return err!(Err::ErrNFTAlreadyLocked);
    }

    let metadata = get_metadata_account(ctx.remaining_accounts.clone())?;
    let checked_metadata = check_metadata(
        &metadata,
        &ctx.accounts.nft_mint.key(),
        ctx.accounts.pool_account.allowed_creators.clone(),
    );

    if checked_metadata.is_err() {
        return err!(Err::ErrInvalidNFT);
    }

    msg!(
        "your NFT allocation rate is {}%",
        checked_metadata.as_ref().unwrap().allocation
    );

    ctx.accounts.whitelist_account.owner = ctx.accounts.participant.key();
    ctx.accounts.whitelist_account.pool = ctx.accounts.pool_account.key();
    ctx.accounts.whitelist_account.locked_nft_mint = ctx.accounts.nft_mint.key();

    ctx.accounts.whitelist_account.settled_allocation = ctx
        .accounts
        .whitelist_account
        .settled_allocation
        .checked_add(
            (ctx.accounts.pool_account.wallet_max_cap as u128)
                .checked_div(100_u128)
                .unwrap()
                .checked_mul(checked_metadata.unwrap().allocation as u128)
                .unwrap() as u64,
        )
        .unwrap();

    msg!(
        "Your settled allocation is {}",
        ctx.accounts.whitelist_account.settled_allocation
    );

    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.nft_user_account.to_account_info(),
                to: ctx.accounts.nft_lock_account.to_account_info(),
                authority: ctx.accounts.participant.to_account_info(),
            },
        ),
        1,
    )?;

    //update pool stats
    ctx.accounts.pool_account.pool_stats.nft_whitelisted += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct WhitelistNftContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(
    init_if_needed,
    payer = participant,
    seeds = [pool_account.key().as_ref(), b"nft_lock_account".as_ref(), nft_mint.key().as_ref()],
    token::mint = nft_mint,
    token::authority = nft_lock_account,
    bump
    )]
    pub nft_lock_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = nft_user_account.mint == nft_mint.key())]
    pub nft_user_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,

    #[account(
    init_if_needed,
    payer = participant,
    space = WhitelistAccount::needed_space(),
    seeds = [pool_account.key().as_ref(), b"whitelist_account".as_ref(), participant.key().as_ref()],
    bump,
    )]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

    #[account(mut)]
    pub participant: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
