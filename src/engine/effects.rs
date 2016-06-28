#![allow(unused_imports)]
use super::*;
use self::EffectType::*;

macro_rules! mkes {
    ( $( $x:expr ), * ) => {
        {
            ($( ( 1 << ($x as usize) ) + )*  0)
        }
    };
}

#[derive(Debug,Clone,Copy)]
#[repr(usize)]
pub enum EffectType {
    Invalid,
    Sleep, Stun, Paralysis, Stop, Silence, Disable, Immobilize, Petrify, Dread,
    Confuse, Berserk, Charm, Controlled,
}
//Note: any effect that can have multiple instances on the same unit will need fixing

pub trait EffectGroups {
    const PREVENT_ACTION : usize = mkes![ Sleep, Stun, Petrify ];
}
