import numpy as np

from ..base.module import BaseANN


class DiskANNRS(BaseANN):
    """ANN-Benchmarks adapter for the Rust DiskANN implementation.

    This adapter is intentionally thin: it delegates all heavy lifting to the
    native extension `diskann_rs_native` (built into the algorithm Docker image).
    """

    def __init__(self, metric, param):
        self.metric = {"angular": "cosine", "euclidean": "l2"}[metric]

        self.l_build = int(param["l_build"])
        self.max_outdegree = int(param["max_outdegree"])
        self.alpha = float(param["alpha"])

        self.name = f"diskann-rs({self.metric})-L{self.l_build}-R{self.max_outdegree}-a{self.alpha}"

        self._index = None
        self._l_search = None

    def fit(self, X):
        # Keep a predictable dtype/layout for the native extension.
        X = np.asarray(X, dtype=np.float32, order="C")
        if X.ndim != 2:
            raise ValueError(f"expected 2D array, got shape={X.shape}")

        from diskann_rs_native import Index

        self._index = Index(
            metric=self.metric,
            l_build=self.l_build,
            max_outdegree=self.max_outdegree,
            alpha=self.alpha,
        )
        self._index.fit(X)

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
        return self._index.search(v, k, l_search)

    def batch_query(self, X, n):
        if self._index is None:
            raise RuntimeError("fit() must be called before batch_query()")

        X = np.asarray(X, dtype=np.float32, order="C")
        if X.ndim != 2:
            raise ValueError(f"expected 2D array, got shape={X.shape}")

        k = int(n)
        l_search = int(self._l_search) if self._l_search is not None else max(k, 10)
        self.res = self._index.batch_search(X, k, l_search)

    def get_batch_results(self):
        return self.res
