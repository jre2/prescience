#![allow(unused_variables)]
use super::*;

#[derive(Debug,Clone,Copy)]
#[repr(usize)]
pub enum AbilityType {
    Attack,
    Heal,
}

// For now, all abilities require a unit as a target. can use trait later
impl Unit {
    pub fn do_ability( &mut self, st : &mut State, at : AbilityType, u : &mut Unit ) {
        match at {
            AbilityType::Attack => {
                if !self.on_attack( u ) { return; }
                if !u.on_attacked( self ) { return; }

                let dmg = self.on_damage( u );
                u.on_damaged( dmg );
                },
            _ => {},
        }
    }
}
