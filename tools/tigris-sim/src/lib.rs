use std::collections::HashMap;

use muddle_core::{
    MuddleCommand, MuddleCommandHint, MuddleCommandOutcome, MuddleError, MuddleExit, MuddleHost,
    MuddleInventoryItem, MuddleResource, MuddleRoom, MuddleVisualNode,
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
    pub persona_chosen: bool,
    pub stake_tokens: u32,
    pub collision_markers: u32,
    pub defended_marks: u32,
    pub adopted_axes: u32,
    pub amendment_scored: bool,
    pub parliament_closed: bool,
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
                room_id: "table",
                command: "choose persona",
                description: "Seat the human designer before drafting.",
            },
            TigrisMuddleCommand {
                room_id: "board",
                command: "draft axis",
                description: "Draft Tension Budget for the human chair.",
            },
            TigrisMuddleCommand {
                room_id: "board",
                command: "stake claim",
                description: "Commit stake tokens to the drafted axis.",
            },
            TigrisMuddleCommand {
                room_id: "board",
                command: "reveal collision",
                description: "Reveal an adjacency collision against the AI axis.",
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
            TigrisMuddleCommand {
                room_id: "score",
                command: "score amendment",
                description: "Score raw points and mark adopted axes.",
            },
            TigrisMuddleCommand {
                room_id: "score",
                command: "close parliament",
                description: "Close the table after amendment scoring.",
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
                persona_chosen: false,
                stake_tokens: 0,
                collision_markers: 0,
                defended_marks: 0,
                adopted_axes: 0,
                amendment_scored: false,
                parliament_closed: false,
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
            "{}\n| tigris: round={} human={} ai={} pressure={} persona={} stakes={} collisions={} defended={} adopted={} amendment={} closed={} human_axis={} ai_axis={}",
            room.ascii_card(),
            self.state.round,
            self.state.human_score,
            self.state.ai_score,
            self.state.ai_pressure,
            self.state.persona_chosen,
            self.state.stake_tokens,
            self.state.collision_markers,
            self.state.defended_marks,
            self.state.adopted_axes,
            self.state.amendment_scored,
            self.state.parliament_closed,
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
            MuddleResource {
                label: "stakes".to_string(),
                value: self.state.stake_tokens.to_string(),
            },
            MuddleResource {
                label: "collisions".to_string(),
                value: self.state.collision_markers.to_string(),
            },
            MuddleResource {
                label: "adopted".to_string(),
                value: self.state.adopted_axes.to_string(),
            },
        ]
    }

    fn inventory_panel(&self) -> Vec<MuddleInventoryItem> {
        vec![
            MuddleInventoryItem {
                label: "persona".to_string(),
                detail: if self.state.persona_chosen {
                    "Knizia seat".to_string()
                } else {
                    "unseated".to_string()
                },
            },
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
                label: "amendment".to_string(),
                detail: if self.state.parliament_closed {
                    "closed".to_string()
                } else if self.state.amendment_scored {
                    "scored".to_string()
                } else {
                    "pending".to_string()
                },
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
            "table" if !self.state.persona_chosen => {
                vec!["Choose a designer persona, then go board.".to_string()]
            }
            "table" => vec!["Go board to start the solo AI-opponent loop.".to_string()],
            "board" => vec![
                "Draft an axis, stake a claim, reveal collision, place a tiger, then end turn."
                    .to_string(),
                "Challenge the AI once pressure reaches 3.".to_string(),
            ],
            "score" if !self.state.amendment_scored => {
                vec!["Inspect AI, then score amendment to classify adoption.".to_string()]
            }
            "score" if !self.state.parliament_closed => {
                vec!["Close parliament after amendment scoring.".to_string()]
            }
            "score" => vec!["Parliament closed; reset or review transcript.".to_string()],
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

    fn visual_nodes(&self, current_room: &str) -> Vec<MuddleVisualNode> {
        let pressure_frame = if self.state.ai_pressure >= 3 {
            "armed"
        } else {
            "building"
        };
        let mut children = vec![
            MuddleVisualNode::sprite(
                "parliament-board-map",
                "Parliament board",
                "sprites/tigris/parliament-board.png",
                "A table-board-score strip for the Parliament AI slice.",
            )
            .with_layer(0)
            .with_rect(0, 0, 8, 4),
            MuddleVisualNode::text("current-room-label", "Current room", current_room)
                .with_layer(30)
                .with_rect(1, 0, 4, 1),
            tigris_room_token("table-token", "Table", "table", current_room, 1),
            tigris_room_token("board-token", "Board", "board", current_room, 2),
            tigris_room_token("score-token", "Score", "score", current_room, 3),
            MuddleVisualNode::sprite(
                "ai-pressure-badge",
                "AI pressure",
                "sprites/tigris/ai-pressure.png",
                format!("AI pressure {}", self.state.ai_pressure),
            )
            .with_layer(20)
            .with_rect(1, 5, 2, 1)
            .with_frame(pressure_frame),
            MuddleVisualNode::text(
                "score-state-label",
                "Score state",
                format!("Human {} / AI {}", self.state.human_score, self.state.ai_score),
            )
            .with_layer(30)
            .with_rect(3, 5, 4, 1),
        ];

        if self.state.amendment_scored {
            children.push(
                MuddleVisualNode::sprite(
                    "amendment-scored-badge",
                    "Amendment scored",
                    "sprites/tigris/amendment.png",
                    "Parliament amendment scored badge.",
                )
                .with_layer(20)
                .with_rect(5, 5, 2, 1)
                .with_frame("scored"),
            );
        }
        if self.state.parliament_closed {
            children.push(
                MuddleVisualNode::sprite(
                    "parliament-closed-badge",
                    "Parliament closed",
                    "sprites/tigris/closed-parliament.png",
                    "Closed Parliament badge.",
                )
                .with_layer(20)
                .with_rect(7, 5, 2, 1)
                .with_frame("closed")
                .with_animation("pulse"),
            );
        }

        vec![MuddleVisualNode::group(
            "tigris-parliament-scene",
            "TIGRIS Parliament scene",
            children,
        )]
    }

    fn export_checkpoint(&self) -> Option<String> {
        Some(format!(
            "round={};human_score={};ai_score={};ai_pressure={};persona_chosen={};stake_tokens={};collision_markers={};defended_marks={};adopted_axes={};amendment_scored={};parliament_closed={};tiger_marker={};human_axis={};ai_axis={};last_ai_move={}",
            self.state.round,
            self.state.human_score,
            self.state.ai_score,
            self.state.ai_pressure,
            self.state.persona_chosen,
            self.state.stake_tokens,
            self.state.collision_markers,
            self.state.defended_marks,
            self.state.adopted_axes,
            self.state.amendment_scored,
            self.state.parliament_closed,
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
        let mut persona_chosen = None;
        let mut stake_tokens = None;
        let mut collision_markers = None;
        let mut defended_marks = None;
        let mut adopted_axes = None;
        let mut amendment_scored = None;
        let mut parliament_closed = None;
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
                "persona_chosen" => persona_chosen = Some(parse_checkpoint_bool(key, value)?),
                "stake_tokens" => stake_tokens = Some(parse_checkpoint_u32(key, value)?),
                "collision_markers" => {
                    collision_markers = Some(parse_checkpoint_u32(key, value)?);
                }
                "defended_marks" => defended_marks = Some(parse_checkpoint_u32(key, value)?),
                "adopted_axes" => adopted_axes = Some(parse_checkpoint_u32(key, value)?),
                "amendment_scored" => amendment_scored = Some(parse_checkpoint_bool(key, value)?),
                "parliament_closed" => parliament_closed = Some(parse_checkpoint_bool(key, value)?),
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
            persona_chosen: persona_chosen.unwrap_or(false),
            stake_tokens: stake_tokens.unwrap_or(0),
            collision_markers: collision_markers.unwrap_or(0),
            defended_marks: defended_marks.unwrap_or(0),
            adopted_axes: adopted_axes.unwrap_or(0),
            amendment_scored: amendment_scored.unwrap_or(false),
            parliament_closed: parliament_closed.unwrap_or(false),
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
            ("table", "choose persona") => {
                self.state.persona_chosen = true;
                Ok(MuddleCommandOutcome::stay(
                    "Human chair takes the Knizia persona board and preferred-axis card.",
                ))
            }
            ("table", "go board") => Ok(MuddleCommandOutcome::move_to(
                "You move to the Parliament axis board.",
                "board",
            )),
            ("table", "go score") => Ok(MuddleCommandOutcome::stay(
                "The score ledger opens after the board state is established. Go board first.",
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
                if !self.state.persona_chosen {
                    self.state.persona_chosen = true;
                }
                self.state.human_axis = Some("Tension Budget".to_string());
                Ok(MuddleCommandOutcome::stay(
                    "Human chair drafts Tension Budget. AI pressure remains visible.",
                ))
            }
            ("board", "stake claim") if self.state.human_axis.is_none() => Ok(
                MuddleCommandOutcome::stay("Draft an axis before committing stake tokens."),
            ),
            ("board", "stake claim") => {
                self.state.stake_tokens = 2;
                self.state.human_score = self.state.scores.add("human", 1);
                Ok(MuddleCommandOutcome::stay(
                    "Human chair stakes 2 tokens on Tension Budget and earns an opening point.",
                ))
            }
            ("board", "reveal collision") if self.state.stake_tokens == 0 => Ok(
                MuddleCommandOutcome::stay("Commit a stake before checking for axis collisions."),
            ),
            ("board", "reveal collision") => {
                self.state.collision_markers += 1;
                self.state.defended_marks += 1;
                self.state.tokens.gain("ai_pressure", 1);
                self.state.ai_pressure = self.state.tokens.count("ai_pressure") as u32;
                Ok(MuddleCommandOutcome::stay(
                    "Tension Budget collides with the AI's Scarcity Bite lane. Defense credit recorded; pressure rises.",
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
            ("score", "score amendment") if self.state.amendment_scored => {
                Ok(MuddleCommandOutcome::stay(
                    "Amendment already scored. Close parliament when ready.",
                ))
            }
            ("score", "score amendment") => {
                if self.state.defended_marks > 0 && self.state.stake_tokens > 0 {
                    self.state.adopted_axes = 1;
                    self.state.human_score = self.state.scores.add("human", 3);
                }
                self.state.amendment_scored = true;
                Ok(MuddleCommandOutcome::stay(format!(
                    "Amendment scored: {} adopted axes, {} collision markers, human {} to AI {}.",
                    self.state.adopted_axes,
                    self.state.collision_markers,
                    self.state.human_score,
                    self.state.ai_score
                )))
            }
            ("score", "close parliament") if !self.state.amendment_scored => Ok(
                MuddleCommandOutcome::stay("Score amendment before closing Parliament."),
            ),
            ("score", "close parliament") => {
                self.state.parliament_closed = true;
                Ok(MuddleCommandOutcome::stay(
                    "Parliament closes. The ledger records Tension Budget as adopted for the next session.",
                ))
            }
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

fn parse_checkpoint_bool(key: &str, value: &str) -> Result<bool, MuddleError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(MuddleError::InvalidHostCheckpoint {
            message: format!("invalid boolean checkpoint field `{key}={value}`"),
        }),
    }
}

fn tigris_room_token(
    id: &str,
    label: &str,
    room_id: &str,
    current_room: &str,
    order: i32,
) -> MuddleVisualNode {
    let frame = if current_room == room_id {
        "active"
    } else {
        "idle"
    };
    MuddleVisualNode::sprite(
        id,
        label,
        format!("sprites/tigris/{room_id}.png"),
        format!("{label} room token"),
    )
    .with_layer(10)
    .with_rect(order * 2 - 1, 2, 1, 1)
    .with_frame(frame)
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
    fn ai_opponent_guides_friction_commands() {
        let mut host = parliament_ai_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");
        for command in [
            "go score",
            "choose persona",
            "go board",
            "draft axis",
            "stake claim",
            "reveal collision",
            "place tiger",
            "end turn",
            "challenge ai",
            "go score",
            "score amendment",
            "close parliament",
        ] {
            session
                .play_turn(&mut host, MuddleCommand::parse(command))
                .expect("friction command remains guided");
        }

        assert_eq!(session.current_room, "score");
        assert!(host.state().parliament_closed);
    }

    #[test]
    fn ai_opponent_emits_visual_scene_nodes() {
        let mut host = parliament_ai_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");
        for command in [
            "choose persona",
            "go board",
            "draft axis",
            "stake claim",
            "reveal collision",
            "place tiger",
            "end turn",
            "challenge ai",
            "go score",
            "score amendment",
            "close parliament",
        ] {
            session
                .play_turn(&mut host, MuddleCommand::parse(command))
                .expect("command plays");
        }

        let visuals = host.visual_nodes(&session.current_room);
        let scene = visuals
            .iter()
            .find(|node| node.id == "tigris-parliament-scene")
            .expect("scene group exists");
        assert!(scene.children.iter().any(|node| node.id == "score-token"));
        assert!(scene
            .children
            .iter()
            .any(|node| node.id == "parliament-closed-badge"));
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
