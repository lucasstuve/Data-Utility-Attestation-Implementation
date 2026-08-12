# Data Utility Attestation — DSL Performance Benchmarks

<!-- TODO: one-line thesis title / author / university -->

Performance benchmarks for EPL DSL query evaluation in the [Data Utility
Attestation protocol](../end-to-end-system), corresponding to Chapter 8.5
("Performance Benchmarks") of the thesis.

## Evaluation design

The performance benchmarks aim to assess whether it is technically
feasible to prove and verify data utility within the proposed system, and
to pinpoint any associated bottlenecks. Two types of benchmarks are
conducted: first, comparing proof generation for different textual queries
under varying workloads; and second, using a fixed workload while varying
the proportion of events selected by the interpreter for subsequent
evaluation.

**Query-varying benchmarks** compare queries B1–B6, representing varying
levels of evaluation complexity, each evaluated on all four
byte-size-varying data files, with both the CPU-supported and
CUDA-supported proving configurations:

| Query | Mechanism | Script |
|---|---|---|
| B1 | Filter (`ANY`) | *no matching script found in this branch* |
| B2 | Filter (`ALL`) | `filter-benchmark.sh` |
| B3 | Filter + Aggregation | `filter-aggregate-benchmark.sh` |
| B4 | Filter + Window | `filter-window-benchmark.sh` |
| B5 | Filter + Session | `filter-session-benchmark.sh` |
| B6 | Filter + Pattern | `filter-pattern-benchmark.sh` |

Two additional scripts don't correspond to a B-numbered query above:
`filter-aggregate-real-data.sh` (same shape as B3, run against the
real-world VW dataset instead of the synthetic byte-size files) and
`filter-window-aggregate-benchmark.sh` (filter + window + aggregation, a
combination not covered by B1–B6).

This setup examines how proving cost is affected by input size, the
evaluated textual utility query, and their combination.

**Event frequency benchmarks** (`frequency-benchmark-100KB.sh`) examine
how the system behaves when the query logic remains constant but the
proportion of events relevant to predicate evaluation varies — separating
computational effort from pure input volume versus the internal logical
workload of the DSL interpreter in the guest environment. The fixed query
covers event schema construction plus filtering with a two-predicate
conjunction.

> **Note:** every benchmark script here performs a **real ZK proof**
> (`RISC0_DEV_MODE=0` is hardcoded in each script) — there is no fast
> dev-mode path on this branch. Expect proving time to scale with dataset
> size. Large (100MB/1000MB) workloads are not part of this branch's
> evaluation scope — see the `end-to-end-system` branch for those.

## Repository layout

| Path                  | What it is                                            |
|------------------------|--------------------------------------------------------|
| `crates/dnf_core`      | EPL parser AST + interpreter (the DSL language itself) |
| `crates/system_core`   | Manufacturer / Data Consumer protocol logic            |
| `host`                 | Runs a single query against a dataset, records results |
| `methods` / `methods/guest` | The zkVM guest program (`eval_ast`)               |
| `benchmarks`           | Test-data generation + benchmark result recording       |
| `scripts`              | CPU benchmark scripts (real proving, datasets up to 1000KB) |
| `scripts-cuda`         | GPU-accelerated versions of the same benchmarks, plus GPU-only 100MB/1000MB runs |

## Quick start (Docker — recommended)

```bash
docker build -t dua-benchmarks .
docker run -it --rm dua-benchmarks
```

Inside the container — each of these performs real ZK proving on CPU:

```bash
bash scripts/filter-benchmark.sh
bash scripts/filter-aggregate-benchmark.sh
bash scripts/filter-aggregate-real-data.sh
bash scripts/filter-pattern-benchmark.sh
bash scripts/filter-session-benchmark.sh
bash scripts/filter-window-benchmark.sh
bash scripts/filter-window-aggregate-benchmark.sh
bash scripts/frequency-benchmark-100KB.sh
```

Results are written as CSV files (e.g. `filter-benchmark.csv`) in the
project root.

See [`DOCKER.md`](DOCKER.md) for more detail.

## GPU-accelerated benchmarks (not covered by this Docker image)

`scripts-cuda/*.sh` mirror the CPU benchmarks above with `--features cuda`
for direct CPU-vs-GPU comparison, on the same 1KB–1000KB datasets. This
requires an NVIDIA GPU, the CUDA toolkit, and driver support on the host
machine — see `scripts-cuda/set-up.sh` for bare-metal setup. It cannot run
inside this container, since the NVIDIA driver must live on the host.
Recorded results from five of these runs (filter, filter-pattern,
filter-window, filter-window-aggregate, frequency) are committed under
`benchmarks/benchmark_results/`.

Large (100MB/1000MB) workloads are out of scope for this branch — see the
`end-to-end-system` branch's README for those.

## License

See [`LICENSE`](LICENSE).
