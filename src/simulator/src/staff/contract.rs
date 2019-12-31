use crate::core::SimulationContext;
use crate::staff::staff::Staff;

pub use chrono::prelude::{DateTime, Datelike, NaiveDate, Utc};

use crate::StaffEvent;
use std::borrow::Cow;
use std::iter;

#[derive(Debug, Clone, PartialEq)]
pub enum StaffPosition {
    SportDirector,
    MainCoach,
    Coach,
    Physio,
}

#[derive(Debug, Clone)]
pub struct StaffClubContract {
    staff: Staff,
    expired: NaiveDate,
    position: StaffPosition,
}

impl StaffClubContract {
    pub fn new(staff: Staff, expired: NaiveDate, position: StaffPosition) -> Self {
        StaffClubContract {
            staff,
            expired,
            position,
        }
    }

    pub fn is_expired(&self, context: &mut SimulationContext) -> bool {
        self.expired >= context.date.date()
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) -> Vec<StaffEvent> {
        if self.is_expired(context) {}

        self.staff.simulate(context)
    }
}

#[derive(Debug, Clone)]
pub struct StaffCollection {
    pub staffs: Vec<StaffClubContract>,
    pub roles: StaffRoles,
}

#[derive(Debug, Clone)]
pub struct StaffRoles {
    main_coach: Option<StaffClubContract>,
    contract_resolver: Option<StaffClubContract>,
}

impl StaffCollection {
    pub fn new(staffs: Vec<StaffClubContract>) -> Self {
        StaffCollection {
            staffs,
            roles: StaffRoles {
                main_coach: None,
                contract_resolver: None,
            },
        }
    }

    pub fn len(&self) -> usize {
        self.staffs.len()
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) -> Vec<StaffEvent> {
        self.staffs
            .iter_mut()
            .flat_map(|staff| staff.simulate(context))
            .collect()
    }

    pub fn get_main_coach(&self) -> Staff {
        let main_coach_contract = self
            .staffs
            .iter()
            .find(|c| c.position == StaffPosition::MainCoach);

        if main_coach_contract.is_none() {
            return Staff::stub();
        }

        main_coach_contract.unwrap().staff.clone()
    }
}
