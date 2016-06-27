use super::*;

#[derive(Debug,Clone,Copy)]
#[repr(usize)]
pub enum AbilityType {
    Attack,
    Heal,
}

impl Unit {
    pub fn choose_ability_randomly( &self ) -> AbilityType {
        AbilityType::Attack
    }
}
