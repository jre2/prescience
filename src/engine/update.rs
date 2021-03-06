use super::*;

// Game tick updates and turn processing
impl State {
    // run one game tick (a sub-turn/round unit of time).
    // mostly determines when rounds/turns occur
    pub fn update( &mut self ) {
        // Update absolute time (rounds) and update effects if new round occurs
        self.ct += self.spd;
        if self.ct >= 100 {
            self.round += 1;
            self.ct = 0;
            self.update_effects();
        }

        // Update units and track whether any are alive for each team
        self.alive = 0;
        self.update_units();
    }
    fn get_next_ready_unit( &mut self ) -> Option< &mut Unit > {
        while self.alive != 0 {
            match self.units.iter_mut().filter(|u| u.is_alive && u.stats.ct >= 100).max_by_key(|u| u.stats.ct) {
                Some(u) => unsafe { return Some( &mut *( u as *mut Unit) ); },
                None => (), // lexical alias scoping prevents calling self.update() here
            }
            self.update();
        }
        None    // impossible iff self.alive is accurate
    }
    pub fn do_turn( &mut self ) {
        // get unit who takes turn next. it's safe to ignore aliasing rules here
        let u = unsafe { match self.get_next_ready_unit() {
            None => { return; },
            Some(u) => &mut *( u as *mut Unit ),
        } };
        u.on_turn( self );
        self.turn += 1;
    }
}

// Update units
impl State {
    fn update_units( &mut self ) {
        for u in &mut self.units {
            u.update();
            if u.is_alive { self.alive |= 1 << (u.team-1); } // maybe move this to unit death handler?
        }
    }
}
impl Unit {
    // runs every game tick (absolute time)
    fn update( &mut self ) {
        if self.is_alive {
            if self.effects.has( EffectSet::PREVENT_CT ) { return; }
            self.stats.ct += self.stats.spd
        }
    }
}

// Update effects
impl State {
    fn update_effects( &mut self ) {
        // implementation specific, won't work for fast version
        self.effects.holes = 0;
        for uid in 0..self.units.len() {
            for e in &mut self.effects.effects.iter_mut().filter(|x| x.owner == uid as u8) {
                if e.on_update() {
                    self.effects.holes += 1;
                    let et = EffectType::from_usize( e.etype as usize );
                    self.units[uid].effects.remove( & et );
                }
            }
        }
        self.effects.defrag();
    }
}

impl Effect {
    // runs every game round (absolute time). return if removed
    pub fn on_update( &mut self ) -> bool {
        if self.ttl == 0 || self.etype == EffectType::Invalid as u8 { return true; }
        self.ttl -= 1;
        // temp hack to handle bad defrag
        if self.ttl == 0 || self.etype == EffectType::Invalid as u8 { return true; }
        false
    }
}
