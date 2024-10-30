use anchor_lang::prelude::error_code;

#[error_code]
pub enum ERROR{
    #[msg("Max Staking limit reached")]
    MaxStakeLimitReached,
    #[msg("Unstake ealier than required")]
    StakingPeriodNotElapsed
}