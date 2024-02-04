use anchor_lang::prelude::*;

use instructions_ido_v1::*;
use state::*;

// use errors::*;
use crate::errors::Err;

pub mod access_controls;
mod anchor_metaplex;
pub mod errors;
mod instructions_ido_v1;
pub mod state;

declare_id!("ido5zRdfphtyeov8grJqBszfWquvTCAvRzGycSfPXuX");
const ADMIN_PUBKEY: &[u8] = b"devKH8tdPeByT13FWMVcX48B43v3wB1Qsa2xfGvpttm";
const TICKET_MINT: &[u8] = b"TicpC2VhBZbknfc4RhYkdPty6SE4HjBozZS9umAMX1d";

#[program]
pub mod ido_protocol {
    use anchor_lang::AccountsClose;

    use crate::access_controls::is_admin;
    use crate::access_controls::is_in_distribution;
    use crate::access_controls::is_in_the_phase;

    use super::*;

    pub fn debug(ctx: Context<DebugContext>) -> Result<()> {
        //only admin can debug
        assert_eq!(
            ADMIN_PUBKEY,
            ctx.accounts.authority.key.to_string().as_bytes()
        );
        // msg!(
        //     IdoPoolAccount::needed_space(5, "hello  00000000000000".try_to_vec().unwrap())
        //         .to_string()
        //         .as_ref()
        // );
        Ok(())
    }

    //admin only, create a pool with start-end date, min-max etc.
    //and some stats.
    //what is fee here?

    #[access_control(is_admin(& ctx.accounts.authority))]
    pub fn create_pool(ctx: Context<CreatePoolContext>, pool_data: PoolDataInput) -> Result<()> {
        instructions_ido_v1::create_pool(ctx, pool_data)
    }

    #[access_control(is_admin(& ctx.accounts.authority))]
    pub fn update_pool(ctx: Context<UpdatePoolContext>, pool_data: PoolDataInput) -> Result<()> {
        instructions_ido_v1::update_pool(ctx, pool_data)
    }

    //register a wallet by locking their nft
    #[access_control(is_in_the_phase(& ctx.accounts.pool_account.phase_whitelist))]
    pub fn whitelist_nft(ctx: Context<WhitelistNftContext>) -> Result<()> {
        instructions_ido_v1::whitelist_nft(ctx)
    }

    //register a wallet by locking their TICKET Tokens
    #[access_control(is_in_the_phase(& ctx.accounts.pool_account.phase_whitelist))]
    pub fn whitelist_ticket(ctx: Context<WhitelistTicketContext>, amount: u64) -> Result<()> {
        instructions_ido_v1::whitelist_ticket(ctx, amount)
    }

    //get user money according to their allocation
    #[access_control(is_in_the_phase(& ctx.accounts.pool_account.phase_sale_nft))]
    pub fn deposit_nft(ctx: Context<DepositNftContext>, amount: u64) -> Result<()> {
        instructions_ido_v1::deposit_nft(ctx, amount)
    }

    #[access_control(is_in_the_phase(& ctx.accounts.pool_account.phase_sale_ticket))]
    pub fn deposit_ticket(ctx: Context<DepositTicketContext>, amount: u64) -> Result<()> {
        instructions_ido_v1::deposit_ticket(ctx, amount)
    }

    pub fn boost(ctx: Context<BoostContext>) -> Result<()> {
        if ctx.accounts.ticket_lock_account.amount == 0 {
            return Ok(());
        }

        anchor_spl::token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Burn {
                    mint: ctx.accounts.ticket_mint.to_account_info(),
                    from: ctx.accounts.ticket_lock_account.to_account_info(),
                    authority: ctx.accounts.ticket_lock_account.to_account_info(),
                },
                &[&[
                    ctx.accounts.pool_account.key().as_ref(),
                    b"ticket_lock_account".as_ref(),
                    ctx.accounts.participant.key().as_ref(),
                    &[*ctx.bumps.get("ticket_lock_account").unwrap()],
                ]],
            ),
            ctx.accounts
                .ticket_lock_account
                .amount
                .checked_div(4)
                .unwrap(),
        )?;

        ctx.accounts.whitelist_account.locked_ticket_amount = ctx
            .accounts
            .whitelist_account
            .locked_ticket_amount
            .checked_div(2)
            .unwrap();

        ctx.accounts.whitelist_account.settled_allocation = ctx
            .accounts
            .whitelist_account
            .settled_allocation
            .checked_mul(2)
            .unwrap();

        Ok(())
    }

    //get user money according to their allocation
    #[access_control(is_in_the_phase(& ctx.accounts.pool_account.phase_distribution))]
    pub fn unlock_nft(ctx: Context<UnlockNftContext>) -> Result<()> {
        //return back NFT
        if ctx.accounts.nft_lock_account.amount == 1 {
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
                1,
            )?;


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
        }

        //ctx.accounts.whitelist_account.locked_nft_mint = Pubkey::default();

        Ok(())
    }

    //get user money according to their allocation
    #[access_control(is_in_the_phase(& ctx.accounts.pool_account.phase_distribution))]
    pub fn unlock_ticket(ctx: Context<UnlockTicketContext>) -> Result<()> {
        instructions_ido_v1::unlock_ticket(ctx)
    }

    //TODO: decide fee here
    //#[access_control(is_phase_over(& ctx.accounts.pool_account.distribution))]
    pub fn raise(ctx: Context<RaiseContext>) -> Result<()> {
        msg!(
            "total raised (stats) {}",
            ctx.accounts.pool_account.pool_stats.total_contribution
        );

        msg!(
            "total in account (transferred) {}",
            ctx.accounts.quote_token_account.amount
        );

        let total_base_needed = ctx
            .accounts
            .quote_token_account
            .amount
            .checked_div(ctx.accounts.pool_account.rate.base)
            .unwrap()
            .checked_mul(ctx.accounts.pool_account.rate.quote)
            .unwrap();

        msg!("total base token needed {}", total_base_needed);

        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.base_token_user_account.to_account_info(),
                    to: ctx.accounts.base_token_account.to_account_info(),
                    authority: ctx.accounts.fundraiser.to_account_info(),
                },
            ),
            total_base_needed,
        )?;

        // msg!(
        //     "total base token needed (stats) {}",
        //     ctx.accounts
        //         .pool_account
        //         .pool_stats
        //         .total_contribution
        //         .checked_mul(ctx.accounts.pool_account.rate.base)
        //         .unwrap()
        //         .checked_mul(ctx.accounts.pool_account.rate.quote)
        //         .unwrap()
        // );

        ctx.accounts.pool_account.is_raised = true;

        Ok(())
    }

    // any fee here?
    // implement a fair distro fee
    // if contribution is less then what we allocated
    // they should pay fair distro fee.
    // its math will be implemented later.
    #[access_control(is_in_distribution(& ctx.accounts.pool_account))]
    pub fn claim(ctx: Context<ClaimContext>) -> Result<()> {
        //check if deposited
        // if !ctx.accounts.pool_account.is_raised {
        //     return err!(Err::ErrFundHasNotBeenRaisedYet);
        // }

        if ctx.accounts.whitelist_account.total_deposit == 0 {
            msg!("no deposit");
            return Ok(());
        }

        msg!(
            "distro type is {}",
            ctx.accounts.pool_account.distribution_type
        );

        if ctx.accounts.pool_account.distribution_type == DISTRIBUTION_TYPE_DROP {
            return err!(Err::ErrTokensWillDrop);
        }

        if ctx.accounts.pool_account.distribution_type == DISTRIBUTION_TYPE_STANDARD {
            //just allow claiming
            //get token from base_token_account
            //calculate from pool_account.rate
            //give some back.
            return Ok(());
        }

        if ctx.accounts.pool_account.distribution_type == DISTRIBUTION_TYPE_VESTED {
            let time = Clock::get()?.unix_timestamp as u64;

            let mut passed = 0;

            if time > ctx.accounts.pool_account.phase_distribution.starts_at {
                passed = time
                    .checked_sub(ctx.accounts.pool_account.phase_distribution.starts_at)
                    .unwrap();
            }

            let passed_period_count = passed
                .checked_div(ctx.accounts.pool_account.distribution_vesting_period)
                .unwrap();

            let base_token_in_total = (ctx.accounts.whitelist_account.total_deposit as u128)
                .checked_mul(ctx.accounts.pool_account.rate.base as u128)
                .unwrap()
                .checked_div(ctx.accounts.pool_account.rate.quote as u128)
                .unwrap() as u64;

            let mut total_reclaim: u64 = 0;

            msg!("base_token_in_total: {}", base_token_in_total);
            msg!("passed periods: {}", passed_period_count);
            for i in 0..ctx.accounts.pool_account.distribution_vesting.len() {
                let transfer_in_period = (base_token_in_total as u128)
                    .checked_mul(
                        *ctx.accounts
                            .pool_account
                            .distribution_vesting
                            .get(i)
                            .unwrap() as u128,
                    )
                    .unwrap()
                    .checked_div(100)
                    .unwrap();

                msg!(
                    "period {},{}%,{}",
                    i,
                    ctx.accounts
                        .pool_account
                        .distribution_vesting
                        .get(i)
                        .unwrap(),
                    transfer_in_period
                );

                if passed_period_count >= i as u64 {
                    total_reclaim = (total_reclaim as u128)
                        .checked_add(transfer_in_period as u128)
                        .unwrap() as u64
                }
            }

            msg!("total reclaim :{}", total_reclaim);
            msg!(
                "reclaim done :{}",
                ctx.accounts.whitelist_account.total_claim
            );

            let remaining = total_reclaim
                .checked_sub(ctx.accounts.whitelist_account.total_claim)
                .unwrap();

            msg!("remaining: {}", remaining);

            if remaining == 0 {
                return Ok(());
            }

            //return the remaining here
            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info().clone(),
                    anchor_spl::token::Transfer {
                        from: ctx.accounts.base_token_account.to_account_info().clone(),
                        to: ctx
                            .accounts
                            .base_token_user_account
                            .to_account_info()
                            .clone(),
                        authority: ctx.accounts.base_token_account.to_account_info().clone(),
                    },
                    &[&[
                        ctx.accounts.pool_account.key().as_ref(),
                        b"base_token_account".as_ref(),
                        ctx.accounts.pool_account.base_mint.as_ref(),
                        &[*ctx.bumps.get("base_token_account").unwrap()],
                    ]],
                ),
                remaining,
            )?;

            ctx.accounts.whitelist_account.total_claim = ctx
                .accounts
                .whitelist_account
                .total_claim
                .checked_add(remaining)
                .unwrap();

            return Ok(());
        }

        // is there vesting?
        // what is the period?
        // when distribution start?
        // how many period has been passed?
        // how many they can claim
        // how many the claimed already?
        // done with claiming or?

        Ok(())
    }

    #[access_control(is_admin(& ctx.accounts.authority))]
    pub fn force_claim(ctx: Context<ForceClaimContext>) -> Result<()> {
        assert_eq!(
            ADMIN_PUBKEY,
            ctx.accounts.authority.key.to_string().as_bytes()
        );

        if ctx.accounts.whitelist_account.total_deposit == 0 {
            msg!("no deposit");
            return Ok(());
        }

        msg!(
            "distro type is {}",
            ctx.accounts.pool_account.distribution_type
        );

        if ctx.accounts.pool_account.distribution_type == DISTRIBUTION_TYPE_DROP {
            return err!(Err::ErrTokensWillDrop);
        }

        if ctx.accounts.pool_account.distribution_type == DISTRIBUTION_TYPE_STANDARD {
            //just allow claiming
            //get token from base_token_account
            //calculate from pool_account.rate
            //give some back.
            return Ok(());
        }

        if ctx.accounts.pool_account.distribution_type == DISTRIBUTION_TYPE_VESTED {
            let time = Clock::get()?.unix_timestamp as u64;

            let mut passed = 0;

            if time > ctx.accounts.pool_account.phase_distribution.starts_at {
                passed = time
                    .checked_sub(ctx.accounts.pool_account.phase_distribution.starts_at)
                    .unwrap();
            }

            let passed_period_count = passed
                .checked_div(ctx.accounts.pool_account.distribution_vesting_period)
                .unwrap();

            let base_token_in_total = (ctx.accounts.whitelist_account.total_deposit as u128)
                .checked_mul(ctx.accounts.pool_account.rate.base as u128)
                .unwrap()
                .checked_div(ctx.accounts.pool_account.rate.quote as u128)
                .unwrap() as u64;

            let mut total_reclaim: u64 = 0;

            msg!("base_token_in_total: {}", base_token_in_total);
            msg!("passed periods: {}", passed_period_count);
            for i in 0..ctx.accounts.pool_account.distribution_vesting.len() {
                let transfer_in_period = (base_token_in_total as u128)
                    .checked_mul(
                        *ctx.accounts
                            .pool_account
                            .distribution_vesting
                            .get(i)
                            .unwrap() as u128,
                    )
                    .unwrap()
                    .checked_div(100)
                    .unwrap();

                msg!(
                    "period {},{}%,{}",
                    i,
                    ctx.accounts
                        .pool_account
                        .distribution_vesting
                        .get(i)
                        .unwrap(),
                    transfer_in_period
                );

                if passed_period_count >= i as u64 {
                    total_reclaim = (total_reclaim as u128)
                        .checked_add(transfer_in_period as u128)
                        .unwrap() as u64
                }
            }

            msg!("total reclaim :{}", total_reclaim);
            msg!(
                "reclaim done :{}",
                ctx.accounts.whitelist_account.total_claim
            );

            let remaining = total_reclaim
                .checked_sub(ctx.accounts.whitelist_account.total_claim)
                .unwrap();

            msg!("remaining: {}", remaining);

            if remaining == 0 {
                return Ok(());
            }

            //return the remaining here
            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info().clone(),
                    anchor_spl::token::Transfer {
                        from: ctx.accounts.base_token_account.to_account_info().clone(),
                        to: ctx
                            .accounts
                            .base_token_user_account
                            .to_account_info()
                            .clone(),
                        authority: ctx.accounts.base_token_account.to_account_info().clone(),
                    },
                    &[&[
                        ctx.accounts.pool_account.key().as_ref(),
                        b"base_token_account".as_ref(),
                        ctx.accounts.pool_account.base_mint.as_ref(),
                        &[*ctx.bumps.get("base_token_account").unwrap()],
                    ]],
                ),
                remaining,
            )?;

            ctx.accounts.whitelist_account.total_claim = ctx
                .accounts
                .whitelist_account
                .total_claim
                .checked_add(remaining)
                .unwrap();

            return Ok(());
        }

        Ok(())
    }

    //refund if the raise date due
    pub fn refund(ctx: Context<RefundContext>) -> Result<()> {
        //check if they deposited
        //refund date is came
        //raise has not been happened
        //assert!(!ctx.accounts.pool_account.is_raised);
        if ctx.accounts.pool_account.is_raised {
            return err!(Err::ErrFundHasNotBeenRaisedYet);
        }

        Ok(())
    }

    #[access_control(is_admin(& ctx.accounts.authority))]
    pub fn close_pool(ctx: Context<ClosePoolContext>) -> Result<()> {
        ctx.accounts
            .pool_account
            .close(ctx.accounts.authority.to_account_info())
    }

    #[access_control(is_admin(& ctx.accounts.authority))]
    pub fn close_whitelist_account(ctx: Context<CloseWhitelistAccountContext>) -> Result<()> {
        assert_eq!(
            ADMIN_PUBKEY,
            ctx.accounts.authority.key.to_string().as_bytes()
        );

        //update pool stats
        {
            //total unique count for ticket
            ctx.accounts
                .pool_account
                .pool_stats
                .ticket_whitelisted_unique = ctx
                .accounts
                .pool_account
                .pool_stats
                .ticket_whitelisted_unique
                .checked_sub(1)
                .unwrap();

            //total unique count for nft
            ctx.accounts.pool_account.pool_stats.nft_whitelisted = ctx
                .accounts
                .pool_account
                .pool_stats
                .nft_whitelisted
                .checked_sub(1)
                .unwrap();

            //the contribution in usd.
            ctx.accounts.pool_account.pool_stats.total_contribution =
                (ctx.accounts.pool_account.pool_stats.total_contribution)
                    .checked_sub(ctx.accounts.whitelist_account.total_deposit)
                    .unwrap();

            if ctx.accounts.ticket_lock_account.amount > 0 {
                if ctx.accounts.pool_account.pool_stats.ticket_whitelisted
                    > ctx.accounts.ticket_lock_account.amount
                {
                    ctx.accounts.pool_account.pool_stats.ticket_whitelisted = ctx
                        .accounts
                        .pool_account
                        .pool_stats
                        .ticket_whitelisted
                        .checked_sub(ctx.accounts.ticket_lock_account.amount)
                        .unwrap();
                }
            }
        }

        //transfer locked tickets back to the user (decrease before we remove)
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
        }

        //close the ticket lock account (PDA)
        {
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

        //return back NFT
        if ctx.accounts.nft_lock_account.amount == 1 {
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
                1,
            )?;

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
        }

        //return back the quote token
        if ctx.accounts.whitelist_account.total_deposit <= ctx.accounts.quote_token_account.amount {
            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        from: ctx.accounts.quote_token_account.to_account_info(),
                        to: ctx.accounts.quote_user_account.to_account_info(),
                        authority: ctx.accounts.quote_token_account.to_account_info(),
                    },
                    &[&[
                        ctx.accounts.pool_account.key().as_ref(),
                        b"quote_token_account".as_ref(),
                        ctx.accounts.quote_mint.key().as_ref(),
                        &[*ctx.bumps.get("quote_token_account").unwrap()],
                    ]],
                ),
                ctx.accounts.whitelist_account.total_deposit,
            )?;
        }

        //wipe the memory we allocate
        ctx.accounts
            .whitelist_account
            .close(ctx.accounts.authority.to_account_info())?;

        Ok(())
    }

    #[access_control(is_admin(& ctx.accounts.admin))]
    pub fn recover_nft(ctx: Context<RecoverNftContext>) -> Result<()> {
        instructions_ido_v1::recover_nft(ctx)
    }

    #[access_control(is_admin(& ctx.accounts.admin))]
    pub fn recover_ticket(ctx: Context<RecoverTicketContext>) -> Result<()> {
        instructions_ido_v1::recover_ticket(ctx)
    }

    #[access_control(is_admin(& ctx.accounts.admin))]
    pub fn recover_usdc(ctx: Context<RecoverUsdcContext>) -> Result<()> {
        instructions_ido_v1::recover_usdc(ctx)
    }


    #[access_control(is_admin(& ctx.accounts.authority))]
    pub fn migrate_ownership(ctx: Context<MigrateOwnershipContext>) -> Result<()> {
        instructions_ido_v1::migrate_ownership(ctx)
    }

    #[access_control(is_admin(& ctx.accounts.authority))]
    pub fn just_close_whitelist_account(
        ctx: Context<JustCloseWhitelistAccountContext>,
    ) -> Result<()> {
        //wipe the memory we allocate
        ctx.accounts
            .whitelist_account
            .close(ctx.accounts.authority.to_account_info())?;

        Ok(())
    }

    #[access_control(is_admin(& ctx.accounts.authority))]
    pub fn update_pool_rate(ctx: Context<UpdatePoolRateContext>, rate: Rate) -> Result<()> {
        ctx.accounts.pool_account.rate = rate;
        Ok(())
    }

    #[access_control(is_admin(& ctx.accounts.authority))]
    pub fn force_raise(ctx: Context<ForceRaiseContext>) -> Result<()> {
        instructions_ido_v1::force_raise(ctx)
    }

    #[access_control(is_admin(& ctx.accounts.authority))]
    pub fn update_whitelist_account(
        ctx: Context<UpdateWhitelistAccountContext>,
        whitelist_account: WhitelistAccountInput,
    ) -> Result<()> {
        ctx.accounts.whitelist_account.owner = whitelist_account.owner;
        ctx.accounts.whitelist_account.pool = whitelist_account.pool;
        ctx.accounts.whitelist_account.locked_nft_mint = whitelist_account.locked_nft_mint;
        ctx.accounts.whitelist_account.locked_ticket_amount =
            whitelist_account.locked_ticket_amount;
        ctx.accounts.whitelist_account.settled_allocation = whitelist_account.settled_allocation;
        ctx.accounts.whitelist_account.total_deposit = whitelist_account.total_deposit;
        ctx.accounts.whitelist_account.total_claim = whitelist_account.total_claim;

        Ok(())
    }

    //TODO: implement updating whitelist account
}
