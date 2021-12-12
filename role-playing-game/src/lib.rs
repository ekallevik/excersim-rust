// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use core::cmp;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match (self.health, self.level) {
            (0, 0..=9) => Some(Player{health: 100, mana: None, level: self.level}),
            (0, level) => Some(Player{ health: 100, mana: Some(100), level }),
            (_, _) => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = if self.health < mana_cost { 0 } else { self.health - mana_cost };
                0
            }
            Some(value) if value < mana_cost => 0,
            Some(value) => {
                self.mana = Some(value - mana_cost);
                2*mana_cost
            }
        }
    }
}
