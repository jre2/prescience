#![allow(unused_variables)]
use super::*;
use std;

impl Unit {
    pub fn on_death( &mut self, st : &mut State ) {
        self.is_alive = false;
        st.effects.rem_effects_by_owner( self.id );
    }
    // cancel attack if fals
    pub fn on_attacked( &mut self, attacker : &mut Unit ) -> bool {
        true
    }
    pub fn on_healed( &mut self, dmg : u8 ) {
        self.stats.hp = match self.stats.hp.checked_add( dmg ) {
            None => std::u8::MAX,
            Some(hp) => hp,
        }
        // cap hp at 100 or allow >100% hp?. allowing overheal for now
    }
    pub fn on_damaged( &mut self, st : &mut State, dmg : u8 ) {
        self.stats.hp = match self.stats.hp.checked_sub( dmg ) {
            None => 0,
            Some(hp) => hp,
        };
        if self.stats.hp <= 0 && self.is_alive { self.on_death( st ); }
    }


    // cancel action if false
    pub fn on_action( &mut self, at : AbilityType ) -> bool {
        true
    }
    // cancel attack if false
    pub fn on_attack( &mut self, targ : &mut Unit ) -> bool {
        true
    }
    pub fn on_damage( &mut self, targ : &mut Unit ) -> u8 {
        self.stats.atk
    }
    pub fn on_heal( &mut self, targ : &mut Unit ) -> u8 {
        self.stats.heal
    }


    pub fn on_turn( &mut self, st : &mut State ) {
        self.stats.ct -= 100;
        if self.status.contains( & EffectType::Disable ) { return; }

        self.on_turn_light_playouts( st );
    }
}

// Light playouts system
impl Unit {
    pub fn choose_ability_randomly( &mut self ) -> AbilityType {
        let idx = rnd_range0( self.abilities.len() );
        self.get_nth_ability( idx )
    }
    pub fn choose_target_unit_randomly<'a>( &'a mut self, st : &'a mut State ) -> &mut Unit {
        let idx = rnd_range0( st.units.len() );
        &mut st.units[ idx ]
    }
    fn on_turn_light_playouts( &mut self, st : &mut State ) {
        let at = self.choose_ability_randomly();
        let targ = unsafe { &mut *(self.choose_target_unit_randomly( st ) as *mut Unit) };
        self.do_ability( st, at, targ );
    }
}
