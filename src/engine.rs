use rand::Rng;
use rand::distributions::Uniform;

/// Single-prediction Brier Score: (f - o)^2
#[inline]
pub fn brier_score(f: f64, o: f64) -> f64 {
    (f - o) * (f - o)
}

/// Zero-one scoring rule: 1 if |f - o| > 0.5, else 0 (proper but NOT strictly proper)
#[inline]
pub fn zero_one_score(f: f64, o: f64) -> f64 {
    if (f - o).abs() > 0.5 { 1.0 } else { 0.0 }
}

/// Volume component: V(N) = min(1, log(1+N) / log(1+N_ref)), N_ref = 500
#[inline]
pub fn volume_component(n: usize) -> f64 {
    let n_ref = 500.0_f64;
    ((1.0 + n as f64).ln() / (1.0 + n_ref).ln()).min(1.0)
}

/// Composite Notch Score: NS = 0.40*(1 - BS) + 0.25*A + 0.20*V(N) + 0.15*C(t)
#[inline]
pub fn notch_score(bs: f64, accuracy: f64, n: usize, consistency: f64) -> f64 {
    0.40 * (1.0 - bs) + 0.25 * accuracy + 0.20 * volume_component(n) + 0.15 * consistency
}

/// Notch Score using an arbitrary scoring rule in place of Brier Score
#[inline]
pub fn notch_score_custom(score: f64, accuracy: f64, n: usize, consistency: f64) -> f64 {
    0.40 * (1.0 - score) + 0.25 * accuracy + 0.20 * volume_component(n) + 0.15 * consistency
}

/// Result of simulating a single predictor over N predictions
#[derive(Clone, Debug)]
pub struct PredictorResult {
    pub brier_score: f64,
    pub accuracy: f64,
    pub notch_score: f64,
    pub n: usize,
}

/// Simulate a predictor with true skill p, stated confidence c, over N predictions.
/// Returns (brier_score, accuracy, notch_score) for one trial.
pub fn simulate_predictor<R: Rng>(rng: &mut R, p: f64, c: f64, n: usize) -> PredictorResult {
    let dist = Uniform::new(0.0_f64, 1.0);
    let mut total_bs = 0.0;
    let mut correct = 0_usize;

    for _ in 0..n {
        let outcome = if rng.sample(dist) < p { 1.0 } else { 0.0 };
        total_bs += brier_score(c, outcome);
        // Directional accuracy: confidence > 0.5 means predicting outcome=1
        let predicted = if c > 0.5 { 1.0 } else { 0.0 };
        if (predicted - outcome).abs() < 0.01 {
            correct += 1;
        }
    }

    let bs = total_bs / n as f64;
    let accuracy = correct as f64 / n as f64;
    // Consistency: approximate as 1 - (variance of rolling accuracy in windows)
    // For simplicity in simulation: use 1.0 - |accuracy - p| as proxy
    let consistency = (1.0 - (accuracy - p).abs() * 4.0).max(0.0).min(1.0);
    let ns = notch_score(bs, accuracy, n, consistency);

    PredictorResult { brier_score: bs, accuracy, notch_score: ns, n }
}

/// Simulate predictor using a custom scoring rule (for impossibility experiment)
pub fn simulate_predictor_custom<R: Rng>(
    rng: &mut R,
    p: f64,
    c: f64,
    n: usize,
    scoring_fn: fn(f64, f64) -> f64,
) -> PredictorResult {
    let dist = Uniform::new(0.0_f64, 1.0);
    let mut total_score = 0.0;
    let mut correct = 0_usize;

    for _ in 0..n {
        let outcome = if rng.sample(dist) < p { 1.0 } else { 0.0 };
        total_score += scoring_fn(c, outcome);
        let predicted = if c > 0.5 { 1.0 } else { 0.0 };
        if (predicted - outcome).abs() < 0.01 {
            correct += 1;
        }
    }

    let score = total_score / n as f64;
    let accuracy = correct as f64 / n as f64;
    let consistency = (1.0 - (accuracy - p).abs() * 4.0).max(0.0).min(1.0);
    let ns = notch_score_custom(score, accuracy, n, consistency);

    PredictorResult { brier_score: score, accuracy, notch_score: ns, n }
}

/// Simulate a Sybil adversary with K wallets, N predictions per wallet.
/// Returns the BEST wallet's result (lowest BS, highest NS) across K wallets.
pub fn simulate_sybil<R: Rng>(rng: &mut R, p: f64, c: f64, k: usize, n: usize) -> PredictorResult {
    let mut best = PredictorResult {
        brier_score: f64::MAX,
        accuracy: 0.0,
        notch_score: f64::MIN,
        n,
    };

    for _ in 0..k {
        let result = simulate_predictor(rng, p, c, n);
        if result.notch_score > best.notch_score {
            best = result;
        }
    }

    best
}

/// Simulate Sybil with custom scoring rule
pub fn simulate_sybil_custom<R: Rng>(
    rng: &mut R,
    p: f64,
    c: f64,
    k: usize,
    n: usize,
    scoring_fn: fn(f64, f64) -> f64,
) -> PredictorResult {
    let mut best = PredictorResult {
        brier_score: f64::MAX,
        accuracy: 0.0,
        notch_score: f64::MIN,
        n,
    };

    for _ in 0..k {
        let result = simulate_predictor_custom(rng, p, c, n, scoring_fn);
        if result.notch_score > best.notch_score {
            best = result;
        }
    }

    best
}

/// Simulate predictor with temporal decay over months for convergence experiment
pub fn simulate_convergence<R: Rng>(
    rng: &mut R,
    p: f64,
    rate_per_month: usize,
    months: usize,
    decay: f64,
) -> Vec<f64> {
    let dist = Uniform::new(0.0_f64, 1.0);
    let mut monthly_ns = Vec::with_capacity(months);

    // Rolling weighted history
    let mut weighted_bs_sum = 0.0;
    let mut weight_sum = 0.0;
    let mut total_correct = 0_usize;
    let mut total_preds = 0_usize;

    for _month in 0..months {
        // Decay existing weights
        weighted_bs_sum *= decay;
        weight_sum *= decay;

        // New predictions this month
        let mut month_correct = 0_usize;
        for _ in 0..rate_per_month {
            let outcome = if rng.sample(dist) < p { 1.0 } else { 0.0 };
            let bs = brier_score(p, outcome); // truthful: c = p
            weighted_bs_sum += bs;
            weight_sum += 1.0;

            let predicted = if p > 0.5 { 1.0 } else { 0.0 };
            if (predicted - outcome).abs() < 0.01 {
                month_correct += 1;
            }
        }

        total_correct += month_correct;
        total_preds += rate_per_month;

        let avg_bs = if weight_sum > 0.0 { weighted_bs_sum / weight_sum } else { 0.25 };
        let accuracy = if total_preds > 0 { total_correct as f64 / total_preds as f64 } else { 0.5 };
        let consistency = (1.0 - (accuracy - p).abs() * 4.0).max(0.0).min(1.0);
        let ns = notch_score(avg_bs, accuracy, total_preds, consistency);
        monthly_ns.push(ns);
    }

    monthly_ns
}
