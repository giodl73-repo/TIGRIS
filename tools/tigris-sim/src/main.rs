use rally_core::{
    percent_of, ActorTrace, SimulationMetric, SimulationRun, ValidationFinding, ValidationReport,
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

fn main() {
    let seed = option_value("--seed").unwrap_or_else(|| "parliament-smoke".to_string());
    let player_count = option_value("--players")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|players| (3..=4).contains(players))
        .unwrap_or(4);
    let runs = option_value("--runs")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1);
    if runs > 1 {
        let results = simulate_batch(&seed, runs, player_count);
        let summary = summarize_batch(&results);
        println!("TIGRIS simulator: Parliament batch");
        println!("seed: {seed}");
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

    let result = simulate_parliament(&seed, player_count);
    println!("TIGRIS simulator: Parliament");
    println!("run_id: {}", result.run.run_id);
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

fn option_value(name: &str) -> Option<String> {
    let args = env::args().collect::<Vec<_>>();
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}

fn simulate_parliament(seed: &str, player_count: usize) -> ParliamentResult {
    let run = SimulationRun::new("tigris-sim", "0001-parliament", seed);
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
                if adjacent(&stakes[left].1, &stakes[right].1) {
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
                        chair.collision_points += 2;
                    }
                }
            }
        }

        for (idx, (_, axis, stake)) in stakes.iter().enumerate() {
            let challenged = rng.percent_chance(65);
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
                axis_states
                    .get(*axis)
                    .is_some_and(|state| state.defended >= 2 && state.refuted == 0)
            })
            .count() as u32;
    }

    let adopted = axis_states
        .values()
        .filter(|state| state.defended >= 2 && state.refuted == 0)
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

fn simulate_batch(seed: &str, runs: usize, player_count: usize) -> Vec<ParliamentResult> {
    (0..runs)
        .map(|idx| simulate_parliament(&format!("{seed}-{idx}"), player_count))
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

fn metric_value(result: &ParliamentResult, name: &str) -> f64 {
    result
        .metrics
        .iter()
        .find(|metric| metric.name == name)
        .map(|metric| metric.value)
        .unwrap_or(0.0)
}

fn adjacent(left: &str, right: &str) -> bool {
    ADJACENCIES
        .iter()
        .any(|(a, b)| (left == *a && right == *b) || (left == *b && right == *a))
}

fn final_score(chair: &Chair) -> i32 {
    (chair.raw_points + chair.collision_points) * (1 + chair.adopted_axes as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parliament_sim_is_repeatable() {
        let left = simulate_parliament("fixed", 4);
        let right = simulate_parliament("fixed", 4);

        assert_eq!(left.collisions, right.collisions);
        assert_eq!(final_score(&left.chairs[0]), final_score(&right.chairs[0]));
    }

    #[test]
    fn parliament_sim_tracks_designer_activity() {
        let result = simulate_parliament("activity", 4);

        assert_eq!(result.chairs.len(), 4);
        assert!(result.chairs.iter().all(|chair| chair.trace.actions > 0));
        assert!(!result.metrics.is_empty());
    }

    #[test]
    fn batch_summary_reports_adoption_pressure() {
        let results = simulate_batch("batch", 8, 4);
        let summary = summarize_batch(&results);

        assert_eq!(summary.runs, 8);
        assert!(summary.average_collisions >= 0.0);
        assert!(!summary.chair_wins.is_empty());
    }
}
