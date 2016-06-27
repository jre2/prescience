#![allow(dead_code)]
pub use collections::enum_set::*;
use std::mem;

// sibling implementation modules
mod render;
pub use self::render::*;

mod ons;
pub use self::ons::*;

mod effects;
pub use self::effects::*;

mod abilities;
pub use self::abilities::*;

mod update;
pub use self::update::*;

// now we define the basic types and their initialization

pub type UnitId = u8;
pub type TeamId = u8; // starting from 1
pub type RowId  = u8;
pub type Stat   = u8;

#[derive(Debug)]
pub struct State {
    units   : Vec<Unit>,    // unit.id = index in units array
    effects : Vec<Effect>,  // sorted by unit they belong to
    eindex  : Vec<usize>,   // eindex[unit.id] stores index of unit's first effect

    pub turn    : u32,      // unit turn count. occurs any time a unit gets 100ct
    pub round   : u32,      // absolute turn count. occurs every absolute 100ct
    ct          : u8,       // game tick level ct/spd
    spd         : u8,

    pub alive   : u8,       // bitflags for whether team has any members alive
}

#[derive(Debug,Clone)]
pub struct Unit {
    pub id          : UnitId,
    team            : TeamId,
    row             : RowId,
    pub is_alive    : bool,

    status          : EnumSet< EffectType >,
    abilities       : EnumSet< AbilityType >,
    pub stats       : UnitStats,
}

#[derive(Debug,Clone)]
pub struct UnitStats {
    pub ct  : u8,
    spd     : u8,

    pub hp  : u8,
    atk     : u8,
    heal    : u8,
}

// Eventually try replacing with u16-32 and manual bitshifting
#[derive(Debug,Clone)]
pub struct Effect {
    etype   : u8,       // support upto 256 effect types (planning only 64)
    ttl     : u8,       // duration in rounds; 0-14 with 15 => inf
    potency : i8,       // SMT/pokemon style "stages"; +/- 5
    linked  : UnitId,   // unit id (and thus index). ideally larger later
}

impl CLike for EffectType {
    fn to_usize( &self ) -> usize { *self as usize }
    fn from_usize( v: usize ) -> EffectType { unsafe { mem::transmute(v) } }
}
impl CLike for AbilityType {
    fn to_usize( &self ) -> usize { *self as usize }
    fn from_usize( v: usize ) -> AbilityType { unsafe { mem::transmute(v) } }
}

// Creation and initialization
impl UnitStats {
    fn new() -> UnitStats {
        UnitStats { ct:0, spd:10, hp:100, atk:10, heal:5 }
    }
}
impl Effect {
    pub fn new() -> Effect {
        Effect { etype : 0, ttl : 0, potency : 0, linked : 0 }
    }
    pub fn init( &mut self, etype : EffectType, ttl : u8, potency : i8, linked : UnitId ) -> Effect {
        let mut e = self.clone();
        e.etype = etype as u8;  // safe for <256 types
        e.ttl = ttl;
        e.potency = potency;
        e.linked = linked;
        e
    }
}
impl Unit {
    fn new() -> Unit {
        Unit { id : 0, team : 0, row : 0, is_alive : false,
            status : EnumSet::new(), abilities : EnumSet::new(), stats : UnitStats::new() }
    }
    fn init( &mut self, id : UnitId, team : TeamId, row : RowId ) -> Unit {
        let mut u = self.clone();
        u.id = id;
        u.team = team;
        u.row = row;
        u.is_alive = true;
        u.abilities.insert( AbilityType::Attack );
        u.abilities.insert( AbilityType::Heal );
        u.stats.ct = id;
        u
    }
    fn get_nth_ability( &self, idx : usize ) -> AbilityType {
        for (i,a) in self.abilities.iter().enumerate() {
            if i == idx { return a; }
        }
        AbilityType::Invalid // impossible
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
        self.eindex.push( self.effects.len() );
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

// random utils
//use rand::{Rng,random};
//use rand::distributions::{IndependentSample, Range};

use libc;
pub fn rnd_range( start: usize, end: usize ) -> usize { unsafe { libc::rand() as usize % end + start } }
pub fn rnd_range0( end : usize ) -> usize { unsafe { libc::rand() as usize % end } }
