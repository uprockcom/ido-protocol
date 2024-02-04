use std::ops::{Deref, DerefMut};

use crate::{anchor_metaplex, CreatorStruct, Err};
use anchor_lang::prelude::{
    msg, next_account_info, Account, AccountInfo, Error, ErrorCode, ProgramError, Pubkey,
};
use anchor_lang::solana_program::borsh::try_from_slice_unchecked;
use anchor_lang::{err, Key};
pub use metaplex_token_metadata::state::PREFIX as PDAPrefix;
use metaplex_token_metadata::state::{Key as MetaplexKey, Metadata, MAX_METADATA_LEN};
use metaplex_token_metadata::utils::try_from_slice_checked;
pub use metaplex_token_metadata::ID;

#[derive(Clone)]
pub struct MetaplexTokenMetadata;

impl anchor_lang::AccountDeserialize for MetaplexTokenMetadata {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, anchor_lang::error::Error> {
        MetaplexTokenMetadata::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self, anchor_lang::error::Error> {
        Ok(MetaplexTokenMetadata)
    }
}

impl anchor_lang::Id for MetaplexTokenMetadata {
    fn id() -> Pubkey {
        ID
    }
}

#[derive(Clone)]
pub struct MetadataAccount(Metadata);

impl MetadataAccount {
    pub const LEN: usize = MAX_METADATA_LEN;
}

impl anchor_lang::AccountDeserialize for MetadataAccount {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, anchor_lang::error::Error> {
        let res = try_from_slice_checked(buf, MetaplexKey::MetadataV1, MAX_METADATA_LEN)
            .map(MetadataAccount);
        if res.is_err() {
            return err!(ErrorCode::AccountDidNotDeserialize);
        }

        return Ok(res.unwrap());
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, anchor_lang::error::Error> {
        let metadata: Metadata = try_from_slice_unchecked(buf)
            .map_err(|err| ProgramError::BorshIoError(err.to_string()))?;
        Ok(MetadataAccount(metadata))
    }
}

impl anchor_lang::AccountSerialize for MetadataAccount {
    fn try_serialize<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> Result<(), anchor_lang::error::Error> {
        // no-op
        Ok(())
    }
}

impl anchor_lang::Owner for MetadataAccount {
    fn owner() -> Pubkey {
        ID
    }
}

impl Deref for MetadataAccount {
    type Target = Metadata;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MetadataAccount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn check_metadata(
    metadata: &Account<MetadataAccount>,
    nft_mint_key: &Pubkey,
    allowed_creators: Vec<CreatorStruct>,
) -> std::result::Result<CreatorStruct, Error> {
    let (expected_address, _) = Pubkey::find_program_address(
        &[
            anchor_metaplex::PDAPrefix.as_bytes(),
            &anchor_metaplex::ID.to_bytes(),
            &nft_mint_key.to_bytes(),
        ],
        &anchor_metaplex::ID,
    );
    if metadata.key() != expected_address {
        msg!("InvalidMetadataAccountAddress");
        return err!(Err::InvalidMetadataAccountAddress);
    }

    if let Some(creators) = &metadata.data.creators {
        for creator in creators.iter() {
            for allowed_creator in allowed_creators.iter() {
                if allowed_creator.eq(creator) {
                    return Ok(allowed_creator.clone());
                }
            }
        }
    }

    return err!(Err::InvalidMetadataCreators);
}

pub fn get_metadata_account<'a, 'b>(
    accounts: &'a [AccountInfo<'b>],
) -> std::result::Result<Account<'b, MetadataAccount>, Err> {
    let accounts_iter = &mut accounts.iter();
    let metadata_info = next_account_info(accounts_iter).or(Err(Err::MetadataAccountNotFound))?;
    if *metadata_info.owner != anchor_metaplex::ID {
        return Err(Err::MetadataAccountNotOwnedByCorrectProgram);
    }
    Ok(Account::try_from_unchecked(&metadata_info).or(Err(Err::InvalidMetadataAccountData))?)
}
