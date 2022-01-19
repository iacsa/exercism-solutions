pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        Some(Player {
            health: 100,
            mana: Some(100).filter(|_| self.level >= 10),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana >= mana_cost {
                self.mana = Some(mana - mana_cost);
                return mana_cost * 2;
            }
        } else {
            self.health = self.health.saturating_sub(mana_cost);
        }
        0
    }
}
