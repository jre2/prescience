use super::*;
use self::AbilityType::*;

#[derive(Debug,Clone,Copy)]
#[repr(usize)]
pub enum AbilityType {
    Invalid,
    Attack, DoubleAttack,
    Heal,
    Sleep,
}

pub trait AbilityGroups {
    group!(ALL;     Attack, Heal, DoubleAttack );
    group!(BASIC;   Attack, Heal, DoubleAttack, Sleep );
}

// For now, all abilities require a unit as a target. can use trait later
impl Unit {
    pub fn do_ability( &mut self, st : &mut State, at : AbilityType, t : &mut Unit ) {
        if !self.on_action( at ) { return; }

        match at {
            Attack => attack( self, t, st ),
            DoubleAttack => { attack( self, t, st ); attack( self, t, st ); },
            Heal => {
                let dmg = self.on_heal( t );
                t.on_healed( dmg );
            },
            Sleep => {
                st.effects.add_effect( t, EffectType::Sleep, 10, 0, 0 );
                t.stats.ct = 0;
            },
            _ => panic!("Ability not implemented"),
        }
    }
}

fn attack( me: &mut Unit, e: &mut Unit, st: &mut State ) {
    if !me.on_attack( e )   { return; }
    if !e.on_attacked( me ) { return; }

    let dmg = me.on_damage( e );
    e.on_damaged( st, dmg );
}
