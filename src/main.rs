mod engine;
mod exp1_merit_gating;
mod exp2_sybil_cost;
mod exp3_impossibility;
mod exp4_convergence;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Calibration-Gated Reputation — Monte Carlo Simulations ║");
    println!("║  Alassa & Alashqar, AFT 2026                           ║");
    println!("║  10,000 trials per configuration                        ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    // Ensure output directories exist
    std::fs::create_dir_all("data").expect("Failed to create data/");
    std::fs::create_dir_all("figures").expect("Failed to create figures/");

    let t0 = std::time::Instant::now();

    exp1_merit_gating::run();
    exp2_sybil_cost::run();
    exp3_impossibility::run();
    exp4_convergence::run();

    let elapsed = t0.elapsed();
    println!("\n{}", "=".repeat(60));
    println!("  All experiments complete in {:.2}s", elapsed.as_secs_f64());
    println!("  Data files: data/exp1_*.csv, data/exp2_cost.csv, data/exp3_results.csv, data/exp4_convergence.csv");
    println!("  To generate figures: gnuplot plot.gp");
    println!("{}", "=".repeat(60));
}
