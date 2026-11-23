# Run `just --list` for the recipes.

# Format, lint and test the workspace.
check:
    @cargo fmt --all -- --check
    @cargo clippy --workspace --all-targets --all-features -- -D warnings
    @cargo nextest run --workspace --all-targets

# Compare start-up and run time with the Node.js CLI (needs /tmp/cli-microsoft365).
bench:
    @cargo build --release -p m365
    @node scripts/bench.mjs

# Compare every command with the Node.js CLI (needs /tmp/cli-microsoft365).
parity layers="meta,help,validation,trace,fixtures":
    @cargo build -p m365 --features parity
    @node scripts/parity/check.mjs --layers {{layers}}

# Test the cross-platform PGO build and benchmark helpers.
pgo-test:
    @python3 scripts/test_pgo.py

# Build one native PGO release binary.
pgo-build target="":
    @python3 scripts/build_m365_pgo.py --target-dir target/m365-pgo {{ if target == "" { "" } else { "--target=" + target } }}

# Build matched ordinary and PGO binaries, then compare held-out workloads.
pgo-bench target="":
    @python3 scripts/build_m365_pgo.py --baseline-and-pgo --target-dir target/m365-pgo {{ if target == "" { "" } else { "--target=" + target } }}
    @python3 scripts/benchmark_m365_pgo.py --target-dir target/m365-pgo {{ if target == "" { "" } else { "--target=" + target } }} --json target/m365-pgo/results.json

# Run every release gate and build one native archive without publishing.
release-check version target="":
    @scripts/release.sh check "{{ version }}" "{{ target }}"

# Check, confirm, tag, and publish a release from a clean main branch.
release version target="":
    @scripts/release.sh release "{{ version }}" "{{ target }}"
