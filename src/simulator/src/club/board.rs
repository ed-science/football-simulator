use crate::core::context::SimulationContext;
use crate::staff::StaffClubContract;

#[derive(Debug, Clone, Default)]
pub struct ClubBoard {
    pub director: Option<StaffClubContract>,
    pub sport_director: Option<StaffClubContract>,
}

impl ClubBoard {
    pub fn new() -> Self {
        ClubBoard {
            director: None,
            sport_director: None,
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        if self.director.is_none() {
            self.run_director_election(context);
        }

        if self.sport_director.is_none() {
            self.run_sport_director_election(context);
        }

        if context.check_contract_expiration() {
            if self.is_director_contract_expiring(context) {}

            if self.is_sport_director_contract_expiring(context) {}
        }
    }

    fn is_director_contract_expiring(&self, context: &mut SimulationContext) -> bool {
        self.director.as_ref().unwrap().is_expired(context)
    }

    fn run_director_election(&mut self, context: &mut SimulationContext) {}

    fn is_sport_director_contract_expiring(&self, context: &mut SimulationContext) -> bool {
        self.director.as_ref().unwrap().is_expired(context)
    }

    fn run_sport_director_election(&mut self, context: &mut SimulationContext) {}
}
