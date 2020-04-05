use crate::Country;

use crate::continent::{Tournament, TournamentContext};
use crate::core::context::GlobalContext;
use crate::country::CountryContext;
pub use rayon::prelude::*;

pub struct Continent {
    pub name: String,
    pub countries: Vec<Country>,

    pub tournaments: Vec<Box<dyn Tournament>>,
}

impl Continent {
    pub fn items_count(&self) -> usize {
        self.countries
            .iter()
            .map(|country| country.items_count())
            .sum()
    }

    pub fn simulate(&mut self, ctx: &mut GlobalContext) {
        let global_ctx = ctx.with_country(CountryContext::new());

        for country in &mut self.countries {
            country.simulate(global_ctx);
        }

        let mut tournament_ctx = TournamentContext::new();

        for tournament in &mut self.tournaments {
            tournament.simulate(&mut tournament_ctx, ctx)
        }
    }
}