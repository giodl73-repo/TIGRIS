use std::collections::HashMap;

use muddle_core::{
    MuddleCommand, MuddleCommandHint, MuddleCommandOutcome, MuddleError, MuddleExit, MuddleHost,
    MuddleInventoryItem, MuddleResource, MuddleRoom,
};
use rally_core::{ScoreTrack, TokenPool, TurnOrder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TigrisMuddleSurface {
    pub host_name: &'static str,
    pub title: &'static str,
    pub start_room: &'static str,
    pub rooms: Vec<TigrisMuddleRoom>,
    pub commands: Vec<TigrisMuddleCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TigrisMuddleRoom {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub exits: Vec<TigrisMuddleExit>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TigrisMuddleExit {
    pub command: &'static str,
    pub target_room: &'static str,
    pub label: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TigrisMuddleCommand {
    pub room_id: &'static str,
    pub command: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TigrisAiOpponentMuddleHost {
    rooms: HashMap<String, MuddleRoom>,
    commands: Vec<TigrisMuddleCommand>,
    state: TigrisAiOpponentState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TigrisAiOpponentState {
    pub round: u32,
    pub human_score: i32,
    pub ai_score: i32,
    pub ai_pressure: u32,
    pub turn_order: TurnOrder,
    pub scores: ScoreTrack,
    pub tokens: TokenPool,
    pub human_axis: Option<String>,
    pub ai_axis: Option<String>,
    pub last_ai_move: String,
}

pub fn parliament_ai_muddle_surface() -> TigrisMuddleSurface {
    TigrisMuddleSurface {
        host_name: "tigris-parliament-ai",
        title: "TIGRIS Parliament AI Table",
        start_room: "table",
        rooms: vec![
            TigrisMuddleRoom {
                id: "table",
                title: "Parliament Table",
                description: "A solo Parliament table with one human chair and a deterministic AI opponent.",
                exits: vec![TigrisMuddleExit {
                    command: "go board",
                    target_room: "board",
                    label: "Axis Board",
                }],
            },
            TigrisMuddleRoom {
                id: "board",
                title: "Axis Board",
                description: "Draft axes, place tiger markers, challenge the AI, then end turn to let the AI respond.",
                exits: vec![
                    TigrisMuddleExit {
                        command: "go table",
                        target_room: "table",
                        label: "Parliament Table",
                    },
                    TigrisMuddleExit {
                        command: "go score",
                        target_room: "score",
                        label: "Score Ledger",
                    },
                ],
            },
            TigrisMuddleRoom {
                id: "score",
                title: "Score Ledger",
                description: "Review adoption pressure, AI counterplay, and the current solo-play score.",
                exits: vec![TigrisMuddleExit {
                    command: "go board",
                    target_room: "board",
                    label: "Axis Board",
                }],
            },
        ],
        commands: vec![
            TigrisMuddleCommand {
                room_id: "table",
                command: "status",
                description: "Show the current solo table state.",
            },
            TigrisMuddleCommand {
                room_id: "board",
                command: "draft axis",
                description: "Draft Tension Budget for the human chair.",
            },
            TigrisMuddleCommand {
                room_id: "board",
                command: "place tiger",
                description: "Commit a tiger marker for one point.",
            },
            TigrisMuddleCommand {
                room_id: "board",
                command: "challenge ai",
                description: "Challenge the AI axis if pressure is high.",
            },
            TigrisMuddleCommand {
                room_id: "board",
                command: "end turn",
                description: "Advance to the deterministic AI opponent turn.",
            },
            TigrisMuddleCommand {
                room_id: "score",
                command: "inspect ai",
                description: "Inspect the AI opponent plan.",
            },
        ],
    }
}

pub fn parliament_ai_muddle_host() -> TigrisAiOpponentMuddleHost {
    TigrisAiOpponentMuddleHost::new(parliament_ai_muddle_surface())
}

impl TigrisAiOpponentMuddleHost {
    pub fn new(surface: TigrisMuddleSurface) -> Self {
        let rooms = surface
            .rooms
            .into_iter()
            .map(|room| {
                (
                    room.id.to_string(),
                    MuddleRoom {
                        id: room.id.to_string(),
                        title: room.title.to_string(),
                        description: room.description.to_string(),
                        exits: room
                            .exits
                            .into_iter()
                            .map(|exit| MuddleExit {
                                command: exit.command.to_string(),
                                target_room: exit.target_room.to_string(),
                                label: exit.label.to_string(),
                            })
                            .collect(),
                    },
                )
            })
            .collect();

        Self {
            rooms,
            commands: surface.commands,
            state: TigrisAiOpponentState {
                round: 1,
                human_score: 0,
                ai_score: 0,
                ai_pressure: 1,
                turn_order: TurnOrder::new(["human", "ai"]),
                scores: ScoreTrack::new(["human", "ai"]),
                tokens: TokenPool::new([("ai_pressure", 1), ("tiger_marker", 0)]),
                human_axis: None,
                ai_axis: None,
                last_ai_move: "AI waits for the opening draft.".to_string(),
            },
        }
    }

    pub fn state(&self) -> &TigrisAiOpponentState {
        &self.state
    }

    fn look(&self, room_id: &str) -> Result<MuddleCommandOutcome, MuddleError> {
        let room = self
            .room(room_id)
            .ok_or_else(|| MuddleError::RoomNotFound {
                room_id: room_id.to_string(),
            })?;
        Ok(MuddleCommandOutcome::stay(format!(
            "{}\n| tigris: round={} human={} ai={} pressure={} human_axis={} ai_axis={}",
            room.ascii_card(),
            self.state.round,
            self.state.human_score,
            self.state.ai_score,
            self.state.ai_pressure,
            self.state.human_axis.as_deref().unwrap_or("none"),
            self.state.ai_axis.as_deref().unwrap_or("none")
        )))
    }

    fn ai_turn(&mut self) -> String {
        self.state.turn_order.advance();
        self.state.turn_order.advance();
        self.state.round = self.state.turn_order.round();
        if self.state.human_axis.is_none() {
            self.state.ai_axis = Some("Elegance".to_string());
            self.state.ai_score = self.state.scores.add("ai", 1);
            self.state.tokens.gain("ai_pressure", 1);
            self.state.ai_pressure = self.state.tokens.count("ai_pressure") as u32;
            self.state.last_ai_move =
                "AI drafted Elegance because the human chair left the axis pool open.".to_string();
            return self.state.last_ai_move.clone();
        }

        if self.state.ai_pressure >= 3 {
            self.state.ai_axis = Some("Anti-Catch-up Pressure".to_string());
            self.state.ai_score = self.state.scores.add("ai", 2);
            self.state.tokens.spend(
                "ai_pressure",
                self.state.tokens.count("ai_pressure").saturating_sub(1),
            );
            self.state.ai_pressure = self.state.tokens.count("ai_pressure") as u32;
            self.state.last_ai_move =
                "AI counter-drafted Anti-Catch-up Pressure and converted stored pressure."
                    .to_string();
            return self.state.last_ai_move.clone();
        }

        self.state.ai_axis = Some("Scarcity Bite".to_string());
        self.state.ai_score = self.state.scores.add("ai", 1);
        self.state.tokens.gain("ai_pressure", 1);
        self.state.ai_pressure = self.state.tokens.count("ai_pressure") as u32;
        self.state.last_ai_move =
            "AI drafted Scarcity Bite to threaten the next collision window.".to_string();
        self.state.last_ai_move.clone()
    }
}

impl MuddleHost for TigrisAiOpponentMuddleHost {
    fn start_room(&self) -> &str {
        "table"
    }

    fn room(&self, room_id: &str) -> Option<&MuddleRoom> {
        self.rooms.get(room_id)
    }

    fn resource_panel(&self) -> Vec<MuddleResource> {
        vec![
            MuddleResource {
                label: "round".to_string(),
                value: self.state.round.to_string(),
            },
            MuddleResource {
                label: "human".to_string(),
                value: self.state.human_score.to_string(),
            },
            MuddleResource {
                label: "ai".to_string(),
                value: self.state.ai_score.to_string(),
            },
            MuddleResource {
                label: "pressure".to_string(),
                value: self.state.ai_pressure.to_string(),
            },
        ]
    }

    fn inventory_panel(&self) -> Vec<MuddleInventoryItem> {
        vec![
            MuddleInventoryItem {
                label: "human tiger".to_string(),
                detail: self
                    .state
                    .human_axis
                    .as_deref()
                    .unwrap_or("unplaced")
                    .to_string(),
            },
            MuddleInventoryItem {
                label: "ai plan".to_string(),
                detail: self.state.last_ai_move.clone(),
            },
            MuddleInventoryItem {
                label: "collision marker".to_string(),
                detail: if self.state.ai_pressure >= 3 {
                    "armed".to_string()
                } else {
                    "building".to_string()
                },
            },
        ]
    }

    fn map_panel(&self, current_room: &str) -> Option<String> {
        Some(format!(
            "[table] -> [board] -> [score] | current={current_room}"
        ))
    }

    fn objective_panel(&self, current_room: &str) -> Vec<String> {
        match current_room {
            "table" => vec!["Go board to start the solo AI-opponent loop.".to_string()],
            "board" => vec![
                "Draft an axis, place a tiger marker, then end turn.".to_string(),
                "Challenge the AI once pressure reaches 3.".to_string(),
            ],
            "score" => vec!["Inspect AI to understand deterministic counterplay.".to_string()],
            _ => Vec::new(),
        }
    }

    fn command_panel(&self, current_room: &str) -> Vec<MuddleCommandHint> {
        self.commands
            .iter()
            .filter(|command| command.room_id == current_room)
            .map(|command| MuddleCommandHint {
                command: command.command.to_string(),
                description: command.description.to_string(),
            })
            .collect()
    }

    fn export_checkpoint(&self) -> Option<String> {
        Some(format!(
            "round={};human_score={};ai_score={};ai_pressure={};tiger_marker={};human_axis={};ai_axis={};last_ai_move={}",
            self.state.round,
            self.state.human_score,
            self.state.ai_score,
            self.state.ai_pressure,
            self.state.tokens.count("tiger_marker"),
            self.state.human_axis.as_deref().unwrap_or("none"),
            self.state.ai_axis.as_deref().unwrap_or("none"),
            self.state.last_ai_move
        ))
    }

    fn import_checkpoint(&mut self, checkpoint: &str) -> Result<(), MuddleError> {
        let mut round = None;
        let mut human_score = None;
        let mut ai_score = None;
        let mut ai_pressure = None;
        let mut tiger_marker = None;
        let mut human_axis = None;
        let mut ai_axis = None;
        let mut last_ai_move = None;

        for part in checkpoint.split(';') {
            let (key, value) =
                part.split_once('=')
                    .ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                        message: format!("malformed checkpoint field `{part}`"),
                    })?;
            match key {
                "round" => round = Some(parse_checkpoint_u32(key, value)?),
                "human_score" => human_score = Some(parse_checkpoint_i32(key, value)?),
                "ai_score" => ai_score = Some(parse_checkpoint_i32(key, value)?),
                "ai_pressure" => ai_pressure = Some(parse_checkpoint_u32(key, value)?),
                "tiger_marker" => tiger_marker = Some(parse_checkpoint_i32(key, value)?),
                "human_axis" => human_axis = Some(parse_checkpoint_option(value)),
                "ai_axis" => ai_axis = Some(parse_checkpoint_option(value)),
                "last_ai_move" => last_ai_move = Some(value.to_string()),
                _ => {
                    return Err(MuddleError::InvalidHostCheckpoint {
                        message: format!("unknown checkpoint field `{key}`"),
                    });
                }
            }
        }

        let round = round.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
            message: "missing round checkpoint field".to_string(),
        })?;
        let human_score = human_score.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
            message: "missing human_score checkpoint field".to_string(),
        })?;
        let ai_score = ai_score.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
            message: "missing ai_score checkpoint field".to_string(),
        })?;
        let ai_pressure = ai_pressure.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
            message: "missing ai_pressure checkpoint field".to_string(),
        })?;
        let tiger_marker = tiger_marker.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
            message: "missing tiger_marker checkpoint field".to_string(),
        })?;

        let mut turn_order = TurnOrder::new(["human", "ai"]);
        for _ in 1..round {
            turn_order.advance();
            turn_order.advance();
        }
        let mut scores = ScoreTrack::new(["human", "ai"]);
        scores.add("human", human_score);
        scores.add("ai", ai_score);

        self.state = TigrisAiOpponentState {
            round,
            human_score,
            ai_score,
            ai_pressure,
            turn_order,
            scores,
            tokens: TokenPool::new([
                ("ai_pressure", ai_pressure as i32),
                ("tiger_marker", tiger_marker),
            ]),
            human_axis: human_axis.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing human_axis checkpoint field".to_string(),
            })?,
            ai_axis: ai_axis.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing ai_axis checkpoint field".to_string(),
            })?,
            last_ai_move: last_ai_move.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing last_ai_move checkpoint field".to_string(),
            })?,
        };
        Ok(())
    }

    fn handle_command(
        &mut self,
        room_id: &str,
        command: &MuddleCommand,
    ) -> Result<MuddleCommandOutcome, MuddleError> {
        let normalized = command.normalized();
        if normalized == "look" || normalized == "status" {
            return self.look(room_id);
        }

        match (room_id, normalized.as_str()) {
            ("table", "go board") => Ok(MuddleCommandOutcome::move_to(
                "You move to the Parliament axis board.",
                "board",
            )),
            ("board", "go table") => Ok(MuddleCommandOutcome::move_to(
                "You return to the Parliament table.",
                "table",
            )),
            ("board", "go score") => Ok(MuddleCommandOutcome::move_to(
                "You open the Parliament score ledger.",
                "score",
            )),
            ("score", "go board") => Ok(MuddleCommandOutcome::move_to(
                "You return to the axis board.",
                "board",
            )),
            ("board", "draft axis") => {
                self.state.human_axis = Some("Tension Budget".to_string());
                Ok(MuddleCommandOutcome::stay(
                    "Human chair drafts Tension Budget. AI pressure remains visible.",
                ))
            }
            ("board", "place tiger") => {
                self.state.tokens.gain("tiger_marker", 1);
                self.state.human_score = self.state.scores.add("human", 1);
                Ok(MuddleCommandOutcome::stay(format!(
                    "Tiger marker placed on {}. Human gains 1 point.",
                    self.state
                        .human_axis
                        .as_deref()
                        .unwrap_or("an undrafted axis")
                )))
            }
            ("board", "challenge ai") => {
                if self.state.ai_pressure >= 3 {
                    self.state.human_score = self.state.scores.add("human", 2);
                    self.state.tokens.spend(
                        "ai_pressure",
                        self.state.tokens.count("ai_pressure").saturating_sub(1),
                    );
                    self.state.ai_pressure = self.state.tokens.count("ai_pressure") as u32;
                    Ok(MuddleCommandOutcome::stay(
                        "Challenge lands. Human scores 2 and resets AI pressure.",
                    ))
                } else {
                    self.state.ai_score = self.state.scores.add("ai", 1);
                    self.state.tokens.gain("ai_pressure", 1);
                    self.state.ai_pressure = self.state.tokens.count("ai_pressure") as u32;
                    Ok(MuddleCommandOutcome::stay(
                        "Challenge is early. AI gains 1 and pressure rises.",
                    ))
                }
            }
            ("board", "end turn") => {
                let ai_move = self.ai_turn();
                Ok(MuddleCommandOutcome::stay(format!("End turn. {ai_move}")))
            }
            ("score", "inspect ai") => Ok(MuddleCommandOutcome::stay(format!(
                "AI opponent: deterministic pressure bot. Last move: {}",
                self.state.last_ai_move
            ))),
            _ => Err(MuddleError::UnknownCommand {
                room_id: room_id.to_string(),
                command: command.clone(),
            }),
        }
    }
}

fn parse_checkpoint_option(value: &str) -> Option<String> {
    if value == "none" {
        None
    } else {
        Some(value.to_string())
    }
}

fn parse_checkpoint_i32(key: &str, value: &str) -> Result<i32, MuddleError> {
    value
        .parse::<i32>()
        .map_err(|_| MuddleError::InvalidHostCheckpoint {
            message: format!("invalid integer checkpoint field `{key}={value}`"),
        })
}

fn parse_checkpoint_u32(key: &str, value: &str) -> Result<u32, MuddleError> {
    value
        .parse::<u32>()
        .map_err(|_| MuddleError::InvalidHostCheckpoint {
            message: format!("invalid unsigned checkpoint field `{key}={value}`"),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use muddle_core::MuddleSession;

    #[test]
    fn ai_opponent_advances_after_end_turn() {
        let mut host = parliament_ai_muddle_host();

        host.handle_command("board", &MuddleCommand::parse("draft axis"))
            .expect("draft succeeds");
        host.handle_command("board", &MuddleCommand::parse("place tiger"))
            .expect("place succeeds");
        let outcome = host
            .handle_command("board", &MuddleCommand::parse("end turn"))
            .expect("ai turn succeeds");

        assert!(outcome.response.contains("AI drafted Scarcity Bite"));
        assert_eq!(host.state().round, 2);
        assert_eq!(host.state().human_score, 1);
        assert_eq!(host.state().ai_score, 1);
    }

    #[test]
    fn table_exit_moves_to_board() {
        let mut host = parliament_ai_muddle_host();

        let outcome = host
            .handle_command("table", &MuddleCommand::parse("go board"))
            .expect("exit succeeds");

        assert_eq!(outcome.next_room, Some("board".to_string()));
    }

    #[test]
    fn challenge_can_reset_armed_ai_pressure() {
        let mut host = parliament_ai_muddle_host();

        host.handle_command("board", &MuddleCommand::parse("draft axis"))
            .expect("draft succeeds");
        host.handle_command("board", &MuddleCommand::parse("end turn"))
            .expect("first ai turn succeeds");
        host.handle_command("board", &MuddleCommand::parse("end turn"))
            .expect("second ai turn succeeds");
        let outcome = host
            .handle_command("board", &MuddleCommand::parse("challenge ai"))
            .expect("challenge succeeds");

        assert!(outcome.response.contains("Challenge lands"));
        assert_eq!(host.state().ai_pressure, 1);
        assert_eq!(host.state().human_score, 2);
    }

    #[test]
    fn ai_opponent_resumes_from_checkpoint_save() {
        let mut host = parliament_ai_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");
        for command in ["go board", "draft axis", "place tiger", "end turn"] {
            session
                .play_turn(&mut host, MuddleCommand::parse(command))
                .expect("command plays");
        }

        let save = session.save_for_host(&host);
        assert!(save
            .host_checkpoint
            .as_deref()
            .unwrap_or_default()
            .contains("human_axis=Tension Budget"));

        let checkpoint_only_save = muddle_core::MuddleSessionSave {
            current_room: "board".to_string(),
            commands: vec!["go board".to_string()],
            host_checkpoint: save.host_checkpoint,
        };
        let mut resumed_host = parliament_ai_muddle_host();
        let mut resumed = MuddleSession::resume_for_host(&mut resumed_host, &checkpoint_only_save)
            .expect("session resumes from host checkpoint");
        resumed
            .play_turn(&mut resumed_host, MuddleCommand::parse("challenge ai"))
            .expect("checkpoint restored AI pressure");

        assert_eq!(resumed.current_room, "board");
        assert_eq!(
            resumed_host.state().human_axis.as_deref(),
            Some("Tension Budget")
        );
        assert!(resumed_host.state().human_score >= 1);
        assert!(resumed_host.state().ai_score >= 1);
    }
}
