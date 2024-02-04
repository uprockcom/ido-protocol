use anchor_lang::prelude::*;

#[error_code]
pub enum Err {
    #[msg("Invalid IDO Type")]
    InvalidIdoDistributionType,

    #[msg("Sum of vesting periods must be 100")]
    InvalidVestingPeriods,

    #[msg("Phase ended")]
    ErrPhaseEnded,

    #[msg("Sold out")]
    ErrSoldOut,

    #[msg("Tokens will be airdropped into your account.")]
    ErrTokensWillDrop,

    #[msg("Phase has not been started")]
    ErrPhaseHasNotBeenStarted,

    #[msg("Wrong time for that phase")]
    ErrWrongTimeForPhase,

    #[msg("Insufficient Ticket To Participate")]
    ErrInsufficientTicket,

    #[msg("Wrong Amount of Ticket To Participate")]
    ErrWrongAmountOfTicket,

    #[msg("Max allocation exceeded ")]
    ErrMaxAllocationExceeded,

    #[msg("IDO has not started yet")]
    ErrIdoHaveNotStarted,

    #[msg("Permission denied")]
    ErrPermissionDenied,

    #[msg("Fund has not been raised yet")]
    ErrFundHasNotBeenRaisedYet,

    #[msg("Fund has already been raised, you have claim your tokens")]
    ErrFundHasAlreadyBeenRaisedYet,

    #[msg("IDO End")]
    ErrIdoEnd,

    #[msg("IDO times are not sequential")]
    ErrIdoSeq,

    #[msg("Invalid NFT")]
    ErrInvalidNFT,

    #[msg("NFT Already locked")]
    ErrNFTAlreadyLocked,

    #[msg("Invalid Timestamp")]
    ErrInvalidTimestamp,

    #[msg("The provided reward mint doesn't have the correct minting authority")]
    RewarderNotMintAuthority,

    #[msg("The provided authority is not valid for the rewarder")]
    InvalidRewarderAuthority,

    #[msg("The provided rewarder does not match the stake account")]
    InvalidRewarder,

    #[msg("The provided owner does not own the stake account")]
    InvalidOwnerForStakeAccount,

    #[msg("The provided Mint is not valid for the provided Rewarder")]
    InvalidRewardMint,

    #[msg("The provided reward token account is not owned by the provided owner")]
    InvalidOwnerForRewardToken,

    #[msg("The provided reward token account is not for the reward token mint")]
    InvalidRewardTokenAccount,

    #[msg("The provided NFT Mint has a supply that isn't 1")]
    InvalidNFTMintSupply,

    #[msg("The provided NFT token account is not owned by the provided owner")]
    InvalidNFTOwner,

    #[msg("The provided NFT token account is not for the NFT mint")]
    InvalidNFTAccountMint,

    #[msg("The provided NFT token account does not have the token")]
    NFTAccountEmpty,

    #[msg("The provided NFT token account is not owned by the provided stake account")]
    InvalidStakedNFTOwner,

    #[msg("There was no Metaplex Metadata account supplied")]
    MetadataAccountNotFound,

    #[msg("The Metaplex Metadata account is not owned by the Metaplex Token Metadata program")]
    MetadataAccountNotOwnedByCorrectProgram,

    #[msg("The Metaplex Metadata account failed to deserialze")]
    InvalidMetadataAccountData,

    #[msg("The Metaplex Metadata account did not have the expected PDA seeds")]
    InvalidMetadataAccountAddress,

    #[msg("The Metaplex Metadata account did not have the expected update authority")]
    InvalidMetadataUpdateAuthority,

    #[msg("The Metaplex Metadata account did not have a name beginning with the collection")]
    InvalidMetadataCollectionPrefix,

    #[msg("The Metaplex Metadata account did not have the expected creators")]
    InvalidMetadataCreators,
}
