This folder builds the native extension `diskann_rs_native` (pyo3) used by
`ann_benchmarks.algorithms.diskann_rs`.

By default the build expects DiskANN sources at:
`ann_benchmarks/algorithms/diskann_rs/third_party/DiskANN-rs/`.

Options:
- Offline/local: run `../sync_diskann_rs.sh` (copies from workspace `DiskANN-rs/`).
- Online: build the Docker image; the Dockerfile will `git clone` Microsoft/DiskANN if the
  sources are not present in `third_party/`.
