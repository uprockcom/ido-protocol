use crate::{Err, IdoPoolAccount, PoolDataInput, PoolStats, DISTRIBUTION_TYPE_VESTED};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn create_pool(ctx: Context<CreatePoolContext>, pool_data: PoolDataInput) -> Result<()> {
    if !pool_data.is_valid_distribution() {
        return err!(Err::InvalidIdoDistributionType);
    }

    if pool_data.distribution_type == DISTRIBUTION_TYPE_VESTED {
        let sum: u8 = pool_data.distribution_vesting.iter().sum();
        if sum != 100 {
            return err!(Err::InvalidVestingPeriods);
        }
    }

    //validate before we process
    pool_data.seq()?;

    ctx.accounts.pool_account.name = pool_data.name;
    ctx.accounts.pool_account.distribution_type = pool_data.distribution_type;
    ctx.accounts.pool_account.fundraiser = pool_data.fundraiser;
    ctx.accounts.pool_account.base_mint = ctx.accounts.base_mint.key();
    ctx.accounts.pool_account.quote_mint = ctx.accounts.quote_mint.key();

    //Phases
    ctx.accounts.pool_account.phase_whitelist = pool_data.phase_whitelist;
    ctx.accounts.pool_account.phase_sale_nft = pool_data.phase_sale_nft;
    ctx.accounts.pool_account.phase_sale_ticket = pool_data.phase_sale_ticket;
    ctx.accounts.pool_account.phase_distribution = pool_data.phase_distribution;

    ctx.accounts.pool_account.distribution_vesting_period = pool_data.distribution_vesting_period;
    ctx.accounts.pool_account.distribution_vesting = pool_data.distribution_vesting;
    //[25,10,10,10,10,10,10,10,5]
    ctx.accounts.pool_account.allowed_creators = pool_data.allowed_creators;

    ctx.accounts.pool_account.wallet_min_cap = pool_data.wallet_min_cap;
    ctx.accounts.pool_account.wallet_max_cap = pool_data.wallet_max_cap;

    ctx.accounts.pool_account.wallet_min_ticket = pool_data.wallet_min_ticket;
    ctx.accounts.pool_account.wallet_max_ticket = pool_data.wallet_max_ticket;

    ctx.accounts.pool_account.ticket_max_allocation_rate = pool_data.ticket_max_allocation_rate;

    ctx.accounts.pool_account.pool_hard_cap = pool_data.pool_hard_cap;
    ctx.accounts.pool_account.rate = pool_data.rate;
    ctx.accounts.pool_account.fair_distro_fee = pool_data.fair_distro_fee;

    //initial, stats
    ctx.accounts.pool_account.pool_stats = PoolStats {
        nft_whitelisted: 0,
        ticket_whitelisted: 0,
        ticket_whitelisted_unique: 0,
        total_contribution: 0,
        unique_contributor: 0,
        nft_total_contribution: 0,
        nft_unique_contributor: 0,
        ticket_total_contribution: 0,
        ticket_unique_contributor: 0,
    };
    ctx.accounts.pool_account.is_raised = false;

    Ok(())
}

#[derive(Accounts)]
#[instruction(pool_data: PoolDataInput)]
pub struct CreatePoolContext<'info> {
    #[account(
    init,
    payer = authority,
    space = IdoPoolAccount::needed_space(pool_data.allowed_creators.len(), pool_data.name),
    )]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(
    init_if_needed,
    payer = authority,
    seeds = [pool_account.key().as_ref(), b"base_token_account".as_ref(), base_mint.key().as_ref()],
    token::mint = base_mint,
    token::authority = base_token_account,
    bump
    )]
    pub base_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub base_mint: Account<'info, Mint>,

    #[account(
    init_if_needed,
    payer = authority,
    seeds = [pool_account.key().as_ref(), b"quote_token_account".as_ref(), quote_mint.key().as_ref()],
    token::mint = quote_mint,
    token::authority = quote_token_account,
    bump
    )]
    pub quote_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_mint: Account<'info, Mint>,

    //this must be signer.
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
