#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HISTORY_DIR="$REPO_ROOT/benchmark-results"
CSV="$HISTORY_DIR/history.csv"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
COMMIT=$(git -C "$REPO_ROOT" rev-parse --short HEAD 2>/dev/null || echo "unknown")
BRANCH=$(git -C "$REPO_ROOT" branch --show-current 2>/dev/null || echo "unknown")

mkdir -p "$HISTORY_DIR"

# Initialise CSV header if missing
if [ ! -f "$CSV" ]; then
    echo "timestamp,commit,branch,benchmark,estimate_ns" > "$CSV"
fi

echo "Running benchmarks..."
cargo bench --bench benchmarks -- --output-format bencher 2>/dev/null | \
    grep "^test " | while IFS= read -r line; do
        # Parse: test <name> ... bench: <ns> ns/iter (+/- <variance>)
        NAME=$(echo "$line" | awk '{print $2}')
        NS=$(echo "$line" | sed 's/.*bench: *\([0-9,]*\).*/\1/' | tr -d ',')
        echo "${TIMESTAMP},${COMMIT},${BRANCH},${NAME},${NS}" >> "$CSV"
        echo "  ${NAME}: ${NS} ns/iter"
    done

echo ""
echo "Results appended to ${CSV}"
echo ""

# 3-point trend (baseline / mid / current)
BENCHMARKS=$(tail -n +2 "$CSV" | cut -d',' -f4 | sort -u)
for bench in $BENCHMARKS; do
    ENTRIES=$(grep ",$bench," "$CSV" | tail -3)
    COUNT=$(echo "$ENTRIES" | wc -l)
    if [ "$COUNT" -ge 2 ]; then
        FIRST=$(echo "$ENTRIES" | head -1 | cut -d',' -f5)
        LAST=$(echo "$ENTRIES" | tail -1 | cut -d',' -f5)
        if [ "$FIRST" -gt 0 ] 2>/dev/null; then
            DELTA=$(( (LAST - FIRST) * 100 / FIRST ))
            echo "  ${bench}: ${FIRST} → ${LAST} ns (${DELTA}%)"
        fi
    fi
done
