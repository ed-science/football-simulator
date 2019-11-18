use crate::club::squad::Squad;
use crate::club::board::ClubBoard;
use crate::club::tactics::Tactics;
use crate::core::SimulationContext;
use crate::player::contract::PlayerClubContract;
use crate::player::player::{Player, PlayerPosition};
use crate::staff::contract::StaffClubContract;
use crate::utils::IntegerUtils;

#[derive(Debug, Clone)]
pub struct Club {
      pub id: u32,
      pub name: String,
      pub board: ClubBoard,
      pub players: Vec<PlayerClubContract>,
      pub staffs: Vec<StaffClubContract>,
      pub tactics: Option<Tactics>,
}

impl Club {
      pub fn new(
            name: String,
            players: Vec<PlayerClubContract>,
            staffs: Vec<StaffClubContract>,
      ) -> Self {
            Club {
                  id: IntegerUtils::random(0, 1000000) as u32,
                  board: ClubBoard::new(),
                  name: name,
                  players: players,
                  staffs: staffs,
                  tactics: None,
            }
      }

      pub fn items_count(&self) -> usize {
            self.players.len()
      }

      fn select_tactics(&mut self){

      }

      pub fn get_match_squad(&self) -> Squad {
            let players = self
                  .players
                  .iter()
                  .filter(|player_contract| !player_contract.is_expired())
                  .map(|p_contract| (PlayerPosition::Goalkeeper, p_contract.player.clone()))
                  .collect();

            Squad{
                  tactics: self.tactics.as_ref().unwrap().clone(),
                  players: players
            }
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            for player in &mut self.players {
                  player.simulate(context);
            }
            for staff in &mut self.staffs {
                  staff.simulate(context);
            }
      }
}