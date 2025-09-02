use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserState {
    // 用户的账号
    pub user: Pubkey,
    // 用户的香火值
    pub incense_points: u64,
    // 功德值
    pub merit: u64,
    // 每日烧香量 最多十次 通过捐助可以进行刷新
    pub incense_number: u8,
    // 更新时间
    pub update_time: i64,
}

impl UserState {
    pub const SEED_PREFIX: &str = "user_state";

    // 判断是否还可以烧-每日10次上限；通过捐助可以刷新次数
    pub fn check_incense_number(&mut self, amount: u8) -> Result<()> {
        let current_time: i64 = Clock::get()?.unix_timestamp;
        let current_day = current_time / (24 * 60 * 60);
        let last_update_day = self.update_time / (24 * 60 * 60);

        if current_day > last_update_day {
            self.incense_number = 0;
            self.update_time = current_time;
        }

        // 检查今天的烧香量加上请求数量是否超过限制
        if self.incense_number + amount > 10 {
            return err!(ErrorCode::DailyIncenseLimitExceeded);
        }

        // 增加当日烧香次数
        self.incense_number += amount;
        self.update_time = current_time;

        Ok(())
    }

    // 增加用户的香火值和功德值
    pub fn add_incense_value_and_merit(&mut self, incense_value: u64, merit: u64) {
        self.incense_points = self
            .incense_points
            .checked_add(incense_value)
            .unwrap_or(self.incense_points);
        self.merit = self.merit.checked_add(merit).unwrap_or(self.merit);
    }

    // 捐助的情况 重置
    pub fn reset_incense_number(&mut self, current_time: i64) {
        self.incense_number = 0;
        self.update_time = current_time;
    }
}
