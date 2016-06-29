use super::*;
use self::AbilityType::*;

#[derive(Debug,Clone,Copy)]
#[repr(usize)]
pub enum AbilityType {
    Invalid,
    Attack,
    Heal,
    DoubleAttack,
}

pub trait AbilityGroups {
    group!(ALL; Attack, Heal, DoubleAttack );
    group!(BASIC; Attack, Heal);
}

// For now, all abilities require a unit as a target. can use trait later
impl Unit {
    pub fn do_ability( &mut self, st : &mut State, at : AbilityType, u : &mut Unit ) {
        if !self.on_action( at ) { return; }

        match at {
            AbilityType::Attack => {
                if !self.on_attack( u ) { return; }
                if !u.on_attacked( self ) { return; }

                let dmg = self.on_damage( u );
                u.on_damaged( st, dmg );
                },

            AbilityType::Heal => {
                let dmg = self.on_heal( u );
                u.on_healed( dmg );
            }
            _ => {},
        }
    }
}
