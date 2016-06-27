#![feature(collections)]
#![feature(enumset)]
#![feature(iter_arith)]
#![feature(libc)]

extern crate collections;
#[macro_use] extern crate itertools;
extern crate time;
extern crate libc;
extern crate rand;

mod engine;
use engine::*;

// Time execution of given function. Expects to receive turns executed
fn timeit<F>( mut f : F )
where F : FnMut() -> u32
{
    let t0 = time::precise_time_ns();
    let turns = f();
    let t = time::precise_time_ns();
    let dt = (t-t0) as f64 / 1e9;
    let tps = ((turns as f64 / dt) / 1000.0) as u32;

    println!("Completed {} turns. {} sec. {} k turns/sec", turns, dt, tps);
}

fn main() {
    let mut st = State::mk_test( 2, 4, 1.5 );
    st.render();
    st.do_turn();
    st.render();
    timeit(|| {
        for _ in 0..9000000 {
            if st.alive == 0 { break; }
            st.do_turn();
        }
        st.turn
    });

    println!("\n");
    st.render();

    let e = Effect::new().init( EffectType::Stop, 5, 2, 0 );
    println!("{:?}", e );
}
