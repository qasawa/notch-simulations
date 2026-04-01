use crate::engine::simulate_convergence;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::io::Write;

/// Experiment 4: Stationary Convergence (Theorem 4.4)
///
/// Three truthful predictors: p in {0.55, 0.65, 0.75}
/// Rate: 20 predictions/month, 36 months, decay lambda=0.95/month
/// 1,000 trials per skill level
pub fn run() {
    println!("\n{}", "=".repeat(60));
    println!("  Experiment 4: Stationary Convergence (Theorem 4.4)");
    println!("{}\n", "=".repeat(60));

    let n_trials = 1_000;
    let months = 36;
    let rate = 20;
    let decay = 0.95;
    let skills = [0.55, 0.65, 0.75];

    // Collect: for each (p, month), store all trial NS values
    let mut all_data: Vec<(usize, f64, f64, f64)> = Vec::new(); // (month, p, mean, std)

    for &p in &skills {
        // Collect per-month NS across trials
        let mut monthly_trials: Vec<Vec<f64>> = vec![Vec::with_capacity(n_trials); months];

        for trial in 0..n_trials {
            let mut rng = ChaCha8Rng::seed_from_u64(42 + trial as u64 + (p * 10000.0) as u64);
            let ns_series = simulate_convergence(&mut rng, p, rate, months, decay);
            for (m, &ns) in ns_series.iter().enumerate() {
                monthly_trials[m].push(ns);
            }
        }

        println!("  p = {:.2}:", p);
        for m in 0..months {
            let vals = &monthly_trials[m];
            let mean = vals.iter().sum::<f64>() / vals.len() as f64;
            let std = (vals.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / vals.len() as f64).sqrt();
            all_data.push((m + 1, p, mean, std));

            if m == months - 1 {
                println!("    Month {:>2}: mean_NS={:.4}, std={:.4} (final)", m + 1, mean, std);
            }
        }
    }

    // Write CSV
    let mut f = std::fs::File::create("data/exp4_convergence.csv").expect("Failed to create CSV");
    writeln!(f, "month,p,mean_ns,std_ns").unwrap();
    for (month, p, mean, std) in &all_data {
        writeln!(f, "{},{:.2},{:.6},{:.6}", month, p, mean, std).unwrap();
    }
    println!("\n  CSV written to data/exp4_convergence.csv");
}
