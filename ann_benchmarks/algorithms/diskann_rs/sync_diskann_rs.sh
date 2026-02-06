#!/usr/bin/env bash
set -euo pipefail

# Copies the local DiskANN-rs checkout (from this workspace) into
# ann_benchmarks/algorithms/diskann_rs/third_party/DiskANN-rs
# so Docker builds can be done without network access.

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEST="$HERE/third_party/DiskANN-rs"

# Heuristic: in this workspace, DiskANN-rs is a sibling of ann-benchmark-epeshared.
WORKSPACE_ROOT="$(cd "$HERE/../../../../" && pwd)"
SRC="$WORKSPACE_ROOT/DiskANN-rs"

if [[ ! -d "$SRC" ]]; then
  echo "error: expected DiskANN-rs at $SRC" >&2
  echo "Set SRC manually or clone https://github.com/microsoft/DiskANN" >&2
  exit 1
fi

mkdir -p "$(dirname "$DEST")"
rm -rf "$DEST"

# Exclude build artifacts to keep it lighter.
rsync -a --delete \
  --exclude 'target/' \
  --exclude '.git/' \
  "$SRC/" "$DEST/"

echo "Synced DiskANN-rs sources to: $DEST"
