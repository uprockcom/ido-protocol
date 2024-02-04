use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{Err, IdoPoolAccount, WhitelistAccount, TICKET_MINT};

pub fn whitelist_ticket(ctx: Context<WhitelistTicketContext>, amount: u64) -> Result<()> {
    ctx.accounts.whitelist_account.owner = ctx.accounts.participant.key();
    ctx.accounts.whitelist_account.pool = ctx.accounts.pool_account.key();

    if ctx.accounts.pool_account.wallet_min_ticket > amount {
        return err!(Err::ErrWrongAmountOfTicket);
    }

    if ctx.accounts.ticket_user_account.amount < amount {
        return err!(Err::ErrInsufficientTicket);
    }

    let remaining_ticket_amount = ctx.accounts.pool_account.wallet_max_ticket
        - ctx.accounts.whitelist_account.locked_ticket_amount;

    let sent_amount = if amount > remaining_ticket_amount {
        remaining_ticket_amount
    } else {
        amount
    };

    msg!("Your TICKET amount is {}", &sent_amount);

    //first time lock? ok, increase unqiue count here
    if ctx.accounts.whitelist_account.locked_ticket_amount == 0 {
        ctx.accounts
            .pool_account
            .pool_stats
            .ticket_whitelisted_unique = ctx
            .accounts
            .pool_account
            .pool_stats
            .ticket_whitelisted_unique
            .checked_add(1)
            .unwrap();
    }

    ctx.accounts.whitelist_account.locked_ticket_amount = ctx
        .accounts
        .whitelist_account
        .locked_ticket_amount
        .checked_add(sent_amount)
        .unwrap();

    if ctx.accounts.whitelist_account.locked_ticket_amount
        > ctx.accounts.pool_account.wallet_max_ticket
    {
        return err!(Err::ErrWrongAmountOfTicket);
    }

    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.ticket_user_account.to_account_info().clone(),
                to: ctx.accounts.ticket_lock_account.to_account_info().clone(),
                authority: ctx.accounts.participant.to_account_info().clone(),
            },
        ),
        sent_amount,
    )?;

    ctx.accounts.whitelist_account.settled_allocation =
        (ctx.accounts.whitelist_account.settled_allocation as u128)
            .checked_add(
                (ctx.accounts.pool_account.wallet_max_cap as u128)
                    .checked_mul(sent_amount as u128)
                    .unwrap()
                    .checked_div(ctx.accounts.pool_account.wallet_max_ticket as u128)
                    .unwrap()
                    .checked_mul(ctx.accounts.pool_account.ticket_max_allocation_rate as u128)
                    .unwrap()
                    .checked_div(100)
                    .unwrap(),
            )
            .unwrap() as u64;

    // if  ctx.accounts.whitelist_account.settled_allocation > ctx.accounts.pool_account.wallet_max_cap{
    //
    // }

    msg!(
        "Your Allocation is {}",
        &ctx.accounts.whitelist_account.settled_allocation
    );

    ctx.accounts.pool_account.pool_stats.ticket_whitelisted =
        (ctx.accounts.pool_account.pool_stats.ticket_whitelisted as u128)
            .checked_add(sent_amount as u128)
            .unwrap() as u64;

    Ok(())
}

#[derive(Accounts)]
pub struct WhitelistTicketContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut)]
    pub participant: Signer<'info>,

    #[account(
    init_if_needed,
    payer = participant,
    space = WhitelistAccount::needed_space(),
    seeds = [pool_account.key().as_ref(),b"whitelist_account".as_ref(),participant.key().as_ref()],
    bump,
    )]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

    #[account(
    init_if_needed,
    payer = participant,
    seeds = [pool_account.key().as_ref(), b"ticket_lock_account".as_ref(),participant.key().as_ref()],
    token::mint = ticket_mint,
    token::authority = ticket_lock_account,
    bump
    )]
    pub ticket_lock_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = ticket_user_account.mint == ticket_mint.key())]
    pub ticket_user_account: Account<'info, TokenAccount>,

    #[account(mut,constraint = ticket_mint.key().to_string().as_bytes() == TICKET_MINT)]
    pub ticket_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
