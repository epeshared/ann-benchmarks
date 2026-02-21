import os
import numpy as np

from ..base.module import BaseANN


class DiskANNRS(BaseANN):
    """ANN-Benchmarks adapter for the Rust DiskANN implementation.

    This adapter is intentionally thin: it delegates all heavy lifting to the
    native extension `diskann_rs_native` (built into the algorithm Docker image).
    """

    def __init__(self, metric, param):
        metric_map = {
            "angular": "cosine",
            "euclidean": "l2",
            # Allow callers outside ann-benchmarks to pass the native names.
            "cosine": "cosine",
            "l2": "l2",
        }
        if metric not in metric_map:
            raise ValueError(f"unsupported metric={metric!r}; expected one of {sorted(metric_map.keys())}")
        self.metric = metric_map[metric]

        self.l_build = int(param["l_build"])
        self.max_outdegree = int(param["max_outdegree"])
        self.alpha = float(param["alpha"])

        # Optional persistence controls (used by external harnesses).
        # - index_prefix: base path prefix for saved index artifacts
        # - index_action: one of {"build", "load", "build_and_save", "auto"}
        self.index_prefix = param.get("index_prefix")
        self.index_action = param.get("index_action", "build")

        self.name = f"diskann-rs({self.metric})-L{self.l_build}-R{self.max_outdegree}-a{self.alpha}"

        self._index = None
        self._l_search = None

    def fit(self, X):
        from diskann_rs_native import Index

        action = str(self.index_action)
        prefix = self.index_prefix

        if action not in {"build", "load", "build_and_save", "auto"}:
            raise ValueError(
                f"invalid index_action={action!r}; expected one of build/load/build_and_save/auto"
            )

        if action in {"load", "auto"}:
            if not prefix:
                raise ValueError("index_prefix is required when index_action is load/auto")
            meta_path = f"{prefix}.meta.json"
            if action == "load" or os.path.exists(meta_path):
                self._index = Index.load(prefix)
            else:
                action = "build_and_save"

        if action in {"build", "build_and_save"}:
            # Keep a predictable dtype/layout for the native extension.
            X = np.asarray(X, dtype=np.float32, order="C")
            if X.ndim != 2:
                raise ValueError(f"expected 2D array, got shape={X.shape}")

            self._index = Index(
                metric=self.metric,
                l_build=self.l_build,
                max_outdegree=self.max_outdegree,
                alpha=self.alpha,
            )
            self._index.fit(X)
            if action == "build_and_save":
                if not prefix:
                    raise ValueError("index_prefix is required when index_action is build_and_save")
                self._index.save(prefix)

        if self._l_search is not None:
            self._index.set_l_search(self._l_search)

    def set_query_arguments(self, l_search):
        self._l_search = int(l_search)
        if self._index is not None:
            self._index.set_l_search(self._l_search)

    def query(self, v, n):
        if self._index is None:
            raise RuntimeError("fit() must be called before query()")

        v = np.asarray(v, dtype=np.float32, order="C")
        if v.ndim != 1:
            raise ValueError(f"expected 1D array, got shape={v.shape}")

        k = int(n)
        l_search = int(self._l_search) if self._l_search is not None else max(k, 10)
        l_search = max(l_search, k)
        return self._index.search(v, k, l_search)

    def batch_query(self, X, n):
        if self._index is None:
            raise RuntimeError("fit() must be called before batch_query()")

        X = np.asarray(X, dtype=np.float32, order="C")
        if X.ndim != 2:
            raise ValueError(f"expected 2D array, got shape={X.shape}")

        k = int(n)
        l_search = int(self._l_search) if self._l_search is not None else max(k, 10)
        l_search = max(l_search, k)
        self.res = self._index.batch_search(X, k, l_search)

    def get_batch_results(self):
        return self.res


class DiskANNRS_PQ(BaseANN):
    """ANN-Benchmarks adapter for DiskANN-rs with Product Quantization (PQ).

    Backed by `diskann_rs_native.PQIndex`.
    """

    def __init__(self, metric, param):
        metric_map = {
            "angular": "cosine",
            "euclidean": "l2",
            "cosine": "cosine",
            "l2": "l2",
        }
        if metric not in metric_map:
            raise ValueError(f"unsupported metric={metric!r}; expected one of {sorted(metric_map.keys())}")
        self.metric = metric_map[metric]

        self.l_build = int(param["l_build"])
        self.max_outdegree = int(param["max_outdegree"])
        self.alpha = float(param["alpha"])

        self.num_pq_chunks = int(param["num_pq_chunks"])
        self.num_centers = int(param.get("num_centers", 256))
        self.max_k_means_reps = int(param.get("max_k_means_reps", 12))

        translate_to_center = param.get("translate_to_center")
        if translate_to_center is None:
            # Centering is generally compatible with L2, but it breaks cosine/inner-product
            # semantics for unit-normalized vectors (e.g. angular datasets).
            translate_to_center = self.metric == "l2"
        elif isinstance(translate_to_center, str):
            v = translate_to_center.strip().lower()
            if v in {"1", "true", "t", "yes", "y"}:
                translate_to_center = True
            elif v in {"0", "false", "f", "no", "n"}:
                translate_to_center = False
            else:
                translate_to_center = bool(translate_to_center)

        self.translate_to_center = bool(translate_to_center)
        self.rng_seed = int(param.get("rng_seed", 0))

        self.index_prefix = param.get("index_prefix")
        self.index_action = param.get("index_action", "build")

        self.name = (
            f"diskann-rs-pq({self.metric})-L{self.l_build}-R{self.max_outdegree}-a{self.alpha}"
            f"-chunks{self.num_pq_chunks}-cent{self.num_centers}"
        )

        self._index = None
        self._l_search = None

    def fit(self, X):
        from diskann_rs_native import PQIndex

        action = str(self.index_action)
        prefix = self.index_prefix

        if action not in {"build", "load", "build_and_save", "auto"}:
            raise ValueError(
                f"invalid index_action={action!r}; expected one of build/load/build_and_save/auto"
            )

        if action in {"load", "auto"}:
            if not prefix:
                raise ValueError("index_prefix is required when index_action is load/auto")
            meta_path = f"{prefix}.pq.meta.json"
            if action == "load" or os.path.exists(meta_path):
                self._index = PQIndex.load(prefix)
            else:
                action = "build_and_save"

        if action in {"build", "build_and_save"}:
            X = np.asarray(X, dtype=np.float32, order="C")
            if X.ndim != 2:
                raise ValueError(f"expected 2D array, got shape={X.shape}")

            self._index = PQIndex(
                metric=self.metric,
                l_build=self.l_build,
                max_outdegree=self.max_outdegree,
                alpha=self.alpha,
                num_pq_chunks=self.num_pq_chunks,
                num_centers=self.num_centers,
                max_k_means_reps=self.max_k_means_reps,
                translate_to_center=self.translate_to_center,
                rng_seed=self.rng_seed,
            )
            self._index.fit(X)
            if action == "build_and_save":
                if not prefix:
                    raise ValueError("index_prefix is required when index_action is build_and_save")
                self._index.save(prefix)

        if self._l_search is not None:
            self._index.set_l_search(self._l_search)

    def set_query_arguments(self, l_search):
        self._l_search = int(l_search)
        if self._index is not None:
            self._index.set_l_search(self._l_search)

    def query(self, v, n):
        if self._index is None:
            raise RuntimeError("fit() must be called before query()")

        v = np.asarray(v, dtype=np.float32, order="C")
        if v.ndim != 1:
            raise ValueError(f"expected 1D array, got shape={v.shape}")

        k = int(n)
        l_search = int(self._l_search) if self._l_search is not None else max(k, 10)
        l_search = max(l_search, k)
        return self._index.search(v, k, l_search)

    def batch_query(self, X, n):
        if self._index is None:
            raise RuntimeError("fit() must be called before batch_query()")

        X = np.asarray(X, dtype=np.float32, order="C")
        if X.ndim != 2:
            raise ValueError(f"expected 2D array, got shape={X.shape}")

        k = int(n)
        l_search = int(self._l_search) if self._l_search is not None else max(k, 10)
        l_search = max(l_search, k)
        self.res = self._index.batch_search(X, k, l_search)

    def get_batch_results(self):
        return self.res


class DiskANNRS_Spherical(BaseANN):
    """ANN-Benchmarks adapter for DiskANN-rs with spherical quantization.

    Backed by `diskann_rs_native.SphericalIndex`.
    """

    def __init__(self, metric, param):
        metric_map = {
            "angular": "cosine",
            "euclidean": "l2",
            "cosine": "cosine",
            "l2": "l2",
        }
        if metric not in metric_map:
            raise ValueError(f"unsupported metric={metric!r}; expected one of {sorted(metric_map.keys())}")
        self.metric = metric_map[metric]

        self.l_build = int(param["l_build"])
        self.max_outdegree = int(param["max_outdegree"])
        self.alpha = float(param["alpha"])
        self.nbits = int(param["nbits"])
        self.rng_seed = int(param.get("rng_seed", 0))

        self.index_prefix = param.get("index_prefix")
        self.index_action = param.get("index_action", "build")

        self.name = (
            f"diskann-rs-spherical({self.metric})-L{self.l_build}-R{self.max_outdegree}"
            f"-a{self.alpha}-nbits{self.nbits}"
        )

        self._index = None
        self._l_search = None

    def fit(self, X):
        from diskann_rs_native import SphericalIndex

        action = str(self.index_action)
        prefix = self.index_prefix

        if action not in {"build", "load", "build_and_save", "auto"}:
            raise ValueError(
                f"invalid index_action={action!r}; expected one of build/load/build_and_save/auto"
            )

        if action in {"load", "auto"}:
            if not prefix:
                raise ValueError("index_prefix is required when index_action is load/auto")
            meta_path = f"{prefix}.spherical.meta.json"
            if action == "load" or os.path.exists(meta_path):
                self._index = SphericalIndex.load(prefix)
            else:
                action = "build_and_save"

        if action in {"build", "build_and_save"}:
            X = np.asarray(X, dtype=np.float32, order="C")
            if X.ndim != 2:
                raise ValueError(f"expected 2D array, got shape={X.shape}")

            self._index = SphericalIndex(
                metric=self.metric,
                l_build=self.l_build,
                max_outdegree=self.max_outdegree,
                alpha=self.alpha,
                nbits=self.nbits,
                rng_seed=self.rng_seed,
            )
            self._index.fit(X)
            if action == "build_and_save":
                if not prefix:
                    raise ValueError("index_prefix is required when index_action is build_and_save")
                self._index.save(prefix)

        if self._l_search is not None:
            self._index.set_l_search(self._l_search)

    def set_query_arguments(self, l_search):
        self._l_search = int(l_search)
        if self._index is not None:
            self._index.set_l_search(self._l_search)

    def query(self, v, n):
        if self._index is None:
            raise RuntimeError("fit() must be called before query()")

        v = np.asarray(v, dtype=np.float32, order="C")
        if v.ndim != 1:
            raise ValueError(f"expected 1D array, got shape={v.shape}")

        k = int(n)
        l_search = int(self._l_search) if self._l_search is not None else max(k, 10)
        l_search = max(l_search, k)
        return self._index.search(v, k, l_search)

    def batch_query(self, X, n):
        if self._index is None:
            raise RuntimeError("fit() must be called before batch_query()")

        X = np.asarray(X, dtype=np.float32, order="C")
        if X.ndim != 2:
            raise ValueError(f"expected 2D array, got shape={X.shape}")

        k = int(n)
        l_search = int(self._l_search) if self._l_search is not None else max(k, 10)
        l_search = max(l_search, k)
        self.res = self._index.batch_search(X, k, l_search)

    def get_batch_results(self):
        return self.res
