use anchor_lang::prelude::*;

use crate::errors::Err;
use crate::{
    IdoPoolAccount, StartsEnds, ADMIN_PUBKEY,
};

pub fn is_admin(signer: &Signer) -> Result<()> {
    if signer.key.to_string().as_bytes() != ADMIN_PUBKEY {
        return err!(Err::ErrPermissionDenied);
    }

    Ok(())
}

pub fn is_in_distribution(pool_account: &IdoPoolAccount) -> Result<()> {
    // if pool_account.distribution_type == DISTRIBUTION_TYPE_DROP
    //     || pool_account.distribution_type == DISTRIBUTION_TYPE_STANDARD
    // {
    //     return is_in_the_phase(&pool_account.phase_distribution);
    // }

    // it has vesting period, let's calculate latest?
    if pool_account.phase_distribution.starts_at > Clock::get()?.unix_timestamp as u64 {
        err!(Err::ErrPhaseHasNotBeenStarted)
    } else {
        Ok(())
    }
}

pub fn is_in_the_phase(phase: &StartsEnds) -> Result<()> {
    // #[cfg(feature = "no-date-check")]
    //return Ok(());
    //return err!(Err::ErrPermissionDenied);

    let clock = Clock::get()?;
    msg!(
        "ts: {} - sa: {} - ea: {}",
        clock.unix_timestamp,
        phase.starts_at,
        phase.ends_at
    );
    if clock.unix_timestamp as u64 <= phase.starts_at {
        return err!(Err::ErrPhaseHasNotBeenStarted);
    } else if phase.ends_at <= clock.unix_timestamp as u64 {
        return err!(Err::ErrIdoEnd);
    }
    Ok(())
}

pub fn is_phase_over(phase: &StartsEnds) -> Result<()> {
    //#[cfg(feature = "no-date-check")]
    //return Ok(());

    let clock = Clock::get()?;
    if (clock.unix_timestamp as u64) < phase.ends_at {
        return err!(Err::ErrWrongTimeForPhase);
    }
    Ok(())
}

pub fn assert(phase: bool) -> Result<()> {
    if phase {
        return Ok(());
    }
    err!(Err::ErrPermissionDenied)
}
