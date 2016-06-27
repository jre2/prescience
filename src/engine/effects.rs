use super::*;

#[derive(Debug,Clone,Copy)]
#[repr(usize)]
pub enum EffectType {
    Sleep, Stun, Paralysis, Stop, Silence, Disable, Immobilize, Petrify, Dread,
    Confuse, Berserk, Charm, Controlled,
}

impl Effect {
}
