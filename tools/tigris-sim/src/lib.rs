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
                description: "A parchment-and-ink argument table: seat a designer persona, read the Axis Pool, then carry the claim to the board.",
                exits: vec![TigrisMuddleExit {
                    command: "go board",
                    target_room: "board",
                    label: "Axis Board",
                }],
            },
            TigrisMuddleRoom {
                id: "board",
                title: "Axis Board",
                description: "A live disagreement board where axis cards, stake bowls, collision lane, AI pressure, and tiger markers make the argument visible.",
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
                description: "A rubric ledger for turning defended stakes and collisions into amendment evidence for the next Parliament session.",
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
                room_id: "table",
                command: "inspect table",
                description: "Inspect the persona mat, pool deck, and table promise.",
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
                room_id: "board",
                command: "inspect board",
                description: "Inspect the axis board, collision lane, and pressure dial.",
            },
            TigrisMuddleCommand {
                room_id: "board",
                command: "inspect collision",
                description: "Inspect why the current axis collision matters.",
            },
            TigrisMuddleCommand {
                room_id: "score",
                command: "inspect ai",
                description: "Inspect the AI opponent plan.",
            },
            TigrisMuddleCommand {
                room_id: "score",
                command: "inspect ledger",
                description: "Inspect the score ledger and amendment stickers.",
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
            "{}\n| parliament read: {}\n| next action: {}\n| tigris: phase={} round={} human={} ai={} pressure={} persona={} stakes={} collisions={} defended={} adopted={} amendment={} closed={} human_axis={} ai_axis={}",
            room.ascii_card(),
            self.table_read(room_id),
            self.next_action(room_id),
            self.phase_label(),
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

    fn phase_label(&self) -> &'static str {
        if self.state.parliament_closed {
            "closed"
        } else if self.state.amendment_scored {
            "closing"
        } else if self.state.human_axis.is_some() || self.state.collision_markers > 0 {
            "argument"
        } else if self.state.persona_chosen {
            "draft"
        } else {
            "seating"
        }
    }

    fn table_read(&self, room_id: &str) -> &'static str {
        match room_id {
            "table" if self.state.persona_chosen => {
                "The Knizia mat is no longer decoration; it tells the player to win by balanced proof."
            }
            "table" => {
                "The empty persona mat, Axis Pool deck, and AI chair explain that every claim needs a point of view."
            }
            "board" if self.state.collision_markers > 0 => {
                "The red collision lane is the table's drama: disagreement is now evidence, not flavor."
            }
            "board" if self.state.stake_tokens > 0 => {
                "The stake bowl is gold, so the axis is no longer just named; it has something at risk."
            }
            "board" => {
                "The board waits for an axis card to become a defended argument under AI pressure."
            }
            "score" if self.state.parliament_closed => {
                "The close badge and adoption sticker show the factory moving forward without silent changes."
            }
            "score" if self.state.amendment_scored => {
                "The ledger is scored but not closed; the table still needs the final governance act."
            }
            "score" => {
                "The ledger is blank until defended marks and collisions explain why the rubric should change."
            }
            _ => "Parliament is a table where design claims must become visible evidence.",
        }
    }

    fn next_action(&self, room_id: &str) -> &'static str {
        match room_id {
            "table" if !self.state.persona_chosen => "choose persona",
            "table" => "go board",
            "board" if self.state.human_axis.is_none() => "draft axis",
            "board" if self.state.stake_tokens == 0 => "stake claim",
            "board" if self.state.collision_markers == 0 => "reveal collision",
            "board" if self.state.tokens.count("tiger_marker") == 0 => "place tiger",
            "board" if self.state.round == 1 => "end turn",
            "board" if self.state.ai_pressure >= 3 => "challenge ai",
            "board" => "go score",
            "score" if !self.state.amendment_scored => "score amendment",
            "score" if !self.state.parliament_closed => "close parliament",
            "score" => "review transcript",
            _ => "look",
        }
    }

    fn subject_card(&self) -> &'static str {
        match self.phase_label() {
            "seating" => "Subject card: a mid-weight Euro about arguing over game taste waits face down.",
            "draft" => "Subject card: choose which axis will be accountable before the AI fills the vacuum.",
            "argument" => "Subject card: Tension Budget vs Scarcity Bite is live on the table.",
            "closing" => "Subject card: evidence has been scored; governance is waiting for closure.",
            "closed" => "Subject card: next session inherits the adopted Tension Budget record.",
            _ => "Subject card: Parliament turns table moments into rubric evidence.",
        }
    }

    fn room_detail(&self, room_id: &str) -> &'static str {
        match room_id {
            "table" => {
                "Table read: persona mat, axis pool deck, stake bowls, and the empty AI chair."
            }
            "board" => {
                "Board read: axis cards, collision lane, tiger marker, pressure dial, and dissent tags."
            }
            "score" => {
                "Ledger read: raw score track, adopted-axis sticker, closure gavel, and next-session note."
            }
            _ => "Parliament table state is ready.",
        }
    }

    fn parliament_prompt(&self, room_id: &str) -> &'static str {
        match room_id {
            "table" if self.state.persona_chosen => {
                "Read: Knizia is seated; the table now wants an axis draft."
            }
            "table" => "Read: this game starts by choosing the design lens you will defend.",
            "board" if self.state.collision_markers > 0 => {
                "Read: collision is the argument becoming visible on the board."
            }
            "board" if self.state.human_axis.is_some() => {
                "Read: Tension Budget needs stake, pressure, and a tiger marker to matter."
            }
            "board" => "Read: draft an axis before the AI turns pressure into a counterclaim.",
            "score" if self.state.parliament_closed => {
                "Read: Parliament closed with a next-session rubric change."
            }
            "score" if self.state.amendment_scored => {
                "Read: scoring only counts if the amendment record explains why."
            }
            "score" => "Read: the ledger converts table argument into forward-only evidence.",
            _ => "Read: every token should explain a design disagreement.",
        }
    }

    fn inspect(&self, room_id: &str, target: &str) -> MuddleCommandOutcome {
        let response = match (room_id, target) {
            ("table", "table") | ("table", "persona") => {
                if self.state.persona_chosen {
                    "The Knizia persona board is seated beside a minimum-score reminder: balance stake points with adoption, or lose on shape."
                } else {
                    "The empty persona mat asks who is making the argument. Without a designer lens, the axis draft is just vocabulary."
                }
            }
            ("table", "pool") | ("table", "deck") => {
                "The Axis Pool deck is the centerpiece: cards can become adopted, contested, ignored, or retired across sessions."
            }
            ("board", "board") | ("board", "axis") => {
                if self.state.human_axis.is_some() {
                    "Tension Budget is drafted into the human lane. Stake tokens decide whether it is merely named or actually defended."
                } else {
                    "The board is arranged around empty axis lanes. Draft first so the pressure dial has something to threaten."
                }
            }
            ("board", "collision") => {
                if self.state.collision_markers > 0 {
                    "The collision lane shows Tension Budget grinding against Scarcity Bite. That visible disagreement is the evidence Parliament wants."
                } else {
                    "The collision lane is quiet. Stake a claim, then reveal collision to force the axes to disagree in public."
                }
            }
            ("board", "pressure") => {
                if self.state.ai_pressure >= 3 {
                    "The AI pressure dial is red: a challenge can now turn stored pressure into human points."
                } else {
                    "The AI pressure dial is amber: it is building toward a counter-draft but not armed yet."
                }
            }
            ("score", "ledger") | ("score", "amendment") => {
                if self.state.parliament_closed {
                    "The ledger is closed with Tension Budget marked as adopted for the next session."
                } else if self.state.amendment_scored {
                    "The ledger has raw points and an adopted-axis sticker waiting for the closing gavel."
                } else {
                    "The ledger is blank until scoring translates defended marks and collisions into amendment evidence."
                }
            }
            ("score", "ai") => {
                return MuddleCommandOutcome::stay(format!(
                    "AI opponent: deterministic pressure bot. Last move: {}",
                    self.state.last_ai_move
                ));
            }
            _ => return MuddleCommandOutcome::stay(format!(
                "You inspect Parliament details. {}",
                self.room_detail(room_id)
            )),
        };
        MuddleCommandOutcome::stay(response)
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
                value: format!("{} ({})", self.state.round, self.phase_label()),
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
                value: if self.state.ai_pressure >= 3 {
                    format!("{} armed", self.state.ai_pressure)
                } else {
                    format!("{} building", self.state.ai_pressure)
                },
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
                    "Knizia seat - balance proof".to_string()
                } else {
                    "unseated - choose a lens".to_string()
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
                detail: if self.state.collision_markers > 0 {
                    "visible disagreement".to_string()
                } else if self.state.ai_pressure >= 3 {
                    "pressure armed".to_string()
                } else {
                    "pressure building".to_string()
                },
            },
        ]
    }

    fn map_panel(&self, current_room: &str) -> Option<String> {
        Some(format!(
            "[table: persona] -> [board: argument] -> [score: amendment] | current={current_room}"
        ))
    }

    fn objective_panel(&self, current_room: &str) -> Vec<String> {
        match current_room {
            "table" if !self.state.persona_chosen => {
                vec![
                    "Choose a designer persona so every later claim has a point of view."
                        .to_string(),
                    "Inspect the table if the Axis Pool and AI chair feel abstract.".to_string(),
                ]
            }
            "table" => vec![
                "Go board to turn the seated persona into a visible argument.".to_string(),
                "Watch for the minimum-score reminder: adoption without defended proof is hollow."
                    .to_string(),
            ],
            "board" => vec![
                "Draft an axis, stake a claim, reveal collision, place a tiger, then end turn."
                    .to_string(),
                "Use the collision lane and pressure dial to decide when challenge ai is worth it."
                    .to_string(),
            ],
            "score" if !self.state.amendment_scored => {
                vec![
                    "Inspect AI or ledger, then score amendment to classify adoption.".to_string(),
                    "Only scored evidence should produce a next-session rubric mark.".to_string(),
                ]
            }
            "score" if !self.state.parliament_closed => {
                vec![
                    "Close parliament after amendment scoring.".to_string(),
                    "The close badge should mean governance, not just victory text.".to_string(),
                ]
            }
            "score" => vec![
                "Parliament closed; reset or review transcript.".to_string(),
                "Next session inherits the visible Tension Budget adoption record.".to_string(),
            ],
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
            "tigris-red"
        } else {
            "tigris-gold"
        };
        let persona_frame = if self.state.persona_chosen {
            "tigris-green"
        } else {
            "tigris-parchment"
        };
        let axis_frame = if self.state.human_axis.is_some() {
            "tigris-ink"
        } else {
            "tigris-parchment"
        };
        let stake_frame = if self.state.stake_tokens > 0 {
            "tigris-gold"
        } else {
            "tigris-parchment"
        };
        let collision_frame = if self.state.collision_markers > 0 {
            "tigris-red"
        } else {
            "tigris-parchment"
        };
        let tiger_frame = if self.state.tokens.count("tiger_marker") > 0 {
            "tigris-gold"
        } else {
            "tigris-parchment"
        };
        let ledger_frame = if self.state.parliament_closed {
            "tigris-closed"
        } else if self.state.amendment_scored {
            "tigris-ledger"
        } else {
            "tigris-parchment"
        };
        let mut children = vec![
            MuddleVisualNode::sprite(
                "parliament-board-map",
                "Parliament board",
                "sprites/tigris/parliament-board.png",
                "A table-board-score strip for the Parliament AI slice with table zones and evidence lanes.",
            )
            .with_layer(0)
            .with_rect(0, 0, 12, 6)
            .with_frame("tigris-parchment"),
            MuddleVisualNode::text("current-room-label", "Current room", current_room)
                .with_layer(30)
                .with_rect(0, 0, 3, 1),
            MuddleVisualNode::text(
                "parliament-title",
                "Table thesis",
                "Parliament: argument -> evidence -> amendment",
            )
            .with_layer(30)
            .with_rect(3, 0, 6, 1),
            MuddleVisualNode::text("phase-readout", "Phase", self.phase_label())
                .with_layer(30)
                .with_rect(9, 0, 3, 1),
            MuddleVisualNode::text(
                "parliament-prompt",
                "Player read",
                self.parliament_prompt(current_room),
            )
            .with_layer(30)
            .with_rect(0, 7, 12, 1),
            tigris_room_token("table-token", "Table", "table", current_room, 1),
            tigris_room_token("board-token", "Board", "board", current_room, 2),
            tigris_room_token("score-token", "Score", "score", current_room, 3),
            MuddleVisualNode::sprite(
                "persona-mat",
                "Persona mat",
                "sprites/tigris/persona-mat.png",
                if self.state.persona_chosen {
                    "Knizia persona seated with minimum-score reminder."
                } else {
                    "Empty persona mat waiting for the human designer."
                },
            )
            .with_layer(12)
            .with_rect(0, 1, 2, 1)
            .with_frame(persona_frame),
            MuddleVisualNode::sprite(
                "axis-pool-deck",
                "Axis Pool deck",
                "sprites/tigris/axis-pool.png",
                "Oversized axis cards: the table's evolving vocabulary.",
            )
            .with_layer(12)
            .with_rect(2, 1, 2, 1)
            .with_frame("tigris-ink"),
            MuddleVisualNode::sprite(
                "subject-card",
                "Subject card",
                "sprites/tigris/subject-card.png",
                self.subject_card(),
            )
            .with_layer(13)
            .with_rect(0, 3, 2, 1)
            .with_frame("tigris-ink"),
            MuddleVisualNode::sprite(
                "argument-clock",
                "Argument clock",
                "sprites/tigris/argument-clock.png",
                format!("Next action: {}", self.next_action(current_room)),
            )
            .with_layer(13)
            .with_rect(2, 3, 2, 1)
            .with_frame(if self.state.parliament_closed {
                "tigris-closed"
            } else {
                "tigris-gold"
            }),
            MuddleVisualNode::sprite(
                "human-axis-card",
                "Human axis card",
                "sprites/tigris/tension-budget.png",
                self.state.human_axis.as_deref().unwrap_or("Undrafted axis lane."),
            )
            .with_layer(14)
            .with_rect(4, 1, 2, 1)
            .with_frame(axis_frame),
            MuddleVisualNode::sprite(
                "stake-token-bowl",
                "Stake token bowl",
                "sprites/tigris/stake-tokens.png",
                format!("{} stake tokens committed.", self.state.stake_tokens),
            )
            .with_layer(14)
            .with_rect(6, 1, 2, 1)
            .with_frame(stake_frame),
            MuddleVisualNode::sprite(
                "collision-lane",
                "Collision lane",
                "sprites/tigris/collision-lane.png",
                format!("{} visible axis collisions.", self.state.collision_markers),
            )
            .with_layer(15)
            .with_rect(4, 3, 3, 1)
            .with_frame(collision_frame),
            MuddleVisualNode::sprite(
                "tiger-marker",
                "Tiger marker",
                "sprites/tigris/tiger-marker.png",
                format!("{} tiger markers placed.", self.state.tokens.count("tiger_marker")),
            )
            .with_layer(16)
            .with_rect(7, 3, 2, 1)
            .with_frame(tiger_frame),
            MuddleVisualNode::sprite(
                "ai-chair",
                "AI chair",
                "sprites/tigris/ai-chair.png",
                self.state.last_ai_move.clone(),
            )
            .with_layer(14)
            .with_rect(9, 1, 2, 1)
            .with_frame(pressure_frame),
            MuddleVisualNode::sprite(
                "ai-pressure-badge",
                "AI pressure",
                "sprites/tigris/ai-pressure.png",
                format!("AI pressure {}", self.state.ai_pressure),
            )
            .with_layer(20)
            .with_rect(9, 3, 2, 1)
            .with_frame(pressure_frame),
            MuddleVisualNode::text(
                "score-state-label",
                "Score state",
                format!(
                    "Human {} / AI {}",
                    self.state.human_score, self.state.ai_score
                ),
            )
            .with_layer(30)
            .with_rect(0, 6, 4, 1),
            MuddleVisualNode::sprite(
                "rubric-ledger",
                "Rubric ledger",
                "sprites/tigris/rubric-ledger.png",
                format!(
                    "Ledger: adopted={} amendment={} closed={}",
                    self.state.adopted_axes,
                    self.state.amendment_scored,
                    self.state.parliament_closed
                ),
            )
            .with_layer(18)
            .with_rect(4, 5, 3, 1)
            .with_frame(ledger_frame),
            MuddleVisualNode::sprite(
                "dissent-tags",
                "Dissent tags",
                "sprites/tigris/dissent-tags.png",
                format!("{} defended marks recorded.", self.state.defended_marks),
            )
            .with_layer(18)
            .with_rect(7, 5, 2, 1)
            .with_frame(if self.state.defended_marks > 0 {
                "tigris-green"
            } else {
                "tigris-parchment"
            }),
            MuddleVisualNode::sprite(
                "adoption-sticker",
                "Adoption sticker",
                "sprites/tigris/adoption-sticker.png",
                if self.state.adopted_axes > 0 {
                    "Gold sticker ready for the next rubric session."
                } else {
                    "Sticker space reserved until amendment scoring."
                },
            )
            .with_layer(18)
            .with_rect(9, 5, 2, 1)
            .with_frame(if self.state.adopted_axes > 0 {
                "tigris-gold"
            } else {
                "tigris-parchment"
            }),
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
                .with_rect(4, 6, 2, 1)
                .with_frame("tigris-ledger"),
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
                .with_rect(6, 6, 2, 1)
                .with_frame("tigris-closed")
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
            (_, "inspect table") => Ok(self.inspect(room_id, "table")),
            (_, "inspect persona") => Ok(self.inspect(room_id, "persona")),
            (_, "inspect pool") | (_, "inspect deck") => Ok(self.inspect(room_id, "pool")),
            (_, "inspect board") => Ok(self.inspect(room_id, "board")),
            (_, "inspect axis") => Ok(self.inspect(room_id, "axis")),
            (_, "inspect collision") => Ok(self.inspect(room_id, "collision")),
            (_, "inspect pressure") => Ok(self.inspect(room_id, "pressure")),
            (_, "inspect ledger") => Ok(self.inspect(room_id, "ledger")),
            (_, "inspect amendment") => Ok(self.inspect(room_id, "amendment")),
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
            ("score", "inspect ai") => Ok(self.inspect(room_id, "ai")),
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
        "tigris-green"
    } else {
        "tigris-ink"
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
        assert!(scene.children.len() >= 24);
        assert!(scene
            .children
            .iter()
            .any(|node| node.id == "parliament-prompt"));
        assert!(scene.children.iter().any(|node| node.id == "phase-readout"));
        assert!(scene.children.iter().any(|node| node.id == "subject-card"));
        assert!(scene
            .children
            .iter()
            .any(|node| node.id == "argument-clock"));
        assert!(scene
            .children
            .iter()
            .any(|node| node.id == "collision-lane"));
        assert!(scene.children.iter().any(|node| node.id == "rubric-ledger"));
        assert!(
            scene
                .children
                .iter()
                .filter(|node| node
                    .sprite
                    .as_ref()
                    .and_then(|sprite| sprite.frame.as_deref())
                    .is_some())
                .count()
                >= 16
        );
        assert!(scene.children.iter().any(|node| {
            node.sprite
                .as_ref()
                .and_then(|sprite| sprite.frame.as_deref())
                == Some("tigris-closed")
        }));
    }

    #[test]
    fn ai_opponent_inspect_beats_are_recoverable_and_state_neutral() {
        let mut host = parliament_ai_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");

        let turn = session
            .play_turn(&mut host, MuddleCommand::parse("inspect table"))
            .expect("table inspection succeeds");
        assert!(turn.response.contains("persona"));
        assert_eq!(session.current_room, "table");
        assert!(!host.state().persona_chosen);

        session
            .play_turn(&mut host, MuddleCommand::parse("go board"))
            .expect("board is reachable");
        let turn = session
            .play_turn(&mut host, MuddleCommand::parse("inspect collision"))
            .expect("collision inspection succeeds");
        assert!(turn.response.contains("collision lane"));
        assert_eq!(session.current_room, "board");
        assert_eq!(host.state().collision_markers, 0);

        let surface = parliament_ai_muddle_surface();
        assert!(surface
            .commands
            .iter()
            .any(|command| command.command == "inspect ledger"));
    }

    #[test]
    fn table_experience_lenses_complete_with_readable_beats() {
        let lenses: [(&str, &[&str], &[&str]); 3] = [
            (
                "first-time-designer",
                &[
                    "look",
                    "inspect table",
                    "choose persona",
                    "look",
                    "go board",
                    "inspect board",
                    "draft axis",
                    "stake claim",
                    "inspect collision",
                    "reveal collision",
                    "place tiger",
                    "end turn",
                    "inspect pressure",
                    "challenge ai",
                    "go score",
                    "inspect ledger",
                    "score amendment",
                    "close parliament",
                ],
                &["parliament read", "next action", "persona"],
            ),
            (
                "competitive-optimizer",
                &[
                    "status",
                    "choose persona",
                    "go board",
                    "draft axis",
                    "stake claim",
                    "reveal collision",
                    "place tiger",
                    "end turn",
                    "inspect pressure",
                    "challenge ai",
                    "go score",
                    "inspect ai",
                    "score amendment",
                    "close parliament",
                ],
                &["phase=", "pressure", "Challenge"],
            ),
            (
                "confused-observer",
                &[
                    "go score",
                    "inspect table",
                    "choose persona",
                    "go board",
                    "inspect axis",
                    "stake claim",
                    "draft axis",
                    "stake claim",
                    "reveal collision",
                    "inspect collision",
                    "place tiger",
                    "end turn",
                    "go score",
                    "close parliament",
                    "score amendment",
                    "inspect ledger",
                    "close parliament",
                ],
                &["Go board first", "Draft an axis", "Score amendment"],
            ),
        ];

        for (name, commands, expected_markers) in lenses {
            let mut host = parliament_ai_muddle_host();
            let mut session = MuddleSession::for_host(&host).expect("host has start room");
            let mut transcript = String::new();

            for command in commands {
                let turn = session
                    .play_turn(&mut host, MuddleCommand::parse(command))
                    .unwrap_or_else(|error| panic!("{name} command `{command}` failed: {error:?}"));
                transcript.push_str(&turn.response);
                transcript.push('\n');
            }

            assert_eq!(session.current_room, "score", "{name} finishes at score");
            assert!(host.state().parliament_closed, "{name} closes Parliament");
            for marker in expected_markers {
                assert!(
                    transcript.contains(marker),
                    "{name} transcript should contain `{marker}`"
                );
            }
        }
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
