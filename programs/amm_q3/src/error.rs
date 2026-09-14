use anchor_lang::prelude::*;
use constant_product_curve::CurveError;
#[error_code]
pub enum AmmError {
    #[msg("fee percentage can only be btween 0 to 100.")]
    FeePErcentErr,
    #[msg("default error.")]
    DefaultError,
    #[msg("offer expired.")]
     OfferExpired,
     #[msg("this pool is locked.")]
     PoolLocked,
     #[msg("slippage exceeded.")]
     SlippageExceeded,
     #[msg("overflow detected.")]
     Overflow,
     #[msg("underflow detected")]
     Underflow,
     #[msg("invalid token.")]
     InvalidToken,
     #[msg("actual liquidity is less than minimum")]
     LiquidityLessThanMinimum,
     #[msg("No liquidity in pool.")]
     NoLiquidityInPool,
     #[msg("Bump error.")]
     BumpError,
     #[msg("curve error.")]
     CurveError,
     #[msg("fee is greater than 100%, this is not a very good deal")]
     InvalidFee,
     #[msg("invalid update authority")]
     InvalidAuthority,
     #[msg("no update authority set")]
     NoAuthoritySet,
     #[msg("invalid amount")]
     InvalidAmount,
     #[msg("invalid precision.")]
     InvalidPrecision,
     #[msg("insufficient balance")]
     InsufficientBalance,
     #[msg("zero balance")]
     ZeroBalance,
}

impl From<CurveError> for AmmError{
    fn from(error: CurveError)-> AmmError{
        match error{
            CurveError::InvalidPrecision=> AmmError::InvalidPrecision,
            CurveError::Overflow=>  AmmError:: Overflow,
            CurveError::Underflow=> AmmError::Underflow,
            CurveError::InvalidFeeAmount=> AmmError::InvalidFee,
            CurveError::InsufficientBalance=> AmmError::InsufficientBalance,
            CurveError::ZeroBalance=> AmmError::ZeroBalance,
            CurveError:: SlippageLimitExceeded=> AmmError::SlippageExceeded,
        }
    }
}