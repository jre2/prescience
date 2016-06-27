use super::*;
use itertools::Itertools;
use std;

impl State {
    fn render_team( & self, team : TeamId, rev : bool ) {
        let rows_ = self.units.iter().filter(|u| u.team==team).map(|u| u.row).unique();
        let rows = if rev { rows_.sorted() } else { rows_.sorted_by(|a,b| b.cmp(a)) };
        for row in rows {
            let us = self.units.iter().filter(|u| u.team==team && u.row==row);
            let s = us.map(|u| u.to_string() ).join(" ");
            println!("Row {}:{}",row,s);
        }
    }

    pub fn render( & self ) {
        let teams = self.units.iter().map(|u| u.team).unique().sorted();

        // Render battlefield
        let mut rev = false;
        for t in teams.iter() {
            println!( "{:-^80}", format!(" Team {} ",t) );
            self.render_team( *t as TeamId, rev );
            rev = !rev;
        }

        // Render health summary
        let s = teams.iter().map(|&t|
            format!("Team {}:{}hp",
                t,
                self.units.iter().filter(|u| u.team==t && u.is_alive )
                    .map(|u| u.stats.hp as i32).sum::<i32>()
                )
            )
            .join(" ");
        println!("\nTurn {} Round {} CT {}. {}", self.turn, self.round, self.ct, s);
    }
}

impl std::fmt::Display for Unit {
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        if !self.is_alive { write!( f, "<DEAD>" ) }
        else { write!( f, "<{}% {} ({})>", self.stats.hp, self.stats.ct, self.id ) }
    }
}
