use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use metaplex_token_metadata::state::Creator;

use errors::Err;

use crate::{errors, TICKET_MINT};

#[derive(Accounts)]
pub struct DebugContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct IdoPoolAccount {
    pub name: Vec<u8>,
    //keep that shit short please
    pub distribution_type: u8,
    pub is_raised: bool,
    pub fundraiser: Pubkey,
    //(Allowed Pubkey to raise)
    pub phase_whitelist: StartsEnds,
    // whitelist() opens at
    pub phase_sale_nft: StartsEnds,
    // participate_nft() closes at
    pub phase_sale_ticket: StartsEnds,
    // participate_ticket() opens at
    pub phase_distribution: StartsEnds,
    // claim() closes at
    pub distribution_vesting_period: u64,
    pub distribution_vesting: Vec<u8>,
    // period in second.
    pub wallet_min_cap: u64,
    pub wallet_max_cap: u64,
    pub wallet_min_ticket: u64,         //100k?
    pub wallet_max_ticket: u64,         //500k?
    pub ticket_max_allocation_rate: u8, // 0- 100
    pub allowed_creators: Vec<CreatorStruct>,
    pub pool_hard_cap: u64,
    pub pool_stats: PoolStats,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub rate: Rate,
    pub fair_distro_fee: u8,
}

impl IdoPoolAccount {
    pub fn needed_space(creator_size: usize, name: Vec<u8>) -> usize {
        8  //account discriminator
            + std::mem::size_of::<IdoPoolAccount>()
            + std::mem::size_of::<Vec<u8>>() * name.len()
            + std::mem::size_of::<Vec<CreatorStruct>>() * creator_size // allowed creators
         + 250 // 250 more bytes for the future? should be enough, I think
    }
}

#[derive(Accounts)]
pub struct ClosePoolContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct JustCloseWhitelistAccountContext<'info> {
    #[account(mut)]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: authority is strictly checked, no worries
    #[account(mut)]
    pub participant: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CloseWhitelistAccountContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut)]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

    #[account(
    mut,
    seeds = [pool_account.key().as_ref(), b"quote_token_account".as_ref(), quote_mint.key().as_ref()],
    bump
    )]
    pub quote_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub quote_user_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

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
    pub nft_user_account: Box<Account<'info, TokenAccount>>,

    // #[account(mut, constraint = nft_user_account.mint == nft_mint.key())]
    // pub nft_user_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = nft_mint.key().as_ref() == whitelist_account.locked_nft_mint.as_ref())]
    pub nft_mint: Account<'info, Mint>,

    #[account(mut,
    seeds = [pool_account.key().as_ref(), b"ticket_lock_account".as_ref(),participant.key().as_ref()],
    bump)]
    pub ticket_lock_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = ticket_user_account.mint == ticket_mint.key())]
    pub ticket_user_account: Account<'info, TokenAccount>,

    #[account(mut,constraint = ticket_mint.key().to_string().as_bytes() == TICKET_MINT)]
    pub ticket_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}



#[derive(Accounts)]
pub struct UpdateWhitelistAccountContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(
    init_if_needed,
    payer = authority,
    space = WhitelistAccount::needed_space(),
    seeds = [pool_account.key().as_ref(),b"whitelist_account".as_ref(),participant.key().as_ref()],
    bump,
    )]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: authority is checked, no worries
    #[account(mut)]
    pub participant: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize, Default, Clone)]
pub struct CreatorStruct {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
    pub allocation: u8, // 0 - 100
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize, Default, Clone)]
pub struct StartsEnds {
    pub starts_at: u64,
    pub ends_at: u64,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize, Default, Clone)]
pub struct PoolStats {
    pub nft_whitelisted: u64,
    pub ticket_whitelisted: u64,
    pub ticket_whitelisted_unique: u64,

    pub total_contribution: u64,
    pub unique_contributor: u64,

    pub nft_total_contribution: u64,
    pub nft_unique_contributor: u64,

    pub ticket_total_contribution: u64,
    pub ticket_unique_contributor: u64,
    // todo: add ticket_total_allocation
    // add nft_total_allocation
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize, Default, Clone)]
pub struct PoolDataInput {
    pub name: Vec<u8>,
    pub fundraiser: Pubkey,
    pub distribution_type: u8,
    pub phase_whitelist: StartsEnds,
    // whitelist() opens at
    pub phase_sale_nft: StartsEnds,
    // participate_nft() closes at
    pub phase_sale_ticket: StartsEnds,
    // participate_ticket() opens at
    pub phase_distribution: StartsEnds,
    pub distribution_vesting_period: u64, //every month
    pub distribution_vesting: Vec<u8>,    //25,10,10,10,10,10,10,10,5 for secretum for example
    pub wallet_min_cap: u64,
    pub wallet_max_cap: u64,
    pub wallet_min_ticket: u64,
    pub wallet_max_ticket: u64,
    pub ticket_max_allocation_rate: u8,
    pub pool_hard_cap: u64,
    pub rate: Rate,
    pub allowed_creators: Vec<CreatorStruct>,
    pub fair_distro_fee: u8,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize, Default, Clone)]
pub struct Rate {
    pub base: u64,
    pub quote: u64,
}

pub const DISTRIBUTION_TYPE_VESTED: u8 = 0;
pub const DISTRIBUTION_TYPE_STANDARD: u8 = 1;
pub const DISTRIBUTION_TYPE_DROP: u8 = 2;

impl PoolDataInput {
    pub fn is_valid_distribution(&self) -> bool {
        self.distribution_type == DISTRIBUTION_TYPE_VESTED
            || self.distribution_type == DISTRIBUTION_TYPE_STANDARD
            || self.distribution_type == DISTRIBUTION_TYPE_DROP
    }
}

impl PartialEq<Creator> for &CreatorStruct {
    fn eq(&self, other: &Creator) -> bool {
        self.address == other.address
            && self.verified == other.verified
            && self.share == other.share
    }
}

impl StartsEnds {
    pub fn seq(&self) -> Result<()> {
        if self.ends_at <= self.starts_at {
            return err!(Err::ErrIdoSeq);
        }
        Ok(())
    }
}

impl PoolDataInput {
    pub fn seq(&self) -> Result<()> {
        self.phase_whitelist.seq()?;
        self.phase_sale_nft.seq()?;
        self.phase_sale_ticket.seq()?;
        self.phase_distribution.seq()?;

        //sale for nft should start after whitelist
        if self.phase_sale_nft.starts_at <= self.phase_whitelist.ends_at {
            return err!(Err::ErrIdoSeq);
        }

        //sale for ticket should start after sale for nft
        if self.phase_sale_ticket.starts_at <= self.phase_sale_nft.ends_at {
            return err!(Err::ErrIdoSeq);
        }

        //distribution should start after ticket sale nft
        if self.phase_distribution.starts_at <= self.phase_sale_ticket.ends_at {
            return err!(Err::ErrIdoSeq);
        }

        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct WhitelistAccount {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub locked_nft_mint: Pubkey,
    pub locked_ticket_amount: u64,
    pub settled_allocation: u64,
    pub total_deposit: u64,
    pub total_claim: u64,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize, Default, Clone)]
pub struct WhitelistAccountInput {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub locked_nft_mint: Pubkey,
    pub locked_ticket_amount: u64,
    pub settled_allocation: u64,
    pub total_deposit: u64,
    pub total_claim: u64,
}

impl WhitelistAccount {
    pub fn needed_space() -> usize {
        //first 8 is anchor account discriminator
        //size of the whitelistaccount should be enough for us
        //we doubleit - just in case - we need more fields in future.
        (8 + std::mem::size_of::<WhitelistAccount>()) * 2
    }
}

#[derive(Accounts)]
pub struct BoostContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut,
    constraint = whitelist_account.pool == pool_account.key(),
    constraint = whitelist_account.owner == participant.key(),
    )]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

    #[account(mut,
    seeds = [pool_account.key().as_ref(), b"ticket_lock_account".as_ref(),participant.key().as_ref()],
    bump)]
    pub ticket_lock_account: Box<Account<'info, TokenAccount>>,

    #[account(mut,constraint = ticket_mint.key().to_string().as_bytes() == TICKET_MINT)]
    pub ticket_mint: Account<'info, Mint>,

    #[account(mut)]
    pub participant: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RaiseContext<'info> {
    #[account(mut,
    constraint = base_token_account.mint == pool_account.base_mint,
    constraint = fundraiser.key() == pool_account.fundraiser,
    constraint = quote_token_user_account.mint == pool_account.quote_mint,
    constraint = quote_token_account.mint == pool_account.quote_mint,
    constraint = base_token_user_account.mint == pool_account.base_mint
    )]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut,
    seeds = [pool_account.key().as_ref(), b"base_token_account".as_ref(), pool_account.base_mint.as_ref()],
    bump
    )]
    pub base_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub base_token_user_account: Account<'info, TokenAccount>,

    #[account(mut,
    seeds = [pool_account.key().as_ref(), b"quote_token_account".as_ref(), pool_account.quote_mint.as_ref()],
    bump
    )]
    pub quote_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_token_user_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub fundraiser: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UnlockNftContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut, constraint = whitelist_account.pool == pool_account.key())]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

    #[account(mut, constraint = whitelist_account.owner == participant.key())]
    pub participant: Signer<'info>,

    #[account(
    mut,
    seeds = [pool_account.key().as_ref(), b"nft_lock_account".as_ref(), nft_mint.key().as_ref()],
    bump
    )]
    pub nft_lock_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = nft_user_account.mint == nft_mint.key())]
    pub nft_user_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = nft_mint.key().as_ref() == whitelist_account.locked_nft_mint.as_ref())]
    pub nft_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UpdatePoolRateContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimContext<'info> {
    #[account(mut,
    constraint = quote_token_account.mint == pool_account.quote_mint,
    constraint = base_token_user_account.mint == pool_account.base_mint,
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
    seeds = [pool_account.key().as_ref(), b"quote_token_account".as_ref(), quote_mint.key().as_ref()],
    bump
    )]
    pub quote_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut,
    seeds = [pool_account.key().as_ref(), b"base_token_account".as_ref(), pool_account.base_mint.as_ref()],
    bump
    )]
    pub base_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub base_token_user_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_mint: Account<'info, Mint>,

    #[account(mut)]
    pub participant: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ForceClaimContext<'info> {
    #[account(mut,
    constraint = quote_token_account.mint == pool_account.quote_mint,
    constraint = base_token_user_account.mint == pool_account.base_mint,
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
    seeds = [pool_account.key().as_ref(), b"quote_token_account".as_ref(), quote_mint.key().as_ref()],
    bump
    )]
    pub quote_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut,
    seeds = [pool_account.key().as_ref(), b"base_token_account".as_ref(), pool_account.base_mint.as_ref()],
    bump
    )]
    pub base_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub base_token_user_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_mint: Account<'info, Mint>,

    /// CHECK: authority is checked, no worries
    #[account(mut)]
    pub participant: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RefundContext<'info> {
    #[account(mut)]
    pub pool_account: Box<Account<'info, IdoPoolAccount>>,

    #[account(mut, constraint = whitelist_account.owner == pool_account.key())]
    pub whitelist_account: Box<Account<'info, WhitelistAccount>>,

    #[account(mut, constraint = whitelist_account.owner == participant.key())]
    pub participant: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
