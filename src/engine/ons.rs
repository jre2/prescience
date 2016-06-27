#![allow(unused_variables)]
use super::*;
use rand;

impl Unit {
    pub fn on_death( &mut self ) {
        self.is_alive = false;
        //TODO: remove all effects attached to this unit
    }
    pub fn on_damaged( &mut self, dmg : u8 ) {
        self.stats.hp -= dmg;
        if self.stats.hp <= 0 { self.on_death(); }
    }
    // returns whether attack hits
    pub fn on_attacked( &mut self, attacker : &mut Unit ) -> bool {
        true
    }


    // returns whether attack hits
    pub fn on_attack( &mut self, targ : &mut Unit ) -> bool {
        true
    }
    pub fn on_damage( &mut self, targ : &mut Unit ) -> u8 {
        self.stats.atk
    }
    pub fn on_action( &mut self, at : AbilityType ) {
    }


    pub fn on_turn( &mut self, st : &mut State ) {
        self.stats.ct -= 100;
        if self.status.contains( & EffectType::Disable ) { return; }

        self.on_turn_light_playouts( st );
    }
}

use rand::distributions::{IndependentSample, Range};

// Light playouts system
impl Unit {
    pub fn choose_ability_randomly( &mut self ) -> AbilityType {
        AbilityType::Attack
    }
    pub fn choose_target_unit_randomly<'a>( &'a mut self, st : &'a mut State ) -> &mut Unit {
        let mut rng = rand::thread_rng();
        let between = Range::new( 0, st.units.len() );
        let idx = between.ind_sample(&mut rng);

        &mut st.units[ idx ]
    }
    fn on_turn_light_playouts( &mut self, st : &mut State ) {
        let at = self.choose_ability_randomly();
        let targ = unsafe { &mut *(self.choose_target_unit_randomly( st ) as *mut Unit) };
        self.do_ability( st, at, targ );
    }
}
