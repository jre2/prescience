# Prescience
A turn based tactical rpg with time travel mechanics

###### TODO
* units need to lookup their effects (for potency and linked unit)

# Requirements
##### Unit actions
* apply/remove instaneous effects to self or other units
    * dmg enemy
    * heal ally
* apply/remove ongoing effects to self or other units
    * add (de)buff to stats of other unit
    * add (not-stat buff based) effect to other unit
    * dispell effect on another unit

### Effects
-----------
**Implicit (modify unit on buff add/removal)**
* On- (Attribute (de)buff) [potency]:
    * bravery, cowardice, heavy (-move spd), slow, hasted, protect/shell (phys/mag def)

**Implicit (modify unit on buff update)**
* On- (HoT/DoT) [potency]:
    * poison, burning, bleeding, regen

**Just check status bit**
* On- (Death Sentence):
    * doom, gradual petrify/slow numb #once duration ends, replace with final form
* OnTurn:
    * doubling/trippling (multiple actions per turn)
* OnAction:
    * sleep, stunned/paralzyed/stopped/silenced/disabled/immobilized/petrified/dread, (prevent or restrict types of actions can take)
    * confuse/berserk (#???[random]???# action taken or effective team),
    * charm, controlled (change effective team)
    * overheating (take dmg/die if take action) #???[potency]???#
* OnAttack [potency]:
    * blind (-acc, every Nth atk misses or hits other target, etc), enlarge/mini (-acc +atk +def +every Nth auto-hits)
        * ^^ psuedo potency that works with duration to determine when to set flag
* OnAttacked:
    * sleep, (phys/mag) blinking, reflecting, floating, decoyed (avoid next atk)

**Unit needs to lookup buff during action**
* OnDamage [potency?]:
    * empowered-for-dmg-type (eg. en-holy, en-dark)
* OnDamaged [potency?]:
    * vulnerable-to-dmg-type (eg. oiled/wet/frozen)
* OnDamaged/OnHealed/OnStatused [linked unit]:
    * guarded (defender)
    * life-link (ally/enemy also loses/gains hp)
    * synchronize (ally/enemy also gains status)

### Buff/Effects Implementation
-----------------------
**On the unit**

Unit checks if affected by an effect with simple bit flag check for majority of buffs.
Only needs to lookup buff if additional data is required (eg. potency or linked unit).

    EffectsBitFlags (64b) -- quick check for presence of certain status effect

**Separate**

Keep an array of all effects so we can update them in vectorized fashion.
Keep sorted by unit it pertains to for improved cache performance (when effects need to modify unit's bitflags, when unit needs to lookup its buffs, and reduces size as we don't store unit ids).
Additionally keep lookup of where in the array an unit's effects start and end.

    UnitId2BuffArrayRange :: Map UnitId (StartIdx, EndIdx)
    EffectsArray -- array of effects, sorted by unit

**Effects Data**
These are minimums based on early brainstorming of effects

    SimpleBuff (9b)
        5b type (upto 32)
        4b duration (upto 15 rounds or inf)
    Buff (13b)
        6b type (upto 64)
        4b duration (upto 15 rounds or inf)
        3b potency (upto +/- 5 stages)
    LinkedBuff (15b)
        2b type (upto 4)
        4b duration (upto 15 rounds or inf)
        9b linked unit (upto 512) -- for guard, life-link, synchronize, etc
