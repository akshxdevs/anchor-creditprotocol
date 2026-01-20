use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub user: Pubkey,
    pub total_loans_taken: u32,
    pub total_loans_repaid: u32,
    pub total_defaults: u32,
    pub reputation_score: u32,
    pub last_loan_ts: i64,
}
#[derive(Clone, AnchorDeserialize, AnchorSerialize, InitSpace)]
pub enum UserTier {
    Tier0,
    Tier1,
    Tier2,
    Tier3,
}

pub fn calculate_user_tier(profile: &UserProfile) -> UserTier {
    match profile.reputation_score {
        0..=200 => UserTier::Tier0,
        201..=500 => UserTier::Tier1,
        501..=800 => UserTier::Tier2,
        _ => UserTier::Tier3,
    }
}
