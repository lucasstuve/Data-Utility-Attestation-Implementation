## Quick start (Docker — recommended)

Make sure to clone and check out the `dsl-performance-benchmarks` branch
**before building the Docker image**:

```bash
git clone https://github.com/lucasstuve/Data-Utility-Attestation-Implementation.git
cd Data-Utility-Attestation-Implementation
git checkout dsl-performance-benchmarks
```

On Linux / x86-64:

```bash
docker build -t dua-benchmarks .

docker run -it --rm dua-benchmarks
```

On Apple Silicon (`M1`, `M2`, `M3`, etc.), build the image explicitly for
`linux/amd64`:

```bash
docker build --platform=linux/amd64 -t dua-benchmarks .

docker run -it --rm dua-benchmarks
```

Inside the container, start with the small batch first:

```bash
bash scripts/test.sh
```

`test.sh` uses the 1KB test file and can be used as a small initial test
to verify that the proving setup works before running the larger
benchmarks.

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

> **Apple Silicon note:** Docker execution was tested on a MacBook Air
> with an Apple M3 chip and 8 GB RAM. `scripts/test.sh` and the smaller
> benchmarks worked, but larger proving workloads may exceed the memory
> available to Docker on an 8 GB system. For larger benchmark runs, a
> machine with more RAM is recommended.

See [`DOCKER.md`](DOCKER.md) for more detail.