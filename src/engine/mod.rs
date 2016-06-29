#![allow(dead_code)]
pub use collections::enum_set::{EnumSet,CLike};
use std::mem;

// sibling implementation modules
#[macro_use]
mod macros;
pub use self::macros::*;

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
pub type AbilitySet = EnumSet< AbilityType >;
pub type EffectSet = EnumSet< EffectType >;

#[derive(Debug)]
pub struct State {
    units   : Vec<Unit>,    // unit.id = index in units array
    effects : EffectStorage,

    pub turn    : u32,      // unit turn count. occurs any time a unit gets 100ct
    pub round   : u32,      // absolute turn count. occurs every absolute 100ct
    ct          : u8,       // game tick level ct/spd
    spd         : u8,

    pub alive   : u8,       // bitflags for whether team has any members alive
}

#[derive(Debug)]
pub struct EffectStorage {
    pub effects : Vec<Effect>,  // sorted by unit they belong to
    pub eindex  : Vec<usize>,   // eindex[unit.id] stores index of unit's first effect
}

#[derive(Debug,Clone)]
pub struct Unit {
    pub id          : UnitId,
    team            : TeamId,
    row             : RowId,
    pub is_alive    : bool,

    status          : EffectSet,
    abilities       : AbilitySet,
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
    owner   : UnitId,   // temporary field for initial easy implementation
}

// Utility for Effect/Ability bitflags
impl CLike for EffectType {
    fn to_usize( &self ) -> usize { *self as usize }
    fn from_usize( v: usize ) -> EffectType { unsafe { mem::transmute(v) } }
}
impl CLike for AbilityType {
    fn to_usize( &self ) -> usize { *self as usize }
    fn from_usize( v: usize ) -> AbilityType { unsafe { mem::transmute(v) } }
}

pub trait EnumSetInt {
    fn load( v: usize ) -> Self;
    fn has( &self, v: usize ) -> bool;
}
impl<T> EnumSetInt for EnumSet<T> {
    fn load( v: usize ) -> EnumSet<T> { unsafe { mem::transmute(v) } }
    fn has( &self, v: usize ) -> bool {
        let bits : usize = unsafe { mem::transmute( *self ) };
        (bits & v) != 0
    }
}

// These are defined in abilities/effects mods
impl AbilityGroups  for AbilitySet {}
impl EffectGroups   for EffectSet {}

// Creation and initialization
impl UnitStats {
    fn new() -> UnitStats {
        UnitStats { ct:0, spd:10, hp:100, atk:10, heal:5 }
    }
}
impl Effect {
    pub fn new() -> Effect {
        Effect { etype : 0, ttl : 0, potency : 0, linked : 0, owner : 0 }
    }
    pub fn init( &mut self, etype : EffectType, ttl : u8, potency : i8, linked : UnitId, owner : UnitId ) -> Effect {
        let mut e = self.clone();
        e.etype = etype as u8;  // safe for <256 types
        e.ttl = ttl;
        e.potency = potency;
        e.linked = linked;
        e.owner = owner;
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
        u.abilities = AbilitySet::load( AbilitySet::BASIC );
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
            effects : EffectStorage::new(),
            turn    : 0,
            round   : 0,
            ct      : 0,
            spd     : 10, // default speed is 10 ticks per round
            alive   : 1,
        }
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
    pub fn add_unit( &mut self, team : TeamId, row : RowId ) -> &mut State {
        let u = Unit::new()
                    .init( self.units.len() as UnitId, team, row );
        self.units.push( u );
        self
    }
}

// Initial implementation is flat array that must be filtered
// elements with type=0 should be ignored
impl EffectStorage {
    pub fn new() -> EffectStorage {
        EffectStorage {
            effects : Vec::new(),
            eindex  : Vec::new(),
        }
    }

    pub fn add_effect( &mut self, e: Effect, _owner: UnitId ) {
        self.effects.push( e );
    }
    pub fn rem_effects_by_owner( &mut self, owner: UnitId ) {
        for e in &mut self.effects {
            if e.owner == owner {
                e.etype = EffectType::Invalid as u8;
            }
        }
        //println!("Removed effects for unit {}", owner);
    }
    /*
    // Finds first. ideally would return iterator of all matches though
    pub fn lookup_by_owner_type( &self, owner: UnitId, etype: EffectType ) -> & Effect {
        self.effects.iter().find(|e| e.owner == owner && e.etype == etype as u8 ).unwrap()
    }
    pub fn rem_effect( &mut self, idx : usize ) {
        self.effects[ idx ].etype = EffectType::Invalid as u8;
    }
    */
}

// random utils
//use rand::{Rng,random};
//use rand::distributions::{IndependentSample, Range};

use libc;
pub fn rnd_range( start: usize, end: usize ) -> usize { unsafe { libc::rand() as usize % end + start } }
pub fn rnd_range0( end : usize ) -> usize { unsafe { libc::rand() as usize % end } }
