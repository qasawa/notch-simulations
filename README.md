# Calibration-Gated Reputation — Monte Carlo Simulations

**Strictly Proper Scoring Rules as Trustless Merit Filters in Decentralized Prediction Systems**

Qais Alassa & Osama Alashqar

---

Rust simulation suite validating 5 theorems from *Calibration-Gated Reputation*. All experiments use 10,000 independent Monte Carlo trials with deterministic seeding (ChaCha8Rng, seed 42) for full reproducibility. Publication-quality figures are generated via gnuplot (B&W, Times font, pdfcairo).

## Results

| Experiment | Theorem | Key Finding |
|---|---|---|
| **Merit-Gating** | Thm 4.1 | Sybil wins **0/10,000** trials. Sybil mean BS=0.373 vs truthful BS=0.227. Complete separation. |
| **Sybil Cost Scaling** | Thm 4.3 | K=1 needs 201 predictions. K=1000 needs 33,000. Superlinear scaling confirmed. Single truthful predictor needs only 125. |
| **Impossibility** | Thm 3.1 | Zero-one rule (proper, not strictly proper): Sybil wins **1.4%**. Brier Score (strictly proper): Sybil wins **0.0%**. Strict propriety is necessary. |
| **Convergence** | Thm 4.4 | p=0.55 → NS 0.779, p=0.65 → NS 0.813, p=0.75 → NS 0.855. Monotonically increasing, distinct stationary values. |

## How to Run

```bash
# Run all experiments (~21 seconds)
cargo run --release

# Generate publication figures
gnuplot plot.gp
```

Data files are written to `data/`. Figures are written to `figures/`.

## Structure

```
├── Cargo.toml
├── src/
│   ├── main.rs                  # Master runner
│   ├── engine.rs                # Core simulation engine
│   ├── exp1_merit_gating.rs     # Theorem 4.1 validation
│   ├── exp2_sybil_cost.rs       # Theorem 4.3 validation
│   ├── exp3_impossibility.rs    # Theorem 3.1 validation
│   └── exp4_convergence.rs      # Theorem 4.4 validation
├── plot.gp                      # Gnuplot script (4 PDF figures)
├── data/                        # CSV output (generated)
└── figures/                     # PDF figures (generated)
```

## Requirements

- Rust 1.75+
- gnuplot 5.4+ (with pdfcairo terminal)

## Paper

Paper available at [SSRN URL — placeholder].

Formal specification: [Zenodo DOI: 10.5281/zenodo.19118356](https://zenodo.org/records/19118356)

## License

MIT
