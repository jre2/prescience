#![allow(dead_code)]
#![feature(iter_arith)]
#![feature(test)]
#![feature(libc)]

extern crate libc;
extern crate rand;
extern crate time;

#[derive(Debug)]
#[derive(PartialEq)]
enum GameStage { InProgress, Team0Won, Team1Won, }

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Team { Team0, Team1 }

#[derive(Debug)]
struct State {
    turn : u32,
    stage : GameStage,

    next_unit_id : u32,
    units : Vec< Unit >,
}

#[derive(Debug)]
#[derive(Clone)]
struct Unit {
    id : u32,

    team : Team,
    row : u32,

    ct : i32,
    spd : i32,

    hp : i32,
    atk : i32,
    heal : i32,
    is_alive : bool,
}

impl State {
    fn new() -> State {
        State {
            turn : 0,
            stage : GameStage::InProgress,
            next_unit_id : 0,
            units : Vec::new(),
        }
    }
    fn add_unit( &mut self, team : Team, row : u32 ) -> &mut State {
        let u = Unit::new( self.next_unit_id, team, row );
        self.units.push( u );
        self.next_unit_id += 1;
        self
    }
    fn mk_test() -> State {
        let mut st = State::new();
        for _ in 0..1 {
            st.add_unit( Team::Team0, 4 );
            st.add_unit( Team::Team1, 4 );
        }
        for _ in 0..2 {
            st.add_unit( Team::Team0, 3 );
            st.add_unit( Team::Team1, 3 );
        }
        for _ in 0..3 {
            st.add_unit( Team::Team0, 2 );
            st.add_unit( Team::Team1, 2 );
        }
        for _ in 0..5 {
            st.add_unit( Team::Team0, 1 );
            st.add_unit( Team::Team1, 1 );
        }
        st
    }
    fn render( &self ) {
        for row in (1..5+1).rev() {
            let x = self.units.iter()
                .filter( |u| u.row==row && u.team == Team::Team0 )
                .map( |u| u.to_string() ).collect::<Vec<_>>().join(" ");
            println!("{}", x);
        }

        println!("{}", std::iter::repeat("-").take(80).collect::<String>() );

        for row in 1..5+1 {
            let x = self.units.iter()
                .filter( |u| u.row==row && u.team == Team::Team1 )
                .map( |u| u.to_string() ).collect::<Vec<_>>().join(" ");
            println!("{}", x);
        }

        let team0_hp = self.units.iter()
            .filter( |u| u.team == Team::Team0 && u.is_alive )
            .map( |u| u.hp )
            .sum::<i32>();
        let team1_hp = self.units.iter()
            .filter( |u| u.team == Team::Team1 && u.is_alive )
            .map( |u| u.hp )
            .sum::<i32>();
        println!("Turn {}. Team 0: {} Team 1: {}", self.turn, team0_hp, team1_hp );
    }
    fn update( &mut self ) {
        let mut any_team0_alive = false;
        let mut any_team1_alive = false;

        for u in &mut self.units {
            u.update();
            if u.is_alive && u.team == Team::Team0 { any_team0_alive = true; }
            if u.is_alive && u.team == Team::Team1 { any_team1_alive = true; }
        }

        self.stage = GameStage::InProgress;
        if !any_team0_alive { self.stage = GameStage::Team1Won; }
        if !any_team1_alive { self.stage = GameStage::Team0Won; }
    }
    /*
    fn _do_turn( &mut self ) {
        let mut id = None;
        while self.stage == GameStage::InProgress {
            id = match self.units.iter_mut().filter( |u| u.ct >= 100 && u.is_alive ).max_by_key( |u| u.ct ) {
                None => None,
                Some(u) => Some(u.id),
            };
            match id {
                None => self.update(),
                Some(_) => { break; }
            }
        }

        if self.stage == GameStage::InProgress {
            let u = self.units.iter_mut().filter( |u| u.id == id.unwrap() ).nth(0).unwrap();
            println!("do_turn with unit {}", u );
        }
    }*/
    fn get_next_ready_unit_idx( &mut self ) -> usize {
        let mut idx = None;
        while self.stage == GameStage::InProgress {
            for (i,u) in self.units.iter().enumerate() {
                if !u.is_alive { continue; }
                if let None = idx { idx = Some(i); }
                if u.ct > self.units[ idx.unwrap() ].ct { idx = Some(i); }
            }

            if self.units[ idx.unwrap() ].ct >= 100 { break; }
            self.update();
        }
        idx.unwrap()
    }
    fn do_turn( &mut self ) {
        let u_idx = self.get_next_ready_unit_idx();
        let u = self.units[ u_idx ].clone();

        if self.stage == GameStage::InProgress {
            self.turn += 1;
            self.do_turn_unit( & u, u_idx );
        }
    }
    fn do_turn_unit( &mut self, me : &Unit, me_idx : usize ) {
        {
            let ref mut u = self.units[ me_idx ];
            u.ct -= 100;
        }

        let rnd_action = unsafe { libc::rand() as u32 % 2 +1 };
        match rnd_action {
            1 => rnd_attack( me, self ),
            2 => rnd_heal( me, self ),
            _ => panic!("impossible"),
        };
    }
}

fn choice<F>( xs : &mut Vec<Unit>, pred : F ) -> Option< &mut Unit >
where F : Fn(&mut Unit) -> bool
//fn choice( xs : &mut Vec<Unit>, pred : Fn(&mut Unit) -> bool ) -> Option< &mut Unit >
{
    let mut seen = 0;
    let mut y = None;

    for x in xs {
        if !pred(x) { continue; }
        seen += 1;
        unsafe { if ( libc::rand() as i32 % seen ) == 0 { y = Some(x); } }
    }
    y
}

fn rnd_attack( me : &Unit, st : &mut State ) {
    match choice( &mut st.units, |u| u.is_alive && u.team != me.team ) {
        Some(e) => do_attack( me, e ),
        None => (), // if no valid target, take no action
    }
}
fn rnd_heal( me : &Unit, st : &mut State ) {
    let mut fallback = false;
    {
        let ma = choice( &mut st.units, |u| u.is_alive && u.team == me.team && u.hp < 100 );
        match ma {
            Some(a) => do_heal( me, a ),
            None => fallback = true, //was `rnd_attack( me, st ),`
        }
    }
    if fallback { rnd_attack( me, st ); }
}

fn do_attack( me : &Unit, e : &mut Unit ) {
    let dist = me.row + e.row;
    let dist_f = 2.0 / dist as f32;
    let dmg = (me.atk as f32 * dist_f) as i32;
    e.hp -= dmg;
}
fn do_heal( me : &Unit, a : &mut Unit ) {
    let hp = std::cmp::min( me.heal, 100-a.hp );
    a.hp += hp;
}

impl std::fmt::Display for Unit {
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        if !self.is_alive { write!( f, "<DEAD>" ) }
        else { write!( f, "<{}% {} ({})>", self.hp, self.ct, self.id ) }
    }
}

impl Unit {
    fn new( id : u32, team : Team, row : u32 ) -> Unit {
        Unit {
            id : id, team : team,
            row : row,
            ct : id as i32,
            spd : 4,
            hp : 100,
            atk : 2, heal : 1,
            is_alive : true,
        }
    }
    fn update( &mut self ) {
        if self.hp <= 0 { self.is_alive = false; }

        if self.is_alive {
            self.ct += self.spd
        }
    }
}

extern crate test;
#[bench]
fn bench_do_turn( b: &mut test::Bencher ) {
    let mut st = State::mk_test();
    b.iter(|| st.do_turn() );
}

fn main() {
    let mut st = State::mk_test();

    let t0 = time::precise_time_ns();
    for _ in 0..9000000 {
        if st.stage != GameStage::InProgress { break; }

        st.do_turn();
        //st.render();
    }
    let t = time::precise_time_ns();
    let dt = (t-t0) as f64 / 1e9;
    let tps = ((st.turn as f64 / dt) / 1000.0) as u32;

    println!("Stage {:?} on turn {}. {} sec {} k turns/sec", st.stage, st.turn, dt, tps );
}
