use super::distributions::random;
use crate::club::PlayerPositionType;
use crate::Squad;
use std::mem;

pub struct FootballEngine<'s> {
    pub home_squad: Squad<'s>,
    pub away_squad: Squad<'s>,
}

const MATCH_ACTIONS: u16 = 50;
const DEFAULT_MATCH_EVENTS: usize = 100;

impl<'s> FootballEngine<'s> {
    pub fn new(home_squad: Squad<'s>, away_squad: Squad<'s>) -> Self {
        FootballEngine {
            home_squad,
            away_squad,
        }
    }

    pub fn play(&mut self) -> FootballMatchDetails {
        let mut field_zone = MatchFieldZone::Midfield;

        let mut result = FootballMatchDetails {
            score: Score { home: 0, away: 0 },
            events: Vec::with_capacity(DEFAULT_MATCH_EVENTS),
            player_changes: Vec::new(),
        };

        let home_team = self.get_team_for_squad(&self.home_squad);
        let away_team = self.get_team_for_squad(&self.away_squad);

        let mut attacking_team = &home_team;
        let mut defending_team = &away_team;

        for _ in 0..MATCH_ACTIONS {
            let winner_team = self.get_battle_winner(&attacking_team, &defending_team, &field_zone);

            if winner_team.id == attacking_team.id {
                if attacking_team.id == home_team.id {
                    if field_zone == MatchFieldZone::BGoal {
                        result.score.home += 1;

                        field_zone = MatchFieldZone::Midfield;
                        mem::swap(&mut attacking_team, &mut defending_team);

                        if let Some(ev) = Self::goalscorer(attacking_team) {
                            result.events.push(MatchEvent::Goal(ev));
                        }
                    } else {
                        field_zone = Self::up_field(&field_zone);
                    }
                } else {
                    if field_zone == MatchFieldZone::AGoal {
                        result.score.away += 1;

                        field_zone = MatchFieldZone::Midfield;
                        mem::swap(&mut attacking_team, &mut defending_team);

                        if let Some(ev) = Self::goalscorer(defending_team) {
                            result.events.push(MatchEvent::Goal(ev));
                        }
                    } else {
                        field_zone = Self::down_field(&field_zone);
                    }
                }
            } else {
                field_zone = MatchFieldZone::Midfield;
                mem::swap(&mut attacking_team, &mut defending_team);
            }
        }

        for home_player in &self.home_squad.main_squad {
            result
                .events
                .push(MatchEvent::MatchPlayed(home_player.player.id, true, 90));
        }

        for away_player in &self.away_squad.main_squad {
            result
                .events
                .push(MatchEvent::MatchPlayed(away_player.player.id, true, 90));
        }

        result
    }

    fn goalscorer(team: &MatchTeam) -> Option<u32> {
        let random_player = random(0.0, 5.0) as usize;

        match team
            .squad
            .main_squad
            .get(team.squad.main_squad.len() - random_player)
        {
            Some(squad_player) => Some(squad_player.player.id),
            None => None,
        }
    }

    fn up_field(field: &MatchFieldZone) -> MatchFieldZone {
        match field {
            MatchFieldZone::AGoal => MatchFieldZone::Midfield,
            MatchFieldZone::AField => MatchFieldZone::Midfield,
            MatchFieldZone::Midfield => MatchFieldZone::BField,
            MatchFieldZone::BField => MatchFieldZone::BGoal,
            MatchFieldZone::BGoal => MatchFieldZone::BField,
            _ => MatchFieldZone::Midfield,
        }
    }

    fn down_field(field: &MatchFieldZone) -> MatchFieldZone {
        match field {
            MatchFieldZone::BGoal => MatchFieldZone::Midfield,
            MatchFieldZone::BField => MatchFieldZone::Midfield,
            MatchFieldZone::Midfield => MatchFieldZone::AField,
            MatchFieldZone::AField => MatchFieldZone::AGoal,
            MatchFieldZone::AGoal => MatchFieldZone::AField,
            _ => MatchFieldZone::Midfield,
        }
    }

    fn get_battle_winner<'a>(
        &self,
        attacking_team: &'a MatchTeam<'a>,
        defending_team: &'a MatchTeam<'a>,
        current_zone: &MatchFieldZone,
    ) -> &'a MatchTeam<'a> {
        let mut attacking_team_skill = 0.0;
        let mut defending_team_skill = 0.0;

        match current_zone {
            MatchFieldZone::AField | MatchFieldZone::BField => {
                attacking_team_skill = attacking_team.striker_skill;
                defending_team_skill = defending_team.defender_skill;
            }
            MatchFieldZone::AGoal | MatchFieldZone::BGoal => {
                attacking_team_skill = attacking_team.defender_skill;
                defending_team_skill = defending_team.striker_skill;
            }
            MatchFieldZone::Midfield => {
                attacking_team_skill = attacking_team.midfielder_skill;
                defending_team_skill = defending_team.midfielder_skill;
            }
            _ => {}
        }

        let random_a = random(0.0, attacking_team_skill as f64);
        let random_d = random(0.0, defending_team_skill as f64);

        if random_a > random_d {
            attacking_team
        } else {
            defending_team
        }
    }

    fn get_team_for_squad<'a>(&self, squad: &'a Squad) -> MatchTeam<'a> {
        let mut team = MatchTeam::new(squad.team_id, squad);

        for player in squad.main_squad.iter().map(|p| &p.player) {
            for position in &player.positions() {
                match position {
                    PlayerPositionType::Goalkeeper => {
                        team.goalkeeping_skill += player.get_skill() as f32;
                    }
                    PlayerPositionType::Sweeper
                    | PlayerPositionType::DefenderLeft
                    | PlayerPositionType::DefenderCenter
                    | PlayerPositionType::DefenderRight => {
                        team.defender_skill += player.get_skill() as f32;
                    }
                    PlayerPositionType::MidfielderLeft
                    | PlayerPositionType::MidfielderCenter
                    | PlayerPositionType::MidfielderRight => {
                        team.defender_skill += 0.5 * player.get_skill() as f32;
                        team.midfielder_skill += player.get_skill() as f32;
                        team.striker_skill += 0.5 * player.get_skill() as f32;
                    }
                    PlayerPositionType::WingbackLeft
                    | PlayerPositionType::Striker
                    | PlayerPositionType::WingbackRight => {
                        team.striker_skill += player.get_skill() as f32;
                    }
                    _ => {}
                }
            }
        }

        team
    }
}

struct MatchTeam<'s> {
    pub id: u32,

    pub goalkeeping_skill: f32,
    pub defender_skill: f32,
    pub midfielder_skill: f32,
    pub striker_skill: f32,

    pub squad: &'s Squad<'s>,
}

impl<'s> MatchTeam<'s> {
    pub fn new(id: u32, squad: &'s Squad<'s>) -> Self {
        MatchTeam {
            id,
            goalkeeping_skill: 0.0,
            defender_skill: 0.0,
            midfielder_skill: 0.0,
            striker_skill: 0.0,
            squad,
        }
    }
}

pub struct FootballMatchDetails {
    pub score: Score,
    pub events: Vec<MatchEvent>,
    pub player_changes: Vec<PlayerChanges>,
}

pub struct Score {
    pub home: i32,
    pub away: i32,
}

#[derive(Debug, PartialEq)]
pub enum MatchFieldZone {
    AGoal,
    AField,
    Midfield,
    BField,
    BGoal,
}

pub struct PlayerChanges {}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}
