use super::*;

#[derive(Debug,Clone,Copy)]
#[repr(usize)]
pub enum EffectType {
    Invalid,
    Sleep, Stun, Paralysis, Stop, Silence, Disable, Immobilize, Petrify, Dread,
    Confuse, Berserk, Charm, Controlled,
}
//Note: any effect that can have multiple instances on the same unit will need fixing

impl Effect {
}
