use crate::continent::{Tournament, TournamentContext};
use crate::core::context::GlobalContext;

pub struct LeagueEurope {}

impl LeagueEurope {}

impl Tournament for LeagueEurope {
    fn simulate(&mut self, tournament_ctx: &mut TournamentContext, ctx: &mut GlobalContext) {
    }
}