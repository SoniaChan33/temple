use anchor_lang::prelude::*;

// 单种香型的属性配置
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct IncenseType {
    pub id: u8, // 香型ID
    #[max_len(32)]
    pub name: String, // 名称
    pub price_lamports: u64, // 单支香的价格
    pub merit: u64, // 功德值
    pub incense_points: u64, // 香火值
    pub is_donation: bool, // 是否捐助的香
}

// 寺庙配置
#[account]
#[derive(InitSpace)]
pub struct TempleConfig {
    pub index: u16,       // 配置索引
    pub owner: Pubkey,    // 寺庙管理员地址
    pub treasury: Pubkey, // 寺庙国库地址
    #[max_len(32)]
    pub incense_types: Vec<IncenseType>, // 所有香型的列表
    pub incense_points: u64, // 香火值
    pub merit: u64,       // 功德
    pub level: u8,        // 寺庙等级
}

impl TempleConfig {
    pub const SEED_PREFIX: &str = "temple_v1";
    // 获取香型类型
    pub fn find_incense_type(&self, id: u8) -> Option<&IncenseType> {
        self.incense_types.iter().find(|t| t.id == id)
    }

    // 获取香型价格
    pub fn get_fee_per_incense(&self, incense_id: u8) -> u64 {
        self.find_incense_type(incense_id)
            .map(|t: &IncenseType| t.price_lamports)
            .unwrap_or(0)
    }

    // 增加
    pub fn add_incense_value_and_merit(&mut self, incense_value: u64, merit: u64) {
        self.incense_points = self
            .incense_points
            .checked_add(incense_value)
            .unwrap_or(self.incense_points);
        self.merit = self.merit.checked_add(merit).unwrap_or(self.merit);
    }
}
