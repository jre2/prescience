use super::*;
use self::EffectType::*;

#[derive(Debug,Clone,Copy)]
#[repr(usize)]
pub enum EffectType {
    Invalid,
    Sleep, Stun, Paralysis, Stop, Silence, Disable, Immobilize, Petrify, Dread,
    Confuse, Berserk, Charm, Controlled,
}

pub trait EffectGroups {
    group!(PREVENT_ACTION; Sleep, Stun, Petrify);
}

impl Effect {}
