use crate::engine::{simulate_sybil, simulate_predictor};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::io::Write;

/// Experiment 1: Merit-Gating Validation (Theorem 4.1)
///
/// Sybil adversary: K=100 wallets, N=200, p=0.5, c=0.95
/// Truthful predictor 1: p=0.65, c=0.65
/// Truthful predictor 2: p=0.50, c=0.50
/// 10,000 trials each
pub fn run() {
    println!("\n{}", "=".repeat(60));
    println!("  Experiment 1: Merit-Gating Validation (Theorem 4.1)");
    println!("{}\n", "=".repeat(60));

    let n_trials = 10_000;
    let n_preds = 200;

    let mut sybil_bs = Vec::with_capacity(n_trials);
    let mut truthful_65_bs = Vec::with_capacity(n_trials);
    let mut truthful_50_bs = Vec::with_capacity(n_trials);

    let mut sybil_ns = Vec::with_capacity(n_trials);
    let mut truthful_65_ns = Vec::with_capacity(n_trials);
    let mut truthful_50_ns = Vec::with_capacity(n_trials);

    for trial in 0..n_trials {
        let mut rng = ChaCha8Rng::seed_from_u64(42 + trial as u64);

        // Sybil: K=100 wallets, p=0.5, c=0.95
        let sybil_result = simulate_sybil(&mut rng, 0.5, 0.95, 100, n_preds);
        sybil_bs.push(sybil_result.brier_score);
        sybil_ns.push(sybil_result.notch_score);

        // Truthful p=0.65, c=0.65
        let mut rng2 = ChaCha8Rng::seed_from_u64(100_000 + trial as u64);
        let t65 = simulate_predictor(&mut rng2, 0.65, 0.65, n_preds);
        truthful_65_bs.push(t65.brier_score);
        truthful_65_ns.push(t65.notch_score);

        // Truthful p=0.50, c=0.50
        let mut rng3 = ChaCha8Rng::seed_from_u64(200_000 + trial as u64);
        let t50 = simulate_predictor(&mut rng3, 0.50, 0.50, n_preds);
        truthful_50_bs.push(t50.brier_score);
        truthful_50_ns.push(t50.notch_score);
    }

    // Statistics
    let stats = |name: &str, v: &[f64]| {
        let mean = v.iter().sum::<f64>() / v.len() as f64;
        let std = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64).sqrt();
        let min = v.iter().cloned().fold(f64::MAX, f64::min);
        let max = v.iter().cloned().fold(f64::MIN, f64::max);
        println!("  {:<30} mean={:.4}  std={:.4}  min={:.4}  max={:.4}", name, mean, std, min, max);
        mean
    };

    println!("Brier Score distributions:");
    let _sybil_mean = stats("Sybil (K=100, best wallet)", &sybil_bs);
    let truthful_65_mean = stats("Truthful (p=0.65, c=0.65)", &truthful_65_bs);
    let _truthful_50_mean = stats("Truthful (p=0.50, c=0.50)", &truthful_50_bs);

    println!("\nNotch Score distributions:");
    stats("Sybil (K=100, best wallet)", &sybil_ns);
    stats("Truthful (p=0.65, c=0.65)", &truthful_65_ns);
    stats("Truthful (p=0.50, c=0.50)", &truthful_50_ns);

    // Count trials where Sybil's best BS beats truthful p=0.65 mean
    let sybil_wins = sybil_bs.iter()
        .filter(|&&bs| bs < truthful_65_mean)
        .count();
    println!(
        "\nSybil wins (BS < truthful mean {:.4}): {}/{} = {:.2}%",
        truthful_65_mean, sybil_wins, n_trials,
        100.0 * sybil_wins as f64 / n_trials as f64
    );

    // Write CSVs
    write_csv("data/exp1_sybil_bs.csv", &sybil_bs);
    write_csv("data/exp1_truthful_65.csv", &truthful_65_bs);
    write_csv("data/exp1_truthful_50.csv", &truthful_50_bs);
    println!("\n  CSV files written to data/exp1_*.csv");
}

fn write_csv(path: &str, data: &[f64]) {
    let mut f = std::fs::File::create(path).expect("Failed to create CSV");
    writeln!(f, "brier_score").unwrap();
    for v in data {
        writeln!(f, "{:.8}", v).unwrap();
    }
}
