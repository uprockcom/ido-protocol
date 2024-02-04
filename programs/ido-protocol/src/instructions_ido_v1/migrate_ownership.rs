use anchor_lang::AccountsClose;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{IdoPoolAccount, TICKET_MINT, WhitelistAccount};

pub fn migrate_ownership(ctx: Context<MigrateOwnershipContext>) -> Result<()> {
    //first create new account
    ctx.accounts.whitelist_account_to.total_deposit =
        ctx.accounts.whitelist_account_from.total_deposit;
    ctx.accounts.whitelist_account_to.total_claim =
        ctx.accounts.whitelist_account_from.total_claim;
    ctx.accounts.whitelist_account_to.settled_allocation =
        ctx.accounts.whitelist_account_from.settled_allocation;
    ctx.accounts.whitelist_account_to.locked_ticket_amount =
        ctx.accounts.whitelist_account_from.locked_ticket_amount;
    ctx.accounts.whitelist_account_to.locked_nft_mint =
        ctx.accounts.whitelist_account_from.locked_nft_mint;

    ctx.accounts.whitelist_account_to.owner = ctx.accounts.participant_to.key();

    //transfer ticket
    //transfer locked tickets back to the user (decrease before we remove)
    {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.ticket_lock_account_from.to_account_info(),
                    to: ctx.accounts.ticket_lock_account_to.to_account_info(),
                    authority: ctx.accounts.ticket_lock_account_from.to_account_info(),
                },
                &[&[
                    ctx.accounts.pool_account.key().as_ref(),
                    b"ticket_lock_account".as_ref(),
                    ctx.accounts.participant_from.key().as_ref(),
                    &[*ctx.bumps.get("ticket_lock_account_from").unwrap()],
                ]],
            ),
            ctx.accounts.ticket_lock_account_from.amount,
        )?;
    }

    //close the ticket lock account (PDA)
    {
        anchor_spl::token::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::CloseAccount {
                account: ctx
                    .accounts
                    .ticket_lock_account_from
                    .to_account_info()
                    .clone(),
                destination: ctx.accounts.participant_from.to_account_info().clone(),
                authority: ctx
                    .accounts
                    .ticket_lock_account_from
                    .to_account_info()
                    .clone(),
            },
            &[&[
                ctx.accounts.pool_account.key().as_ref(),
                b"ticket_lock_account".as_ref(),
                ctx.accounts.participant_from.key().as_ref(),
                &[*ctx.bumps.get("ticket_lock_account_from").unwrap()],
            ]],
        ))?;
    }

    ctx.accounts
        .whitelist_account_from
        .close(ctx.accounts.participant_from.to_account_info())
}


#[derive(Accounts)]
pub struct MigrateOwnershipContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(
    mut,
    seeds = [pool_account.key().as_ref(), b"whitelist_account".as_ref(), participant_from.key().as_ref()],
    bump,
    )]
    pub whitelist_account_from: Box<Account<'info, WhitelistAccount>>,

    #[account(
    init_if_needed,
    payer = authority,
    space = WhitelistAccount::needed_space(),
    seeds = [pool_account.key().as_ref(), b"whitelist_account".as_ref(), participant_to.key().as_ref()],
    bump,
    )]
    pub whitelist_account_to: Box<Account<'info, WhitelistAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: authority is checked, no worries
    #[account(mut)]
    pub participant_from: AccountInfo<'info>,

    #[account(mut,
    seeds = [pool_account.key().as_ref(), b"ticket_lock_account".as_ref(), participant_from.key().as_ref()],
    bump)]
    pub ticket_lock_account_from: Box<Account<'info, TokenAccount>>,

    /// CHECK: authority is checked, no worries
    #[account(mut)]
    pub participant_to: AccountInfo<'info>,

    #[account(
    init_if_needed,
    payer = authority,
    seeds = [pool_account.key().as_ref(), b"ticket_lock_account".as_ref(), participant_to.key().as_ref()],
    token::mint = ticket_mint,
    token::authority = ticket_lock_account_to,
    bump
    )]
    pub ticket_lock_account_to: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = ticket_mint.key().to_string().as_bytes() == TICKET_MINT)]
    pub ticket_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}