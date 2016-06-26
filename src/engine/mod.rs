#![allow(dead_code)]

// sibling implementation modules
mod render;
pub use self::render::*;

mod ons;
pub use self::ons::*;

// other modules

pub type UnitId = u8;
pub type TeamId = u8; // starting from 1
pub type RowId = u8;
pub type Stat = u8;

#[derive(Debug)]
pub struct State {
    units   : Vec<Unit>,    // unit.id = index in units array
    effects : Vec<Effect>,  // sorted by unit they belong to
    eindex  : Vec<usize>,   // eindex[unit.id] stores index of unit's first effect

    pub turn    : u32,      // unit turn count. occurs any time a unit gets 100ct
    pub round   : u32,      // absolute turn count. occurs every absolute 100ct
    ct          : u8,
    spd         : u8,

    pub alive   : u8,       // bitflags for whether team has any members alive
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Unit {
    pub id          : UnitId,
    team            : TeamId,
    row             : RowId,
    pub is_alive    : bool,
    pub stats       : UnitStats,
}
#[derive(Debug)]
#[derive(Clone)]
pub struct UnitStats {
    pub ct  : u8,
    spd     : u8,
    pub hp  : u8,
    atk     : u8,
    heal    : u8,
}

// Eventually try replacing with u16-32 and manual bitshifting
#[derive(Debug)]
struct Effect {
    etype   : u8,       // support upto 256 effect types (planning only 64)
    ttl     : u8,       // duration in rounds; 0-14 with 15 => inf
    potency : i8,       // SMT/pokemon style "stages"; +/- 5
    linked  : UnitId,   // unit id (and thus index). ideally larger later
}

impl UnitStats {
    fn new() -> UnitStats {
        UnitStats { ct:0, spd:10, hp:100, atk:20, heal:5 }
    }
}
impl Effect {
    fn new() -> Effect {
        Effect { etype : 0, ttl : 0, potency : 0, linked : 0 }
    }
}
impl Unit {
    fn new() -> Unit {
        Unit { id : 0, team : 0, row : 0, is_alive : false, stats : UnitStats::new() }
    }
    fn init( &mut self, id : UnitId, team : TeamId, row : RowId ) -> Unit {
        let mut u = self.clone();
        u.id = id;
        u.team = team;
        u.row = row;
        u.is_alive = true;
        u.stats.ct = id;
        u
    }
}
impl State {
    pub fn new() -> State {
        State {
            units   : Vec::new(),
            effects : Vec::new(),
            eindex  : Vec::new(),
            turn    : 0,
            round   : 0,
            ct      : 0,
            spd     : 10, // default speed is 10 ticks per round
            alive   : 1,
        }
    }
    pub fn add_unit( &mut self, team : TeamId, row : RowId ) -> &mut State {
        let u = Unit::new()
                    .init( self.units.len() as UnitId, team, row );
        self.units.push( u );
        self
    }
    pub fn mk_test( teams : TeamId, rows : RowId, row_size_mult : f32) -> State {
        let mut st = State::new();

        let mut row_size;
        for team in 1..teams+1 {
            row_size = 1.0_f32;
            for row in (1..rows+1).rev() {
                for _ in 0..(row_size as i32) {
                    st.add_unit( team, row );
                }
                row_size = (row_size_mult * row_size).ceil();
            }
        }
        st
    }
}

// Update and turn processing functions
impl Unit {
    fn update( &mut self ) {
        if self.is_alive {
            self.stats.ct += self.stats.spd
        }
    }
}
impl State {
    // run one game tick (a sub-turn/round unit of time).
    // mostly determines when rounds/turns occur
    fn update( &mut self ) {
        // Update absolute time (rounds) and update effects if new round occurs
        self.ct += self.spd;
        if self.ct >= 100 {
            self.round += 1;
            self.ct = 0;
            //TODO: run updates on all effects
        }

        // Update units and track whether any are alive for each team
        for u in &mut self.units {
            u.update();
            if u.is_alive { self.alive |= 1 << (u.team-1); } // maybe move this to unit death handler?
        }
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