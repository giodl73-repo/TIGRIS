use muddle_core::{MuddleSession, MuddleSessionSave};
use serde_json::{json, Value};
use tigris_parliament_workbench::evaluate;

fn run(value: Value) -> Value {
    serde_json::from_str(&evaluate(&value.to_string())).unwrap()
}
fn initial_save() -> MuddleSessionSave {
    let host = tigris_sim::parliament_ai_muddle_host();
    MuddleSession::for_host(&host).unwrap().save_for_host(&host)
}

#[test]
fn play_resume_and_original_snapshot_transport() {
    let first = run(json!({"commands":["choose persona","go board","draft axis","stake claim"]}));
    assert_eq!(first["kind"], "report");
    let resumed = run(
        json!({"save_text":first["save_text"],"commands":["reveal collision","place tiger","end turn"]}),
    );
    let direct = run(
        json!({"commands":["choose persona","go board","draft axis","stake claim","reveal collision","place tiger","end turn"]}),
    );
    for field in [
        "snapshot",
        "save_text",
        "replay_save_text",
        "transcript_text",
        "checkpoint_text",
    ] {
        assert_eq!(resumed[field], direct[field], "{field}");
    }
    assert_eq!(direct["snapshot"]["history"].as_array().unwrap().len(), 7);
    assert!(!direct["snapshot"]["controls"]
        .as_array()
        .unwrap()
        .is_empty());
    assert!(!direct["snapshot"]["visual_nodes"]
        .as_array()
        .unwrap()
        .is_empty());
}

#[test]
fn errors_are_attempts_without_fabricated_native_turns() {
    let report = run(json!({"commands":["not a command","look"]}));
    assert_eq!(report["kind"], "report");
    assert_eq!(report["attempts"][0]["kind"], "native_error");
    assert_eq!(report["attempts"][1]["kind"], "turn");
    assert_eq!(report["snapshot"]["turns"], 1);
}

#[test]
fn native_format_and_checkpoint_errors_stay_native() {
    assert_eq!(run(json!({"save_text":"invalid"}))["kind"], "native_error");
    let mut save = initial_save();
    save.host_checkpoint = Some("round=not-a-number".into());
    let result = run(json!({"save_text":save.encode()}));
    assert_eq!(result["kind"], "native_error");
    assert!(result["message"]
        .as_str()
        .unwrap()
        .contains("InvalidHostCheckpoint"));
}

#[test]
fn huge_round_and_duplicate_occurrences_are_refused_before_import() {
    for checkpoint in [
        "round=4294967295",
        "round=4294967295;round=1",
        "round=1;round=4294967295",
        "human_score=-2147483648",
        "stake_tokens=4097",
    ] {
        let mut save = initial_save();
        save.host_checkpoint = Some(checkpoint.into());
        assert_eq!(
            run(json!({"save_text":save.encode()}))["kind"],
            "browser_refusal",
            "{checkpoint}"
        );
    }
}

#[test]
fn exact_counter_boundary_can_resume_but_overflowing_result_is_not_accepted() {
    let mut save = initial_save();
    save.host_checkpoint = Some(save.host_checkpoint.unwrap().replacen(
        "round=1;",
        "round=4096;",
        1,
    ));
    assert_eq!(run(json!({"save_text":save.encode()}))["kind"], "report");
    let result = run(json!({"save_text":save.encode(),"commands":["go board","end turn"]}));
    assert_eq!(result["kind"], "browser_refusal");
    assert!(result.get("save_text").is_none());
}

#[test]
fn bounded_attempts_and_commands_are_not_truncated() {
    assert_eq!(
        run(json!({"commands":vec!["look";128]}))["snapshot"]["turns"],
        128
    );
    assert_eq!(
        run(json!({"commands":vec!["look";129]}))["kind"],
        "browser_refusal"
    );
    assert_eq!(
        run(json!({"commands":["é".repeat(513)]}))["kind"],
        "browser_refusal"
    );
    let saved = run(json!({"commands":vec!["look";128]}));
    assert_eq!(
        run(json!({"save_text":saved["save_text"],"commands":["look"]}))["kind"],
        "browser_refusal"
    );
}

#[test]
fn oversized_and_unknown_requests_are_typed_whole_failures() {
    for request in [
        json!({"save_text":"x".repeat(131073)}),
        json!({"commands":[],"unknown":true}),
    ] {
        let result = run(request);
        assert_ne!(result["kind"], "report");
        assert!(result.get("snapshot").is_none());
    }
    let result: Value = serde_json::from_str(&evaluate(&" ".repeat(524289))).unwrap();
    assert_eq!(result["kind"], "browser_refusal");
}

#[test]
fn native_control_preserves_refusal_when_a_byte_prefix_splits_utf8() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let request = json!({"save_text":"é".repeat(300000)}).to_string();
    let mut child = Command::new(env!("CARGO_BIN_EXE_tigris-parliament-control"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Err(error) = child.stdin.take().unwrap().write_all(request.as_bytes()) {
        assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
    }
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim(),
        evaluate(&request)
    );
}
