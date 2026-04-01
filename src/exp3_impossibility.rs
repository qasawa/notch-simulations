use crate::engine::{brier_score, zero_one_score, simulate_sybil_custom, simulate_predictor_custom};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::io::Write;

/// Experiment 3: Impossibility Demonstration (Theorem 3.1)
///
/// Under zero-one scoring (proper but NOT strictly proper), Sybil wins frequently.
/// Under Brier Score (strictly proper), Sybil almost never wins.
pub fn run() {
    println!("\n{}", "=".repeat(60));
    println!("  Experiment 3: Impossibility Demonstration (Theorem 3.1)");
    println!("{}\n", "=".repeat(60));

    let n_trials = 10_000;
    let k = 50;
    let n = 200;
    let p_sybil = 0.5;
    let c_sybil = 0.99;
    let p_truthful = 0.65;
    let c_truthful = 0.65;

    // Under zero-one rule
    let mut zo_sybil_wins = 0;
    let mut zo_sybil_scores = Vec::with_capacity(n_trials);
    let mut zo_truthful_scores = Vec::with_capacity(n_trials);

    for trial in 0..n_trials {
        let mut rng = ChaCha8Rng::seed_from_u64(42 + trial as u64);
        let sybil = simulate_sybil_custom(&mut rng, p_sybil, c_sybil, k, n, zero_one_score);
        let mut rng2 = ChaCha8Rng::seed_from_u64(500_000 + trial as u64);
        let truthful = simulate_predictor_custom(&mut rng2, p_truthful, c_truthful, n, zero_one_score);

        zo_sybil_scores.push(sybil.notch_score);
        zo_truthful_scores.push(truthful.notch_score);

        if sybil.notch_score >= truthful.notch_score {
            zo_sybil_wins += 1;
        }
    }

    // Under Brier Score
    let mut bs_sybil_wins = 0;
    let mut bs_sybil_scores = Vec::with_capacity(n_trials);
    let mut bs_truthful_scores = Vec::with_capacity(n_trials);

    for trial in 0..n_trials {
        let mut rng = ChaCha8Rng::seed_from_u64(42 + trial as u64);
        let sybil = simulate_sybil_custom(&mut rng, p_sybil, c_sybil, k, n, brier_score);
        let mut rng2 = ChaCha8Rng::seed_from_u64(500_000 + trial as u64);
        let truthful = simulate_predictor_custom(&mut rng2, p_truthful, c_truthful, n, brier_score);

        bs_sybil_scores.push(sybil.notch_score);
        bs_truthful_scores.push(truthful.notch_score);

        if sybil.notch_score >= truthful.notch_score {
            bs_sybil_wins += 1;
        }
    }

    let zo_rate = zo_sybil_wins as f64 / n_trials as f64;
    let bs_rate = bs_sybil_wins as f64 / n_trials as f64;
    let zo_sybil_mean = zo_sybil_scores.iter().sum::<f64>() / n_trials as f64;
    let zo_truthful_mean = zo_truthful_scores.iter().sum::<f64>() / n_trials as f64;
    let bs_sybil_mean = bs_sybil_scores.iter().sum::<f64>() / n_trials as f64;
    let bs_truthful_mean = bs_truthful_scores.iter().sum::<f64>() / n_trials as f64;

    println!("  Zero-One Rule (proper, NOT strictly proper):");
    println!("    Sybil win rate:     {:.1}% ({}/{})", 100.0 * zo_rate, zo_sybil_wins, n_trials);
    println!("    Sybil mean NS:      {:.4}", zo_sybil_mean);
    println!("    Truthful mean NS:   {:.4}", zo_truthful_mean);
    println!();
    println!("  Brier Score (strictly proper):");
    println!("    Sybil win rate:     {:.1}% ({}/{})", 100.0 * bs_rate, bs_sybil_wins, n_trials);
    println!("    Sybil mean NS:      {:.4}", bs_sybil_mean);
    println!("    Truthful mean NS:   {:.4}", bs_truthful_mean);

    // Write CSV
    let mut f = std::fs::File::create("data/exp3_results.csv").expect("Failed to create CSV");
    writeln!(f, "scoring_rule,sybil_win_rate,sybil_mean_score,truthful_mean_score").unwrap();
    writeln!(f, "zero_one,{:.6},{:.6},{:.6}", zo_rate, zo_sybil_mean, zo_truthful_mean).unwrap();
    writeln!(f, "brier,{:.6},{:.6},{:.6}", bs_rate, bs_sybil_mean, bs_truthful_mean).unwrap();
    println!("\n  CSV written to data/exp3_results.csv");
}
