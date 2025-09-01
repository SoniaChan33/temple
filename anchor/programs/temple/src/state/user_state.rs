use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserState {
    // 用户的账号
    pub user: Pubkey,
    // 用户的香火值
    pub incense_value: u64,
    // 功德值
    pub merit: u64,
}

impl UserState {
    pub fn new(&mut self, incense_value: u64) {}

    // 香火值更新
    pub fn incense_value_update(&self, incense_value: u64, update_time: i64) -> Result<()> {
        Ok(())
    }
}
