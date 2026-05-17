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

fn main() {
    let seed = option_value("--seed").unwrap_or_else(|| "parliament-smoke".to_string());
    let result = simulate_parliament(&seed, 4);
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
}
