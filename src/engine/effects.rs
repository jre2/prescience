use super::*;
use std::mem;

#[derive(Debug,Clone,Copy)]
#[repr(usize)]
pub enum EffectType {
    Sleep, Stun, Paralysis, Stop, Silence, Disable, Immobilize, Petrify, Dread,
    Confuse, Berserk, Charm, Controlled,
}
impl CLike for EffectType {
    fn to_usize( &self ) -> usize { *self as usize }
    fn from_usize( v: usize ) -> EffectType { unsafe { mem::transmute(v) } }
}

impl Effect {
    pub fn on_update( &mut self ) {
        if self.ttl == 0 { return; }
        self.ttl -= 1;
    }
}
