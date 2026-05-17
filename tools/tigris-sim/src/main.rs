use rally_core::{
    percent_of, ActorTrace, ComparisonDelta, ComparisonReport, SimulationMetric, SimulationRun,
    ValidationFinding, ValidationReport,
};
use std::collections::BTreeMap;
use std::env;

const AXES: &[&str] = &[
    "Tension Budget",
    "Minimum-Score Shape",
    "Elegance",
    "Scarcity Bite",
    "Engine-Garden Dependency",
    "Late-Game Lock-in",
    "Point-Salad Incommensurability",
    "Anti-Catch-up Pressure",
    "Decision Density",
    "Architectural Novelty",
    "Interaction",
    "Catastrophe Pressure",
];

const ADJACENCIES: &[(&str, &str)] = &[
    ("Tension Budget", "Anti-Catch-up Pressure"),
    ("Tension Budget", "Scarcity Bite"),
    ("Elegance", "Action-Menu Clarity"),
    ("Decision Density", "Point-Salad Incommensurability"),
    ("Late-Game Lock-in", "Anti-Catch-up Pressure"),
    ("Variance Calibration", "Catastrophe Pressure"),
];

const EXPANDED_ADJACENCIES: &[(&str, &str)] = &[
    ("Architectural Novelty", "Interaction"),
    ("Minimum-Score Shape", "Anti-Catch-up Pressure"),
    ("Scarcity Bite", "Late-Game Lock-in"),
    ("Elegance", "Decision Density"),
];

#[derive(Debug, Clone, Copy)]
struct RuleVariant {
    name: &'static str,
    adoption_threshold: u32,
    collision_credit: i32,
    expanded_adjacency: bool,
    challenge_chance: u32,
}

const BASELINE: RuleVariant = RuleVariant {
    name: "baseline",
    adoption_threshold: 2,
    collision_credit: 2,
    expanded_adjacency: false,
    challenge_chance: 65,
};

const VARIANTS: &[RuleVariant] = &[
    BASELINE,
    RuleVariant {
        name: "expanded-adjacency",
        adoption_threshold: 2,
        collision_credit: 2,
        expanded_adjacency: true,
        challenge_chance: 65,
    },
    RuleVariant {
        name: "lower-adoption",
        adoption_threshold: 1,
        collision_credit: 2,
        expanded_adjacency: false,
        challenge_chance: 65,
    },
    RuleVariant {
        name: "collision-boost",
        adoption_threshold: 2,
        collision_credit: 3,
        expanded_adjacency: false,
        challenge_chance: 75,
    },
    RuleVariant {
        name: "tournament-pressure",
        adoption_threshold: 2,
        collision_credit: 3,
        expanded_adjacency: true,
        challenge_chance: 75,
    },
];

#[derive(Debug, Clone)]
struct Chair {
    id: String,
    axes: Vec<String>,
    reserve: i32,
    raw_points: i32,
    collision_points: i32,
    adopted_axes: u32,
    trace: ActorTrace,
}

#[derive(Debug, Clone)]
struct AxisState {
    defended: u32,
    refuted: u32,
}

#[derive(Debug, Clone)]
struct ParliamentResult {
    run: SimulationRun,
    chairs: Vec<Chair>,
    axis_states: BTreeMap<String, AxisState>,
    collisions: u32,
    metrics: Vec<SimulationMetric>,
    report: ValidationReport,
}

#[derive(Debug, Clone)]
struct BatchSummary {
    runs: usize,
    average_collisions: f64,
    adoption_rate: f64,
    no_collision_rate: f64,
    no_adoption_rate: f64,
    average_winning_score: f64,
    chair_wins: BTreeMap<String, u32>,
}

#[derive(Debug, Clone, Copy)]
struct UpstageVariant {
    name: &'static str,
    trigger_bonus: i32,
    commitment_bonus: i32,
    double_bonus: i32,
    chain_limit: Option<u32>,
}

#[derive(Debug, Clone)]
struct UpstagePlayer {
    id: String,
    score: u32,
    star_moments: u32,
    trace: ActorTrace,
}

#[derive(Debug, Clone)]
struct UpstageResult {
    run: SimulationRun,
    players: Vec<UpstagePlayer>,
    scenes: u32,
    upstages: u32,
    doubles: u32,
    false_upstages: u32,
    chaos_scenes: u32,
    metrics: Vec<SimulationMetric>,
    report: ValidationReport,
}

#[derive(Debug, Clone)]
struct UpstageBatchSummary {
    runs: usize,
    average_upstages: f64,
    double_rate: f64,
    false_upstage_rate: f64,
    chaos_rate: f64,
    average_score_spread: f64,
}

const UPSTAGE_BASELINE: UpstageVariant = UpstageVariant {
    name: "baseline",
    trigger_bonus: 0,
    commitment_bonus: 0,
    double_bonus: 0,
    chain_limit: None,
};

const UPSTAGE_VARIANTS: &[UpstageVariant] = &[
    UPSTAGE_BASELINE,
    UpstageVariant {
        name: "warmup-scene",
        trigger_bonus: 0,
        commitment_bonus: 12,
        double_bonus: 0,
        chain_limit: None,
    },
    UpstageVariant {
        name: "clearer-triggers",
        trigger_bonus: 10,
        commitment_bonus: 6,
        double_bonus: 0,
        chain_limit: None,
    },
    UpstageVariant {
        name: "double-spotlight",
        trigger_bonus: 4,
        commitment_bonus: 4,
        double_bonus: 18,
        chain_limit: None,
    },
    UpstageVariant {
        name: "eight-player-chain-limit",
        trigger_bonus: 6,
        commitment_bonus: 6,
        double_bonus: 10,
        chain_limit: Some(3),
    },
];

fn main() {
    let seed = option_value("--seed").unwrap_or_else(|| "parliament-smoke".to_string());
    let player_count = option_value("--players")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|players| (3..=4).contains(players))
        .unwrap_or(4);
    let runs = option_value("--runs")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1);
    let game = option_value("--game").unwrap_or_else(|| "parliament".to_string());
    if game == "upstage" {
        run_upstage_cli(&seed, runs);
        return;
    }
    if has_flag("--compare-variants") {
        let runs = runs.max(20);
        let baseline_results = simulate_batch(&seed, runs, player_count, BASELINE);
        let baseline = summarize_batch(&baseline_results);
        println!("TIGRIS simulator: Parliament variant comparison");
        println!("seed: {seed}");
        println!("players: {player_count}");
        println!("runs_per_variant: {runs}");
        for variant in VARIANTS {
            let results = simulate_batch(&seed, runs, player_count, *variant);
            let summary = summarize_batch(&results);
            let comparison =
                compare_to_baseline("0001-parliament", &baseline, variant.name, &summary);
            let status = if variant.name == "baseline" {
                "baseline".to_string()
            } else {
                comparison.status().to_string()
            };
            let improved = if variant.name == "baseline" {
                "-".to_string()
            } else {
                format!(
                    "{}/{}",
                    comparison.improved_count(),
                    comparison.deltas.len()
                )
            };
            println!(
                "variant:{} status={} improved={} avg_collisions={:.2} adoption_rate={:.1}% no_collision={:.1}% no_adoption={:.1}% avg_winner={:.2}",
                variant.name,
                status,
                improved,
                summary.average_collisions,
                summary.adoption_rate,
                summary.no_collision_rate,
                summary.no_adoption_rate,
                summary.average_winning_score
            );
        }
        return;
    }
    let variant = option_value("--variant")
        .as_deref()
        .and_then(find_variant)
        .unwrap_or(BASELINE);
    if runs > 1 {
        let results = simulate_batch(&seed, runs, player_count, variant);
        let summary = summarize_batch(&results);
        println!("TIGRIS simulator: Parliament batch");
        println!("seed: {seed}");
        println!("variant: {}", variant.name);
        println!("players: {player_count}");
        println!("runs: {}", summary.runs);
        println!("average_collisions: {:.2}", summary.average_collisions);
        println!("adoption_rate: {:.1}%", summary.adoption_rate);
        println!("no_collision_rate: {:.1}%", summary.no_collision_rate);
        println!("no_adoption_rate: {:.1}%", summary.no_adoption_rate);
        println!(
            "average_winning_score: {:.2}",
            summary.average_winning_score
        );
        for (chair, wins) in summary.chair_wins {
            println!("wins:{}={}", chair, wins);
        }
        return;
    }

    let result = simulate_parliament(&seed, player_count, variant);
    println!("TIGRIS simulator: Parliament");
    println!("run_id: {}", result.run.run_id);
    println!("variant: {}", variant.name);
    println!("status: {}", result.report.status());
    println!("collisions: {}", result.collisions);
    println!("axes_seen: {}", result.axis_states.len());
    for metric in &result.metrics {
        println!("metric:{}={:.2}", metric.name, metric.value);
    }
    for chair in &result.chairs {
        println!(
            "{} final={} actions={} blocked={}",
            chair.id,
            final_score(chair),
            chair.trace.actions,
            chair.trace.blocked_turns
        );
    }
    for finding in &result.report.findings {
        println!("{}:{}: {}", finding.severity, finding.code, finding.message);
    }
}

fn run_upstage_cli(seed: &str, runs: usize) {
    let player_count = option_value("--players")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|players| (4..=8).contains(players))
        .unwrap_or(6);
    if has_flag("--compare-variants") {
        let runs = runs.max(24);
        let baseline_results = simulate_upstage_batch(seed, runs, player_count, UPSTAGE_BASELINE);
        let baseline = summarize_upstage_batch(&baseline_results);
        println!("TIGRIS simulator: UPSTAGE variant comparison");
        println!("seed: {seed}");
        println!("players: {player_count}");
        println!("runs_per_variant: {runs}");
        for variant in UPSTAGE_VARIANTS {
            let results = simulate_upstage_batch(seed, runs, player_count, *variant);
            let summary = summarize_upstage_batch(&results);
            let comparison = compare_upstage_to_baseline(&baseline, variant.name, &summary);
            let status = if variant.name == "baseline" {
                "baseline".to_string()
            } else {
                comparison.status().to_string()
            };
            let improved = if variant.name == "baseline" {
                "-".to_string()
            } else {
                format!(
                    "{}/{}",
                    comparison.improved_count(),
                    comparison.deltas.len()
                )
            };
            println!(
                "variant:{} status={} improved={} avg_upstages={:.2} double_rate={:.1}% false_upstage_rate={:.1}% chaos_rate={:.1}% avg_score_spread={:.2}",
                variant.name,
                status,
                improved,
                summary.average_upstages,
                summary.double_rate,
                summary.false_upstage_rate,
                summary.chaos_rate,
                summary.average_score_spread
            );
        }
        return;
    }

    let variant = option_value("--variant")
        .as_deref()
        .and_then(find_upstage_variant)
        .unwrap_or(UPSTAGE_BASELINE);
    if runs > 1 {
        let results = simulate_upstage_batch(seed, runs, player_count, variant);
        let summary = summarize_upstage_batch(&results);
        println!("TIGRIS simulator: UPSTAGE batch");
        println!("seed: {seed}");
        println!("variant: {}", variant.name);
        println!("players: {player_count}");
        println!("runs: {}", summary.runs);
        println!("average_upstages: {:.2}", summary.average_upstages);
        println!("double_rate: {:.1}%", summary.double_rate);
        println!("false_upstage_rate: {:.1}%", summary.false_upstage_rate);
        println!("chaos_rate: {:.1}%", summary.chaos_rate);
        println!("average_score_spread: {:.2}", summary.average_score_spread);
        return;
    }

    let result = simulate_upstage(seed, player_count, variant);
    println!("TIGRIS simulator: UPSTAGE");
    println!("run_id: {}", result.run.run_id);
    println!("variant: {}", variant.name);
    println!("players: {player_count}");
    println!("status: {}", result.report.status());
    println!("scenes: {}", result.scenes);
    println!("upstages: {}", result.upstages);
    println!("doubles: {}", result.doubles);
    println!("false_upstages: {}", result.false_upstages);
    println!("chaos_scenes: {}", result.chaos_scenes);
    for metric in &result.metrics {
        println!("metric:{}={:.2}", metric.name, metric.value);
    }
    for player in &result.players {
        println!(
            "{} score={} star_moments={} actions={} blocked={}",
            player.id,
            player.score,
            player.star_moments,
            player.trace.actions,
            player.trace.blocked_turns
        );
    }
    for finding in &result.report.findings {
        println!("{}:{}: {}", finding.severity, finding.code, finding.message);
    }
}

fn has_flag(name: &str) -> bool {
    env::args().any(|arg| arg == name)
}

fn option_value(name: &str) -> Option<String> {
    let args = env::args().collect::<Vec<_>>();
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}

fn find_variant(name: &str) -> Option<RuleVariant> {
    VARIANTS
        .iter()
        .copied()
        .find(|variant| variant.name == name)
}

fn find_upstage_variant(name: &str) -> Option<UpstageVariant> {
    UPSTAGE_VARIANTS
        .iter()
        .copied()
        .find(|variant| variant.name == name)
}

fn simulate_parliament(seed: &str, player_count: usize, variant: RuleVariant) -> ParliamentResult {
    let run = SimulationRun::new(
        "tigris-sim",
        &format!("0001-parliament-{}", variant.name),
        seed,
    );
    let mut rng = run.rng();
    let mut draft_pool = AXES.iter().map(|axis| axis.to_string()).collect::<Vec<_>>();
    let mut chairs = (0..player_count)
        .map(|idx| Chair {
            id: format!("C{}", idx + 1),
            axes: Vec::new(),
            reserve: if player_count == 3 { 12 } else { 9 },
            raw_points: 0,
            collision_points: 0,
            adopted_axes: 0,
            trace: ActorTrace::new(&format!("C{}", idx + 1), "designer-chair"),
        })
        .collect::<Vec<_>>();

    for _ in 0..3 {
        for chair in &mut chairs {
            let idx = rng.choose_index(draft_pool.len()).unwrap_or(0);
            chair.axes.push(draft_pool.remove(idx));
            chair.trace.record_action();
        }
    }

    let mut axis_states = BTreeMap::<String, AxisState>::new();
    let mut collisions = 0;
    for round in 0..4 {
        let stakes = chairs
            .iter_mut()
            .map(|chair| {
                let axis = chair.axes[round % chair.axes.len()].clone();
                let stake = 1 + rng.next_bounded(3) as i32;
                chair.reserve -= stake;
                chair.trace.record_action();
                (chair.id.clone(), axis, stake)
            })
            .collect::<Vec<_>>();

        for left in 0..stakes.len() {
            for right in left + 1..stakes.len() {
                if adjacent(&stakes[left].1, &stakes[right].1, variant) {
                    collisions += 1;
                    let left_wins = rng.percent_chance(55);
                    let winner_axis = if left_wins {
                        &stakes[left].1
                    } else {
                        &stakes[right].1
                    };
                    let loser_axis = if left_wins {
                        &stakes[right].1
                    } else {
                        &stakes[left].1
                    };
                    axis_states
                        .entry(winner_axis.clone())
                        .or_insert(AxisState {
                            defended: 0,
                            refuted: 0,
                        })
                        .defended += 1;
                    axis_states
                        .entry(loser_axis.clone())
                        .or_insert(AxisState {
                            defended: 0,
                            refuted: 0,
                        })
                        .refuted += 1;
                    if let Some(chair) = chairs.iter_mut().find(|chair| {
                        chair.id
                            == if left_wins {
                                stakes[left].0.as_str()
                            } else {
                                stakes[right].0.as_str()
                            }
                    }) {
                        chair.collision_points += variant.collision_credit;
                    }
                }
            }
        }

        for (idx, (_, axis, stake)) in stakes.iter().enumerate() {
            let challenged = rng.percent_chance(variant.challenge_chance);
            let chair = &mut chairs[idx];
            if !challenged {
                chair.trace.record_blocked_turn();
                continue;
            }
            let can_defend = chair.reserve >= 2 && rng.percent_chance(60);
            let state = axis_states.entry(axis.clone()).or_insert(AxisState {
                defended: 0,
                refuted: 0,
            });
            if can_defend {
                chair.reserve -= 2;
                state.defended += 1;
                chair.raw_points += 1 + stake;
            } else {
                state.refuted += 1;
                chair.raw_points -= 1;
            }
            chair.trace.record_action();
        }
    }

    for chair in &mut chairs {
        chair.adopted_axes = chair
            .axes
            .iter()
            .filter(|axis| {
                axis_states.get(*axis).is_some_and(|state| {
                    state.defended >= variant.adoption_threshold && state.refuted == 0
                })
            })
            .count() as u32;
    }

    let adopted = axis_states
        .values()
        .filter(|state| state.defended >= variant.adoption_threshold && state.refuted == 0)
        .count() as u32;
    let refuted = axis_states
        .values()
        .filter(|state| state.refuted >= 2)
        .count() as u32;
    let mut findings = Vec::new();
    if collisions == 0 {
        findings.push(ValidationFinding::warning(
            "no-collisions",
            "games/0001-parliament/design.md#9",
            "no adjacency collisions fired; Parliament loses its core pressure source",
        ));
    }
    if adopted == 0 {
        findings.push(ValidationFinding::warning(
            "no-adoptions",
            "games/0001-parliament/design.md#6.3",
            "no axis reached adopted state in the simulated session",
        ));
    }
    let metrics = vec![
        SimulationMetric::new("collision_rate", percent_of(collisions, 4)),
        SimulationMetric::new("adopted_axes", adopted as f64),
        SimulationMetric::new("refuted_axes", refuted as f64),
        SimulationMetric::new("adoption_threshold", variant.adoption_threshold as f64),
        SimulationMetric::new("collision_credit", variant.collision_credit as f64),
    ];

    ParliamentResult {
        run,
        chairs,
        axis_states,
        collisions,
        metrics,
        report: ValidationReport {
            subject: "0001-parliament".to_string(),
            findings,
        },
    }
}

fn simulate_batch(
    seed: &str,
    runs: usize,
    player_count: usize,
    variant: RuleVariant,
) -> Vec<ParliamentResult> {
    (0..runs)
        .map(|idx| simulate_parliament(&format!("{seed}-{idx}"), player_count, variant))
        .collect()
}

fn summarize_batch(results: &[ParliamentResult]) -> BatchSummary {
    let collision_sum = results.iter().map(|result| result.collisions).sum::<u32>();
    let adopted_runs = results
        .iter()
        .filter(|result| metric_value(result, "adopted_axes") > 0.0)
        .count();
    let no_collision_runs = results
        .iter()
        .filter(|result| result.collisions == 0)
        .count();
    let no_adoption_runs = results.len().saturating_sub(adopted_runs);
    let mut chair_wins = BTreeMap::new();
    let winning_score_sum = results
        .iter()
        .map(|result| {
            let winner = result
                .chairs
                .iter()
                .max_by_key(|chair| final_score(chair))
                .expect("parliament result should have chairs");
            *chair_wins.entry(winner.id.clone()).or_insert(0) += 1;
            final_score(winner)
        })
        .sum::<i32>();

    BatchSummary {
        runs: results.len(),
        average_collisions: collision_sum as f64 / results.len().max(1) as f64,
        adoption_rate: percent_of(adopted_runs as u32, results.len() as u32),
        no_collision_rate: percent_of(no_collision_runs as u32, results.len() as u32),
        no_adoption_rate: percent_of(no_adoption_runs as u32, results.len() as u32),
        average_winning_score: winning_score_sum as f64 / results.len().max(1) as f64,
        chair_wins,
    }
}

fn compare_to_baseline(
    subject: &str,
    baseline: &BatchSummary,
    candidate_id: &str,
    candidate: &BatchSummary,
) -> ComparisonReport {
    let mut report = ComparisonReport::new(subject, "baseline", candidate_id);
    report.add_delta(ComparisonDelta::higher_is_better(
        "adoption_rate",
        baseline.adoption_rate,
        candidate.adoption_rate,
    ));
    report.add_delta(ComparisonDelta::higher_is_better(
        "average_collisions",
        baseline.average_collisions,
        candidate.average_collisions,
    ));
    report.add_delta(ComparisonDelta::lower_is_better(
        "no_adoption_rate",
        baseline.no_adoption_rate,
        candidate.no_adoption_rate,
    ));
    report
}

fn simulate_upstage(seed: &str, player_count: usize, variant: UpstageVariant) -> UpstageResult {
    let run = SimulationRun::new("tigris-sim", &format!("upstage-{}", variant.name), seed);
    let mut rng = run.rng();
    let mut players = (0..player_count)
        .map(|idx| UpstagePlayer {
            id: format!("P{}", idx + 1),
            score: 0,
            star_moments: 0,
            trace: ActorTrace::new(&format!("P{}", idx + 1), "stage-player"),
        })
        .collect::<Vec<_>>();
    let scenes = (player_count as u32 + 2).max(6);
    let mut holder = 0usize;
    let mut upstages = 0;
    let mut doubles = 0;
    let mut false_upstages = 0;
    let mut chaos_scenes = 0;

    for scene in 0..scenes {
        players[holder].trace.record_action();
        let mut committed = Vec::new();
        for idx in 0..players.len() {
            if idx == holder {
                continue;
            }
            let trigger_chance =
                (26 + player_count as i32 * 4 + variant.trigger_bonus).clamp(10, 80) as u32;
            if !rng.percent_chance(trigger_chance) {
                continue;
            }
            let commitment_chance =
                (72 + variant.commitment_bonus - player_count as i32).clamp(35, 95) as u32;
            if rng.percent_chance(commitment_chance) {
                players[idx].trace.record_action();
                committed.push(idx);
            } else {
                players[idx].trace.record_blocked_turn();
                false_upstages += 1;
            }
        }

        if let Some(limit) = variant.chain_limit {
            if committed.len() as u32 > limit {
                committed.truncate(limit as usize);
            }
        }
        if committed.len() >= 4 {
            chaos_scenes += 1;
        }
        if committed.is_empty() {
            players[holder].score += 1;
            holder = (holder + 1) % players.len();
            continue;
        }

        upstages += committed.len() as u32;
        let double_chance =
            (22 + variant.double_bonus + player_count as i32 * 2).clamp(0, 85) as u32;
        if committed.len() >= 2 && rng.percent_chance(double_chance) {
            doubles += 1;
            let left = committed[0];
            let right = committed[1];
            players[left].score += 1;
            players[right].score += 1;
            players[left].star_moments += 1;
            players[right].star_moments += 1;
            holder = if scene % 2 == 0 { left } else { right };
        } else {
            let winner = committed[0];
            players[winner].score += 1;
            if committed.len() >= 2 {
                players[winner].star_moments += 1;
            }
            holder = winner;
        }
    }

    let score_spread = score_spread(&players);
    let mut findings = Vec::new();
    if upstages < scenes / 2 {
        findings.push(ValidationFinding::warning(
            "low-upstage-pressure",
            "parlor/games/0001-upstage/design.md#phase-3-the-upstage-may-occur-multiple-times",
            "too few scenes produced an upstage; the physical trigger economy may feel flat",
        ));
    }
    if false_upstages > player_count as u32 {
        findings.push(ValidationFinding::warning(
            "commitment-risk",
            "parlor/games/0001-upstage/design.md#the-commitment-rule",
            "players hesitated after trigger fires often enough to threaten the social contract",
        ));
    }
    if player_count >= 8 && chaos_scenes > 2 {
        findings.push(ValidationFinding::warning(
            "eight-player-chaos",
            "parlor/games/0001-upstage/panel/SUMMARY.md#open-design-flags",
            "high player count produced repeated pile-on scenes",
        ));
    }

    UpstageResult {
        run,
        players,
        scenes,
        upstages,
        doubles,
        false_upstages,
        chaos_scenes,
        metrics: vec![
            SimulationMetric::new("upstage_rate", percent_of(upstages, scenes)),
            SimulationMetric::new("double_rate", percent_of(doubles, scenes)),
            SimulationMetric::new(
                "false_upstage_rate",
                percent_of(false_upstages, upstages.max(1)),
            ),
            SimulationMetric::new("chaos_rate", percent_of(chaos_scenes, scenes)),
            SimulationMetric::new("score_spread", score_spread as f64),
        ],
        report: ValidationReport {
            subject: "parlor-0001-upstage".to_string(),
            findings,
        },
    }
}

fn simulate_upstage_batch(
    seed: &str,
    runs: usize,
    player_count: usize,
    variant: UpstageVariant,
) -> Vec<UpstageResult> {
    (0..runs)
        .map(|idx| simulate_upstage(&format!("{seed}-{idx}"), player_count, variant))
        .collect()
}

fn summarize_upstage_batch(results: &[UpstageResult]) -> UpstageBatchSummary {
    let scenes = results.iter().map(|result| result.scenes).sum::<u32>();
    let upstages = results.iter().map(|result| result.upstages).sum::<u32>();
    let doubles = results.iter().map(|result| result.doubles).sum::<u32>();
    let false_upstages = results
        .iter()
        .map(|result| result.false_upstages)
        .sum::<u32>();
    let chaos_scenes = results
        .iter()
        .map(|result| result.chaos_scenes)
        .sum::<u32>();
    let score_spread_sum = results
        .iter()
        .map(|result| score_spread(&result.players))
        .sum::<u32>();

    UpstageBatchSummary {
        runs: results.len(),
        average_upstages: upstages as f64 / results.len().max(1) as f64,
        double_rate: percent_of(doubles, scenes),
        false_upstage_rate: percent_of(false_upstages, upstages.max(1)),
        chaos_rate: percent_of(chaos_scenes, scenes),
        average_score_spread: score_spread_sum as f64 / results.len().max(1) as f64,
    }
}

fn compare_upstage_to_baseline(
    baseline: &UpstageBatchSummary,
    candidate_id: &str,
    candidate: &UpstageBatchSummary,
) -> ComparisonReport {
    let mut report = ComparisonReport::new("parlor-0001-upstage", "baseline", candidate_id);
    report.add_delta(ComparisonDelta::higher_is_better(
        "average_upstages",
        baseline.average_upstages,
        candidate.average_upstages,
    ));
    report.add_delta(ComparisonDelta::higher_is_better(
        "double_rate",
        baseline.double_rate,
        candidate.double_rate,
    ));
    report.add_delta(ComparisonDelta::lower_is_better(
        "false_upstage_rate",
        baseline.false_upstage_rate,
        candidate.false_upstage_rate,
    ));
    report.add_delta(ComparisonDelta::lower_is_better(
        "chaos_rate",
        baseline.chaos_rate,
        candidate.chaos_rate,
    ));
    report
}

fn metric_value(result: &ParliamentResult, name: &str) -> f64 {
    result
        .metrics
        .iter()
        .find(|metric| metric.name == name)
        .map(|metric| metric.value)
        .unwrap_or(0.0)
}

fn adjacent(left: &str, right: &str, variant: RuleVariant) -> bool {
    ADJACENCIES
        .iter()
        .any(|(a, b)| (left == *a && right == *b) || (left == *b && right == *a))
        || (variant.expanded_adjacency
            && EXPANDED_ADJACENCIES
                .iter()
                .any(|(a, b)| (left == *a && right == *b) || (left == *b && right == *a)))
}

fn final_score(chair: &Chair) -> i32 {
    (chair.raw_points + chair.collision_points) * (1 + chair.adopted_axes as i32)
}

fn score_spread(players: &[UpstagePlayer]) -> u32 {
    let min = players.iter().map(|player| player.score).min().unwrap_or(0);
    let max = players.iter().map(|player| player.score).max().unwrap_or(0);
    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parliament_sim_is_repeatable() {
        let left = simulate_parliament("fixed", 4, BASELINE);
        let right = simulate_parliament("fixed", 4, BASELINE);

        assert_eq!(left.collisions, right.collisions);
        assert_eq!(final_score(&left.chairs[0]), final_score(&right.chairs[0]));
    }

    #[test]
    fn parliament_sim_tracks_designer_activity() {
        let result = simulate_parliament("activity", 4, BASELINE);

        assert_eq!(result.chairs.len(), 4);
        assert!(result.chairs.iter().all(|chair| chair.trace.actions > 0));
        assert!(!result.metrics.is_empty());
    }

    #[test]
    fn batch_summary_reports_adoption_pressure() {
        let results = simulate_batch("batch", 8, 4, BASELINE);
        let summary = summarize_batch(&results);

        assert_eq!(summary.runs, 8);
        assert!(summary.average_collisions >= 0.0);
        assert!(!summary.chair_wins.is_empty());
    }

    #[test]
    fn tournament_pressure_variant_improves_adoption_pressure() {
        let baseline = summarize_batch(&simulate_batch("variant", 20, 4, BASELINE));
        let tuned = summarize_batch(&simulate_batch(
            "variant",
            20,
            4,
            find_variant("tournament-pressure").unwrap(),
        ));

        assert!(tuned.adoption_rate >= baseline.adoption_rate);
        assert!(tuned.average_collisions >= baseline.average_collisions);
    }

    #[test]
    fn comparison_report_marks_tournament_pressure_improved() {
        let baseline = summarize_batch(&simulate_batch("comparison", 20, 4, BASELINE));
        let tuned = summarize_batch(&simulate_batch(
            "comparison",
            20,
            4,
            find_variant("tournament-pressure").unwrap(),
        ));
        let report =
            compare_to_baseline("0001-parliament", &baseline, "tournament-pressure", &tuned);

        assert_eq!(report.status(), "improved");
    }

    #[test]
    fn upstage_sim_is_repeatable() {
        let left = simulate_upstage("fixed-upstage", 6, UPSTAGE_BASELINE);
        let right = simulate_upstage("fixed-upstage", 6, UPSTAGE_BASELINE);

        assert_eq!(left.upstages, right.upstages);
        assert_eq!(score_spread(&left.players), score_spread(&right.players));
    }

    #[test]
    fn upstage_batch_reports_social_contract_pressure() {
        let results = simulate_upstage_batch("upstage-batch", 12, 8, UPSTAGE_BASELINE);
        let summary = summarize_upstage_batch(&results);

        assert_eq!(summary.runs, 12);
        assert!(summary.average_upstages >= 0.0);
        assert!(summary.false_upstage_rate >= 0.0);
    }

    #[test]
    fn upstage_ship_variant_reduces_false_upstages() {
        let baseline = summarize_upstage_batch(&simulate_upstage_batch(
            "upstage-variant",
            24,
            8,
            UPSTAGE_BASELINE,
        ));
        let limited = summarize_upstage_batch(&simulate_upstage_batch(
            "upstage-variant",
            24,
            8,
            find_upstage_variant("eight-player-chain-limit").unwrap(),
        ));

        assert!(limited.false_upstage_rate <= baseline.false_upstage_rate);
        assert!(limited.chaos_rate <= baseline.chaos_rate);
    }
}
