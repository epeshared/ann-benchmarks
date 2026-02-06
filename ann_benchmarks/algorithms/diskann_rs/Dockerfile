FROM ann-benchmarks

# Build requirements for a pyo3 native extension + DiskANN dependencies.
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      curl \
      pkg-config \
      libssl-dev \
      python3-dev \
 && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain (DiskANN-rs is pinned to recent stable; keep it explicit for reproducibility).
ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:$PATH

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal \
 && rustup toolchain install 1.92.0 \
 && rustup default 1.92.0

RUN pip install --no-cache-dir maturin

WORKDIR /tmp/build
COPY ann_benchmarks/algorithms/diskann_rs /tmp/build/diskann_rs

# If the DiskANN sources are not vendored into the build context, fetch them.
RUN if [ ! -d /tmp/build/diskann_rs/third_party/DiskANN-rs ]; then \
      git clone --depth 1 https://github.com/microsoft/DiskANN /tmp/build/diskann_rs/third_party/DiskANN-rs; \
    fi

RUN pip install -v /tmp/build/diskann_rs/native

RUN python -c "import diskann_rs_native; print('diskann_rs_native import OK')"
