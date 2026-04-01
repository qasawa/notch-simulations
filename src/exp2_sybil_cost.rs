use crate::engine::simulate_sybil;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::io::Write;

/// Experiment 2: Sybil Cost Scaling (Theorem 4.3)
///
/// For K in [1..1000], find minimum total predictions N_total for Sybil (p=0.60, c=0.80)
/// to achieve NS >= 0.70 with probability >= 0.95.
/// Binary search on N per wallet, 1000 trials at each candidate.
pub fn run() {
    println!("\n{}", "=".repeat(60));
    println!("  Experiment 2: Sybil Cost Scaling (Theorem 4.3)");
    println!("{}\n", "=".repeat(60));

    let ks = [1, 2, 5, 10, 20, 50, 100, 200, 500, 1000];
    let target_ns = 0.70;
    let target_prob = 0.95;
    let check_trials = 1000;
    let p_sybil = 0.60;
    let c_sybil = 0.80;

    let mut results: Vec<(usize, usize)> = Vec::new();

    // Also compute N_single for truthful predictor
    let n_single = find_min_n_truthful(0.60, 0.60, target_ns, target_prob, check_trials);
    println!("  Single truthful predictor (p=0.60, c=0.60): N_single = {}", n_single);
    println!();

    for &k in &ks {
        let n_per_wallet = find_min_n_sybil(p_sybil, c_sybil, k, target_ns, target_prob, check_trials);
        let n_total = k * n_per_wallet;
        results.push((k, n_total));
        println!("  K={:>5}  N_per_wallet={:>5}  N_total={:>8}", k, n_per_wallet, n_total);
    }

    // Write CSV
    let mut f = std::fs::File::create("data/exp2_cost.csv").expect("Failed to create CSV");
    writeln!(f, "K,N_total,N_single").unwrap();
    for (k, n_total) in &results {
        writeln!(f, "{},{},{}", k, n_total, n_single).unwrap();
    }
    println!("\n  CSV written to data/exp2_cost.csv");
}

/// Binary search for minimum N per wallet such that P(max NS >= target) >= prob
fn find_min_n_sybil(p: f64, c: f64, k: usize, target_ns: f64, target_prob: f64, trials: usize) -> usize {
    let mut lo: usize = 1;
    let mut hi: usize = 2000;

    // First check if hi is enough
    while check_sybil_prob(p, c, k, hi, target_ns, trials) < target_prob {
        hi *= 2;
        if hi > 100_000 { return hi; }
    }

    while lo < hi {
        let mid = (lo + hi) / 2;
        let prob = check_sybil_prob(p, c, k, mid, target_ns, trials);
        if prob >= target_prob {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    lo
}

fn check_sybil_prob(p: f64, c: f64, k: usize, n: usize, target_ns: f64, trials: usize) -> f64 {
    let mut successes = 0;
    for trial in 0..trials {
        let mut rng = ChaCha8Rng::seed_from_u64(42_000 + trial as u64 + (k * n) as u64);
        let result = simulate_sybil(&mut rng, p, c, k, n);
        if result.notch_score >= target_ns {
            successes += 1;
        }
    }
    successes as f64 / trials as f64
}

fn find_min_n_truthful(p: f64, c: f64, target_ns: f64, target_prob: f64, trials: usize) -> usize {
    let mut lo: usize = 1;
    let mut hi: usize = 2000;

    while check_truthful_prob(p, c, hi, target_ns, trials) < target_prob {
        hi *= 2;
        if hi > 100_000 { return hi; }
    }

    while lo < hi {
        let mid = (lo + hi) / 2;
        if check_truthful_prob(p, c, mid, target_ns, trials) >= target_prob {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    lo
}

fn check_truthful_prob(p: f64, c: f64, n: usize, target_ns: f64, trials: usize) -> f64 {
    use crate::engine::simulate_predictor;
    let mut successes = 0;
    for trial in 0..trials {
        let mut rng = ChaCha8Rng::seed_from_u64(300_000 + trial as u64 + n as u64);
        let result = simulate_predictor(&mut rng, p, c, n);
        if result.notch_score >= target_ns {
            successes += 1;
        }
    }
    successes as f64 / trials as f64
}
