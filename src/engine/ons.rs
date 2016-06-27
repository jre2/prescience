use super::*;

impl Unit {
    pub fn on_death( &mut self ) {
        self.is_alive = false;
        //TODO: remove all effects attached to this unit
    }
    pub fn on_damaged( &mut self, dmg : u8 ) {
        self.stats.hp -= dmg;
        if self.stats.hp <= 0 { self.on_death(); }
    }
    pub fn on_turn( &mut self, st : &mut State ) {
        self.stats.ct -= 100;

        if self.status.contains( & EffectType::Disable ) { return; }

        st.units[0].on_damaged( self.stats.atk );
    }
}
