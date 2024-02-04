use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{ IdoPoolAccount, WhitelistAccount, TICKET_MINT, ADMIN_PUBKEY};

pub fn recover_ticket(ctx: Context<RecoverTicketContext>) -> Result<()> {

    assert_eq!(
        ADMIN_PUBKEY,
        ctx.accounts.admin.key.to_string().as_bytes()
    );


    //return back Ticket if locked any
    if ctx.accounts.ticket_lock_account.amount > 0 {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.ticket_lock_account.to_account_info(),
                    to: ctx.accounts.ticket_user_account.to_account_info(),
                    authority: ctx.accounts.ticket_lock_account.to_account_info(),
                },
                &[&[
                    ctx.accounts.pool_account.key().as_ref(),
                    b"ticket_lock_account".as_ref(),
                    ctx.accounts.participant.key().as_ref(),
                    &[*ctx.bumps.get("ticket_lock_account").unwrap()],
                ]],
            ),
            ctx.accounts.ticket_lock_account.amount,
        )?;

        anchor_spl::token::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::CloseAccount {
                account: ctx.accounts.ticket_lock_account.to_account_info().clone(),
                destination: ctx.accounts.participant.to_account_info().clone(),
                authority: ctx.accounts.ticket_lock_account.to_account_info().clone(),
            },
            &[&[
                ctx.accounts.pool_account.key().as_ref(),
                b"ticket_lock_account".as_ref(),
                ctx.accounts.participant.key().as_ref(),
                &[*ctx.bumps.get("ticket_lock_account").unwrap()],
            ]],
        ))?;
    }

    //life saver
    ctx.accounts.whitelist_account.locked_ticket_amount = 0;
    Ok(())
}


#[derive(Accounts)]
pub struct RecoverTicketContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut, constraint = whitelist_account.pool == pool_account.key())]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

    /// CHECK: no worries `admin` is admin.
    #[account(mut, constraint = whitelist_account.owner == participant.key())]
    pub participant: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut,
    seeds = [pool_account.key().as_ref(), b"ticket_lock_account".as_ref(), participant.key().as_ref()],
    bump)]
    pub ticket_lock_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = ticket_user_account.mint == ticket_mint.key())]
    pub ticket_user_account: Account<'info, TokenAccount>,

    #[account(mut, constraint = ticket_mint.key().to_string().as_bytes() == TICKET_MINT)]
    pub ticket_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
