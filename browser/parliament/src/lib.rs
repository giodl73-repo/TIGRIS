//! Bounded browser admission over the original Parliament pilot and MUDDLE session.
use muddle_core::{MuddleClientInfo, MuddleCommand, MuddleSession, MuddleSessionSave};
use serde::Deserialize;
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

pub const REQUEST_BYTES: usize = 524_288;
const REPORT_BYTES: usize = 2_097_152;
const SAVE_BYTES: usize = 131_072;
const ATTEMPTS: usize = 128;
const COUNTER_MAX: u32 = 4096;
const SCORE_MAX: u32 = 10_000;
const NAME: &str = "tigris-parliament-ai";
const DESCRIPTION: &str =
    "TIGRIS Parliament: product-owned MUDDLE host with a deterministic AI opponent.";
const HINTS: &str = "`choose persona`, `go board`, `draft axis`, `stake claim`, `reveal collision`, `place tiger`, `end turn`, `challenge ai`, `go score`, `score amendment`, `close parliament`, `quit`.";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    #[serde(default)]
    commands: Vec<String>,
    #[serde(default)]
    save_text: Option<String>,
}

type Failure = (&'static str, String);
fn refusal(message: impl Into<String>) -> Failure {
    ("browser_refusal", message.into())
}
fn bounded(text: &str, limit: usize, name: &str) -> Result<(), Failure> {
    if text.len() > limit {
        Err(refusal(format!("{name} exceeds {limit} UTF-8 bytes")))
    } else {
        Ok(())
    }
}

// Resource admission only. Native decode/import retain format and policy ownership.
// Inspect EVERY occurrence: native import permits duplicate fields with last wins.
fn admit_checkpoint(checkpoint: &str) -> Result<(), Failure> {
    bounded(checkpoint, 8192, "Host checkpoint")?;
    for part in checkpoint.split(';') {
        bounded(part, 1024, "Checkpoint field")?;
        if let Some((key, value)) = part.split_once('=') {
            match key {
                "round" | "ai_pressure" | "stake_tokens" | "collision_markers"
                | "defended_marks" | "adopted_axes" => {
                    if let Ok(number) = value.parse::<u32>() {
                        if number > COUNTER_MAX {
                            return Err(refusal(format!("Checkpoint {key} exceeds {COUNTER_MAX}")));
                        }
                    }
                }
                "human_score" | "ai_score" | "tiger_marker" => {
                    if let Ok(number) = value.parse::<i32>() {
                        if number.unsigned_abs() > SCORE_MAX {
                            return Err(refusal(format!(
                                "Checkpoint {key} magnitude exceeds {SCORE_MAX}"
                            )));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn play(request: Request) -> Result<Value, Failure> {
    if request.commands.len() > ATTEMPTS {
        return Err(refusal("At most 128 command attempts"));
    }
    for command in &request.commands {
        bounded(command, 1024, "Command")?;
    }
    let native = |error| ("native_error", format!("{error:?}"));
    let mut host = tigris_sim::parliament_ai_muddle_host();
    let mut session = if let Some(text) = request.save_text {
        bounded(&text, SAVE_BYTES, "Save")?;
        let save = MuddleSessionSave::decode(&text).map_err(native)?;
        if save.commands.len() + request.commands.len() > ATTEMPTS {
            return Err(refusal("Saved commands plus new attempts exceed 128"));
        }
        bounded(&save.current_room, 1024, "Saved room")?;
        for command in &save.commands {
            bounded(command, 1024, "Saved command")?;
        }
        if let Some(checkpoint) = &save.host_checkpoint {
            admit_checkpoint(checkpoint)?;
        }
        MuddleSession::resume_for_host(&mut host, &save).map_err(native)?
    } else {
        MuddleSession::for_host(&host).map_err(native)?
    };
    let mut attempts = Vec::new();
    let mut last = session
        .transcript
        .last()
        .map(|turn| turn.response.clone())
        .unwrap_or_else(|| "Ready. Choose a native hint or type a command.".into());
    for input in request.commands {
        let command = MuddleCommand::parse(&input);
        let normalized = command.normalized();
        match session.play_turn(&mut host, command) {
            Ok(turn) => {
                last = turn.response.clone();
                attempts.push(
                    json!({"input":input,"normalized":normalized,"kind":"turn","response":last}),
                );
            }
            Err(error) => {
                last = format!("{error:?}");
                attempts.push(json!({"input":input,"normalized":normalized,"kind":"native_error","error":last}));
            }
        }
    }
    let snapshot = session.client_snapshot(
        &host,
        MuddleClientInfo {
            host: NAME.into(),
            description: DESCRIPTION.into(),
            suggested_commands: HINTS.into(),
        },
        last,
    );
    let save = session.save_for_host(&host);
    // Outputs must remain admissible on the next request; never truncate history.
    let save_text = save.encode();
    bounded(&save_text, SAVE_BYTES, "Result save")?;
    if let Some(checkpoint) = &save.host_checkpoint {
        admit_checkpoint(checkpoint)?;
    }
    let transcript = muddle_cli::render_transcript(
        muddle_cli::MuddleCliHostInfo {
            name: NAME,
            description: DESCRIPTION,
            suggested_commands: HINTS,
        },
        &session,
    );
    Ok(json!({
        "kind":"report",
        "snapshot":muddle_browser_player::snapshot_json(&snapshot),
        "attempts":attempts,
        "save_text":save_text,
        "replay_save_text":session.save().encode(),
        "transcript_text":transcript,
        "checkpoint_text":save.host_checkpoint,
        "checkpoint_semantics":"Native resume replays commands, checks final room, then imports the checkpoint. A save carries no host identity. Acceptance does not prove provenance, transcript/state consistency, corpus review or rubric authority."
    }))
}

/// Exact JSON transport shared by the native control executable and browser WASM.
#[wasm_bindgen]
pub fn evaluate(input: &str) -> String {
    let result = if input.len() > REQUEST_BYTES {
        Err(refusal("Request exceeds 512 KiB"))
    } else {
        serde_json::from_str::<Request>(input)
            .map_err(|error| ("input_error", error.to_string()))
            .and_then(play)
    };
    let value = match result {
        Ok(value) => value,
        Err((kind, message)) => json!({"kind":kind,"message":message}),
    };
    let report = value.to_string();
    if report.len() > REPORT_BYTES {
        json!({"kind":"browser_refusal","message":"Complete report exceeds 2 MiB; no partial result accepted"}).to_string()
    } else {
        report
    }
}
