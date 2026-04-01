# ============================================================
# Calibration-Gated Reputation — Publication Figures
# Alassa & Alashqar, AFT 2026
# Usage: cd simulations && gnuplot plot.gp
# ============================================================

set terminal pdfcairo enhanced font "Times,11" size 7in,4.5in linewidth 1.5
set encoding utf8
set style line 1 lt 1 lw 2 lc rgb "black" dt solid
set style line 2 lt 1 lw 2 lc rgb "black" dt (12,4)
set style line 3 lt 1 lw 1.5 lc rgb "gray40" dt (4,4)
set style line 4 lt 1 lw 1 lc rgb "gray60" dt (2,4)

# ────────────────────────────────────────────────────────────
# Figure 1: Merit-Gating — BS Distribution Histograms
# ────────────────────────────────────────────────────────────
set output "figures/exp1_merit_gating.pdf"
set title "Brier Score Distributions: Sybil vs. Truthful Predictors" font "Times,13"
set xlabel "Brier Score" font "Times,12"
set ylabel "Frequency" font "Times,12"
set key top right font "Times,10" box lw 0.5

binwidth = 0.005
set boxwidth binwidth
set style fill transparent solid 0.3 border
set xrange [0.05:0.55]
set yrange [0:*]

bin(x, w) = w * floor(x/w) + w/2.0

plot 'data/exp1_sybil_bs.csv' every ::1 using (bin($1,binwidth)):(1.0) smooth freq \
       with boxes ls 1 title "Sybil best (K=100, p=0.50, c=0.95)", \
     'data/exp1_truthful_65.csv' every ::1 using (bin($1,binwidth)):(1.0) smooth freq \
       with boxes ls 2 title "Truthful (p=0.65, c=0.65)", \
     'data/exp1_truthful_50.csv' every ::1 using (bin($1,binwidth)):(1.0) smooth freq \
       with boxes ls 3 title "Truthful (p=0.50, c=0.50)"

# ────────────────────────────────────────────────────────────
# Figure 2: Sybil Cost Scaling — Log-Log
# ────────────────────────────────────────────────────────────
set output "figures/exp2_sybil_cost.pdf"
set title "Sybil Cost Scaling: Total Predictions Required" font "Times,13"
set xlabel "Number of Sybil Identities (K)" font "Times,12"
set ylabel "Total Predictions (N_{total})" font "Times,12"
set key top left font "Times,10" box lw 0.5

set logscale xy
set xrange [1:1200]
set yrange [50:100000]
set format x "10^{%L}"
set format y "10^{%L}"
unset boxwidth
set style fill empty

set datafile separator ","

plot 'data/exp2_cost.csv' every ::1 using 1:2 with linespoints ls 1 pt 7 ps 0.8 \
       title "Sybil adversary N_{total}", \
     'data/exp2_cost.csv' every ::1 using 1:3 with lines ls 2 \
       title "Single truthful identity", \
     'data/exp2_cost.csv' every ::1 using 1:($1 * (log($1+1)/log(2)) * 200) with lines ls 3 \
       title "{/Symbol W}(K log K) reference"

set datafile separator whitespace

# ────────────────────────────────────────────────────────────
# Figure 3: Impossibility — Sybil Win Rate Comparison
# ────────────────────────────────────────────────────────────
set output "figures/exp3_impossibility.pdf"
set title "Sybil Win Rate: Zero-One Rule vs. Brier Score" font "Times,13"
set xlabel "" font "Times,12"
set ylabel "Sybil Win Rate (%)" font "Times,12"
set key off

unset logscale
set xrange [-0.5:1.5]
set yrange [0:*]
set format x ""
set format y "%g%%"

set boxwidth 0.6
set style fill solid 0.4 border lc rgb "black"

set datafile separator ","

set xtics ("Zero-One\n(proper)" 0, "Brier Score\n(strictly proper)" 1)

plot 'data/exp3_results.csv' every ::1 using ($0):($2*100) with boxes ls 1 notitle

set datafile separator whitespace
unset xtics
set xtics auto

# ────────────────────────────────────────────────────────────
# Figure 4: Convergence — Score Trajectories Over Time
# ────────────────────────────────────────────────────────────
set output "figures/exp4_convergence.pdf"
set title "Notch Score Convergence Under Temporal Decay" font "Times,13"
set xlabel "Month" font "Times,12"
set ylabel "Notch Score" font "Times,12"
set key bottom right font "Times,10" box lw 0.5

set xrange [1:36]
set yrange [0.4:1]
set format x "%g"
set format y "%.2f"
set xtics 6

set datafile separator ","

# p=0.75 shaded band
plot 'data/exp4_convergence.csv' every ::1 \
       using ($2 == 0.75 ? $1 : 1/0):($3-$4):($3+$4) with filledcurves fc rgb "gray85" notitle, \
     '' every ::1 using ($2 == 0.65 ? $1 : 1/0):($3-$4):($3+$4) with filledcurves fc rgb "gray90" notitle, \
     '' every ::1 using ($2 == 0.55 ? $1 : 1/0):($3-$4):($3+$4) with filledcurves fc rgb "gray93" notitle, \
     '' every ::1 using ($2 == 0.75 ? $1 : 1/0):3 with lines ls 1 title "p = 0.75", \
     '' every ::1 using ($2 == 0.65 ? $1 : 1/0):3 with lines ls 2 title "p = 0.65", \
     '' every ::1 using ($2 == 0.55 ? $1 : 1/0):3 with lines ls 3 title "p = 0.55"

set output
