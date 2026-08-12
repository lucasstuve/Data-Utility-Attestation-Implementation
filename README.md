# Data Utility Attestation — DSL Performance Benchmarks

<!-- TODO: one-line thesis title / author / university -->

Performance benchmarks for EPL DSL query evaluation in the [Data Utility
Attestation protocol](../end-to-end-system) — proving time, cycle counts,
and verification time for the RISC Zero zkVM guest, across query shapes
(filter, filter+aggregate, pattern, session, window, window+aggregate,
frequency) and data sizes (1KB–1000KB, plus larger GPU-only runs).

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
