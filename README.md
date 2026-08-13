# Data Utility Attestation — End-to-End System

<!-- TODO: one-line thesis title / author / university -->

An implementation of a zero-knowledge **data utility attestation** protocol.
Data consumers express utility requirements over a data batch as a query in
a custom DSL (referred to in the code as **EPL**); the query is evaluated
against the private batch inside a [RISC Zero](https://risczero.com) zkVM
guest, producing a receipt that proves the evaluation result without
revealing the raw data.

## Protocol overview

The demo (`host`) walks through six phases, matching the roles in the
protocol:

1. **Batch provisioning** — the Manufacturer signs a data batch (RSA).
2. **Utility rule generation** — the Data Consumer trains a decision tree
   over labelled data and derives a utility predicate from it.
3. **Preprocessing** — the User (data holder) parses the predicate into an
   EPL AST and indexes the raw batch.
4. **Proof generation** — the User runs the AST + batch + signature through
   the zkVM guest (`methods/guest`) and gets back a receipt.
5. **Verification** — the Data Consumer verifies the receipt, checks the
   committed predicate matches what was requested, and checks the
   Manufacturer's signature is intact.
6. **Purchase decision** — based on the verified result.

> **Note:** `scripts/end-to-end-test.sh` runs this protocol on CPU with a
> single small workload (`user-batch.json`) purely to **illustrate** the
> six phases above. It is not the thesis's evaluation methodology. The
> actual evaluation runs — larger workloads (10MB/100MB/1000MB), real ZK
> proofs, GPU-accelerated — are done via
> `scripts-cuda/end-to-end-test-cuda.sh` on a CUDA-capable machine; see
> "Real evaluation runs" below.

## Repository layout

| Path                  | What it is                                            |
|------------------------|--------------------------------------------------------|
| `crates/dnf_core`      | EPL parser AST + interpreter (the DSL language itself) |
| `crates/system_core`   | Manufacturer / Data Consumer protocol logic            |
| `host`                 | Orchestrates the end-to-end demo (`host/src/main.rs`)  |
| `methods` / `methods/guest` | The zkVM guest program (`eval_ast`) that evaluates the EPL AST privately |
| `benchmarks`           | Test-data generation + benchmark result recording       |
| `scripts`              | CPU demo/smoke-test scripts                             |
| `scripts-cuda`         | Bare-metal GPU setup + the real evaluation/benchmark scripts |

## Quick start (Docker — recommended)

```bash
docker build -t dua-e2e .
docker run -it --rm dua-e2e
```

Inside the container:

```bash
# unit tests
cargo test -p dnf_core -p system_core -p host

# just the DSL language-expressiveness suite (host/src/unit_tests_expressiveness.rs,
# module `lang_expressiveness_tests`) — parses and evaluates queries across
# quantifiers, value types, DNF combinations, every aggregate function, and
# time-windows
cargo test -p host --lib lang_expressiveness_tests

# fast smoke test (dev mode, small fixed dataset)
bash scripts/test.sh

# CPU demonstrator only — illustrates the protocol, not the real evaluation
bash scripts/end-to-end-test.sh
```

See [`DOCKER.md`](DOCKER.md) for more detail.

## Quick start (without Docker)

Requires [`rustup`](https://rustup.rs) (picks up the pinned toolchain from
`rust-toolchain.toml` automatically) and the RISC Zero toolchain:

```bash
curl -L https://risczero.com/install | bash
rzup install
cargo run -p host --release -- user-batch.json
```

Dev mode (fast, no real proof) is controlled by `RISC0_DEV_MODE=1|0`.

## Real evaluation runs (not covered by the Docker image)

The thesis's actual end-to-end evaluation runs on larger workloads
(10MB/100MB/1000MB datasets) with GPU acceleration, via
`scripts-cuda/end-to-end-test-cuda.sh`, plus two dedicated large-workload
runs: `scripts-cuda/extensive-bechmark-100MB.sh` and
`scripts-cuda/extensive-bechmark-1000MB.sh`. This is the branch that owns
these high-workload runs (as opposed to `dsl-performance-benchmarks`,
which stays in the 1KB–1000KB range).

All of these require an NVIDIA GPU, the CUDA toolkit, and driver support
on the host machine — see `scripts-cuda/set-up.sh` for bare-metal setup.
They cannot run inside this container, since the NVIDIA driver must live
on the host. Recorded results from these runs are committed under
`benchmarks/benchmark_results/`.

## License

See [`LICENSE`](LICENSE).
