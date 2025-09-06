use crate::error::ErrorCode;
use anchor_lang::prelude::*;

// 定义香型余额结构
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct IncenseBalance {
    pub incense_id: u8,
    pub balance: u64,
}

// 定义每日烧香次数结构
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DailyIncenseCount {
    pub incense_id: u8,
    pub count: u8,
}

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

    // 新增：按香型存储余额
    #[max_len(20)]
    pub incense_balance: Vec<IncenseBalance>,
    // 新增：按香型存储当日次数
    #[max_len(20)]
    pub daily_incense_count: Vec<DailyIncenseCount>,
    pub bump: u8,
}

impl UserState {
    pub const SEED_PREFIX: &str = "user_state";

    /// 获取指定香型的余额
    pub fn get_incense_balance(&self, incense_id: u8) -> u64 {
        self.incense_balance
            .iter()
            .find(|item| item.incense_id == incense_id)
            .map(|item| item.balance)
            .unwrap_or(0)
    }

    /// 设置指定香型的余额
    pub fn set_incense_balance(&mut self, incense_id: u8, balance: u64) {
        if let Some(item) = self
            .incense_balance
            .iter_mut()
            .find(|item| item.incense_id == incense_id)
        {
            item.balance = balance;
        } else {
            // 如果不存在，添加新的记录
            self.incense_balance.push(IncenseBalance {
                incense_id,
                balance,
            });
        }
    }

    /// 增加指定香型的余额
    pub fn add_incense_balance(&mut self, incense_id: u8, amount: u64) {
        let current_balance = self.get_incense_balance(incense_id);
        self.set_incense_balance(incense_id, current_balance.saturating_add(amount));
    }

    /// 减少指定香型的余额
    pub fn subtract_incense_balance(&mut self, incense_id: u8, amount: u64) -> Result<()> {
        let current_balance = self.get_incense_balance(incense_id);
        if current_balance < amount {
            return err!(ErrorCode::InsufficientIncenseBalance);
        }
        self.set_incense_balance(incense_id, current_balance - amount);
        Ok(())
    }

    /// 获取指定香型的当日烧香次数
    pub fn get_daily_incense_count(&self, incense_id: u8) -> u8 {
        self.daily_incense_count
            .iter()
            .find(|item| item.incense_id == incense_id)
            .map(|item| item.count)
            .unwrap_or(0)
    }

    /// 设置指定香型的当日烧香次数
    pub fn set_daily_incense_count(&mut self, incense_id: u8, count: u8) {
        if let Some(item) = self
            .daily_incense_count
            .iter_mut()
            .find(|item| item.incense_id == incense_id)
        {
            item.count = count;
        } else {
            // 如果不存在，添加新的记录
            self.daily_incense_count
                .push(DailyIncenseCount { incense_id, count });
        }
    }

    /// 检查香火量是否超出限制
    pub fn check_daily_incense_limit(&self, incense_id: u8, amount: u8) -> Result<()> {
        // 1. 先判断是否跨天，跨天则重置该香型次数
        let now = Clock::get()?.unix_timestamp;
        let is_new_day = now - self.update_time >= 86400; // 24小时=86400秒
        let current_count = if is_new_day {
            0
        } else {
            self.get_daily_incense_count(incense_id)
        };

        // 2. 校验次数（单个香型每日≤10）
        if current_count + amount > 10 {
            return err!(ErrorCode::ExceedDailyIncenseLimit);
        }
        Ok(())
    }

    /// 更新当日烧香次数
    pub fn update_daily_count(&mut self, incense_id: u8, amount: u8) {
        let now = Clock::get().unwrap().unix_timestamp;
        // 跨天则重置所有次数+更新重置时间
        if now - self.update_time >= 86400 {
            self.daily_incense_count.clear();
            self.update_time = now;
        }
        // 累加当前香型次数
        let current_count = self.get_daily_incense_count(incense_id);
        self.set_daily_incense_count(incense_id, current_count + amount);
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
