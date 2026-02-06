/*
CORRUPTED PRELUDE (generated during scaffolding)

The content below was accidentally concatenated/duplicated and is not valid Rust.
It is intentionally commented out so the real implementation further down can
compile. Once things are stable, this block can be deleted.

































































































































































































































































































}    Ok(())    m.add_class::<Index>()?;fn diskann_rs_native(m: &Bound<'_, PyModule>) -> PyResult<()> {#[pymodule]}    }        Ok(array.into_pyarray(py))            .map_err(|e| PyDiskAnnError::InvalidParameter(e.to_string()))?;        let array = ndarray::Array2::from_shape_vec((n_queries, k), results)        })?;            Ok(all)            }                all.extend_from_slice(&ids);                    .map_err(PyDiskAnnError::Ann)?;                    .search(&FullPrecision, &DefaultContext, query, &search_params, &mut out)                index                let mut out = search_output_buffer::IdDistance::new(&mut ids, &mut distances);                distances.fill(0.0);                ids.fill(0);                let query: &[f32] = &queries[start..end];                let end = start + dim;                let start = qi * dim;            for qi in 0..n_queries {            let mut distances = vec![0.0f32; k];            let mut ids = vec![0u32; k];            let mut all = Vec::with_capacity(n_queries * k);            let search_params = SearchParams::new(k, l_search, None).map_err(ANNError::from)?;        let results = py.allow_threads(move || -> Result<Vec<u32>, PyDiskAnnError> {        let queries: Vec<f32> = xq.iter().copied().collect();        }            )));                xq.shape()[1]                "query dim mismatch: expected {dim}, got {}",            return Err(PyDiskAnnError::InvalidParameter(format!(        if xq.shape()[1] != dim {        let n_queries = xq.shape()[0];        let xq = xq.as_array();        }            ));                "l_search must be >= k".into(),            return Err(PyDiskAnnError::InvalidParameter(        if l_search < k {        }            return Err(PyDiskAnnError::InvalidParameter("k must be > 0".into()));        if k == 0 {        })?;            PyDiskAnnError::InvalidParameter("fit() must be called before batch_search()".into())        let index = self.index.as_ref().ok_or_else(|| {        };            ));                "fit() must be called before batch_search()".into(),            return Err(PyDiskAnnError::InvalidParameter(        let Some(dim) = self.dim else {    ) -> Result<Bound<'py, PyArray2<u32>>, PyDiskAnnError> {        l_search: usize,        k: usize,        xq: PyReadonlyArray2<'py, f32>,        py: Python<'py>,        &self,    fn batch_search<'py>(    }        Ok(ids.into_pyarray(py))        })?;            Ok(ids)                .map_err(PyDiskAnnError::Ann)?;                .search(&FullPrecision, &DefaultContext, query.as_ref(), &search_params, &mut out)            index            let mut out = search_output_buffer::IdDistance::new(&mut ids, &mut distances);            let mut distances = vec![0.0f32; k];            let mut ids = vec![0u32; k];            let search_params = SearchParams::new(k, l_search, None).map_err(ANNError::from)?;        let ids = py.allow_threads(move || -> Result<Vec<u32>, PyDiskAnnError> {        let query: Arc<[f32]> = q.iter().copied().collect::<Vec<_>>().into();        // Copy query so we can release the GIL during search.        }            )));                q.len()                "query dim mismatch: expected {dim}, got {}",            return Err(PyDiskAnnError::InvalidParameter(format!(        if q.len() != dim {        let q = q.as_array();        }            ));                "l_search must be >= k".into(),            return Err(PyDiskAnnError::InvalidParameter(        if l_search < k {        }            return Err(PyDiskAnnError::InvalidParameter("k must be > 0".into()));        if k == 0 {        })?;            PyDiskAnnError::InvalidParameter("fit() must be called before search()".into())        let index = self.index.as_ref().ok_or_else(|| {        };            ));                "fit() must be called before search()".into(),            return Err(PyDiskAnnError::InvalidParameter(        let Some(dim) = self.dim else {    ) -> Result<Bound<'py, PyArray1<u32>>, PyDiskAnnError> {        l_search: usize,        k: usize,        q: PyReadonlyArray1<'py, f32>,        py: Python<'py>,        &self,    fn search<'py>(    }        Ok(())        self.index = Some(built);        self.dim = Some(dim);        })?;            Ok(index)            }                    .map_err(PyDiskAnnError::Ann)?;                    .insert(FullPrecision, &DefaultContext, &(i as u32), row)                index                let row: &[f32] = &data[start..end];                let end = start + dim;                let start = i * dim;            for i in 0..n_points {            // Insert points in order using the identity mapping (external_id == internal_id).            let index = wrapped_async::DiskANNIndex::new_with_multi_thread_runtime(config, provider);                .map_err(PyDiskAnnError::Ann)?;            let provider = DefaultProvider::new_empty(params, fp_precursor, NoStore, NoDeletes)            let fp_precursor = CreateFullPrecision::<f32>::new(dim, None);            };                max_degree: config.max_degree_u32().get(),                prefetch_cache_line_level: None,                prefetch_lookahead: None,                metric,                dim,                frozen_points: diskann::utils::ONE,                max_points: n_points,            let params = DefaultProviderParameters {            .map_err(ANNError::from)?;            .build()            )                },                    b.alpha(alpha);                |b| {                prune_kind,                l_build,                graph_config::MaxDegree::default_slack(),                max_outdegree,            let config = graph_config::Builder::new_with(            let prune_kind = graph_config::PruneKind::from_metric(metric);        let built = py.allow_threads(move || -> Result<_, PyDiskAnnError> {        let alpha = self.alpha;        let max_outdegree = self.max_outdegree;        let l_build = self.l_build;        let metric = self.metric;        let data: Vec<f32> = x.iter().copied().collect();        // Copy data so we can safely release the GIL during index build.        }            ));                "X has too many rows for u32 ids".into(),            return Err(PyDiskAnnError::InvalidParameter(        if n_points > (u32::MAX as usize) {        }            ));                "X must be non-empty".into(),            return Err(PyDiskAnnError::InvalidParameter(        if n_points == 0 || dim == 0 {        let dim = x.shape()[1];        let n_points = x.shape()[0];        let x = x.as_array();    fn fit<'py>(&mut self, py: Python<'py>, x: PyReadonlyArray2<'py, f32>) -> Result<(), PyDiskAnnError> {    }        })            index: None,            dim: None,            alpha,            max_outdegree,            l_build,            metric,        Ok(Self {        }            ));                "alpha must be finite and > 0".into(),            return Err(PyDiskAnnError::InvalidParameter(        if !alpha.is_finite() || alpha <= 0.0 {        }            ));                "max_outdegree must be > 0".into(),            return Err(PyDiskAnnError::InvalidParameter(        if max_outdegree == 0 {        }            return Err(PyDiskAnnError::InvalidParameter("l_build must be > 0".into()));        if l_build == 0 {            .map_err(|_| PyDiskAnnError::InvalidMetric(metric.clone()))?;        let metric = Metric::from_str(&metric)    fn new(metric: String, l_build: usize, max_outdegree: usize, alpha: f32) -> Result<Self, PyDiskAnnError> {    #[new]impl Index {#[pymethods]}    >>,        >,            DefaultContext,            NoDeletes,            NoStore,            f32,        diskann_providers::model::graph::provider::async_::inmem::FullPrecisionProvider<    index: Option<wrapped_async::DiskANNIndex<    dim: Option<usize>,    alpha: f32,    max_outdegree: usize,    l_build: usize,    metric: Metric,struct Index {#[pyclass]}    }        pyo3::exceptions::PyValueError::new_err(err.to_string())    fn from(err: PyDiskAnnError) -> Self {impl From<PyDiskAnnError> for PyErr {}    Ann(#[from] ANNError),    #[error(transparent)]    InvalidParameter(String),    #[error("invalid parameter: {0}")]    InvalidMetric(String),    #[error("invalid metric: {0}")]enum PyDiskAnnError {#[derive(Debug, thiserror::Error)]use diskann_vector::distance::Metric;};    },        inmem::{CreateFullPrecision, DefaultProvider, DefaultProviderParameters},        common::{FullPrecision, NoDeletes, NoStore},    model::graph::provider::async_::{    index::wrapped_async,use diskann_providers::{};    ANNError,    provider::DefaultContext,    graph::{config as graph_config, search_output_buffer, SearchParams},use diskann::{use pyo3::prelude::*;use numpy::{IntoPyArray, PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2};};    sync::Arc,    str::FromStr,    num::NonZeroUsize,
    str::FromStr,
    sync::Arc,
};

*/

#![allow(unsafe_op_in_unsafe_fn)]

use std::{str::FromStr, sync::Arc};

use numpy::{IntoPyArray, PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;

use diskann::{
    graph::{config as graph_config, search_output_buffer, SearchParams},
    provider::DefaultContext,
    ANNError,
};
use diskann_providers::{
    index::wrapped_async,
    model::graph::provider::async_::{
        common::{FullPrecision, NoDeletes, NoStore},
        inmem::{CreateFullPrecision, DefaultProvider, DefaultProviderParameters},
    },
};
use diskann_vector::distance::Metric;

#[derive(Debug, thiserror::Error)]
enum PyDiskAnnError {
    #[error("invalid metric: {0}")]
    InvalidMetric(String),
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),
    #[error(transparent)]
    Ann(#[from] ANNError),
}

impl From<PyDiskAnnError> for PyErr {
    fn from(err: PyDiskAnnError) -> Self {
        pyo3::exceptions::PyValueError::new_err(err.to_string())
    }
}

#[pyclass]
struct Index {
    metric: Metric,
    l_build: usize,
    max_outdegree: usize,
    alpha: f32,

    l_search: usize,
    dim: Option<usize>,

    index: Option<wrapped_async::DiskANNIndex<
        diskann_providers::model::graph::provider::async_::inmem::FullPrecisionProvider<
            f32,
            NoStore,
            NoDeletes,
            DefaultContext,
        >,
    >>,
}

#[pymethods]
impl Index {
    #[new]
    fn new(metric: String, l_build: usize, max_outdegree: usize, alpha: f32) -> Result<Self, PyDiskAnnError> {
        let metric = Metric::from_str(&metric)
            .map_err(|_| PyDiskAnnError::InvalidMetric(metric.clone()))?;

        if l_build == 0 {
            return Err(PyDiskAnnError::InvalidParameter("l_build must be > 0".into()));
        }
        if max_outdegree == 0 {
            return Err(PyDiskAnnError::InvalidParameter(
                "max_outdegree must be > 0".into(),
            ));
        }
        if !alpha.is_finite() || alpha <= 0.0 {
            return Err(PyDiskAnnError::InvalidParameter(
                "alpha must be finite and > 0".into(),
            ));
        }

        Ok(Self {
            metric,
            l_build,
            max_outdegree,
            alpha,
            l_search: 10,
            dim: None,
            index: None,
        })
    }

    fn set_l_search(&mut self, l_search: usize) -> Result<(), PyDiskAnnError> {
        if l_search == 0 {
            return Err(PyDiskAnnError::InvalidParameter(
                "l_search must be > 0".into(),
            ));
        }
        self.l_search = l_search;
        Ok(())
    }

    fn fit<'py>(&mut self, py: Python<'py>, x: PyReadonlyArray2<'py, f32>) -> Result<(), PyDiskAnnError> {
        let x = x.as_array();
        let n_points = x.shape()[0];
        let dim = x.shape()[1];

        if n_points == 0 || dim == 0 {
            return Err(PyDiskAnnError::InvalidParameter(
                "X must be non-empty".into(),
            ));
        }
        if n_points > (u32::MAX as usize) {
            return Err(PyDiskAnnError::InvalidParameter(
                "X has too many rows for u32 ids".into(),
            ));
        }

        // Copy data so we can safely release the GIL during index build.
        let data: Vec<f32> = x.iter().copied().collect();

        let metric = self.metric;
        let l_build = self.l_build;
        let max_outdegree = self.max_outdegree;
        let alpha = self.alpha;

        let built = py.allow_threads(move || -> Result<_, PyDiskAnnError> {
            let prune_kind = graph_config::PruneKind::from_metric(metric);
            let config = graph_config::Builder::new_with(
                max_outdegree,
                graph_config::MaxDegree::default_slack(),
                l_build,
                prune_kind,
                |b| {
                    b.alpha(alpha);
                },
            )
            .build()
            .map_err(ANNError::from)?;

            let params = DefaultProviderParameters {
                max_points: n_points,
                frozen_points: diskann::utils::ONE,
                dim,
                metric,
                prefetch_lookahead: None,
                prefetch_cache_line_level: None,
                max_degree: config.max_degree_u32().get(),
            };

            let fp_precursor = CreateFullPrecision::<f32>::new(dim, None);
            let provider = DefaultProvider::new_empty(params, fp_precursor, NoStore, NoDeletes)
                .map_err(PyDiskAnnError::Ann)?;

            let index = wrapped_async::DiskANNIndex::new_with_multi_thread_runtime(config, provider);

            // Insert points in order using the identity mapping (external_id == internal_id).
            for i in 0..n_points {
                let start = i * dim;
                let end = start + dim;
                let row: &[f32] = &data[start..end];

                index
                    .insert(FullPrecision, &DefaultContext, &(i as u32), row)
                    .map_err(PyDiskAnnError::Ann)?;
            }

            Ok(index)
        })?;

        self.dim = Some(dim);
        self.index = Some(built);

        Ok(())
    }

    fn search<'py>(
        &self,
        py: Python<'py>,
        q: PyReadonlyArray1<'py, f32>,
        k: usize,
        l_search: usize,
    ) -> Result<Bound<'py, PyArray1<u32>>, PyDiskAnnError> {
        let Some(dim) = self.dim else {
            return Err(PyDiskAnnError::InvalidParameter(
                "fit() must be called before search()".into(),
            ));
        };
        let index = self.index.as_ref().ok_or_else(|| {
            PyDiskAnnError::InvalidParameter("fit() must be called before search()".into())
        })?;

        if k == 0 {
            return Err(PyDiskAnnError::InvalidParameter("k must be > 0".into()));
        }
        if l_search < k {
            return Err(PyDiskAnnError::InvalidParameter(
                "l_search must be >= k".into(),
            ));
        }

        let q = q.as_array();
        if q.len() != dim {
            return Err(PyDiskAnnError::InvalidParameter(format!(
                "query dim mismatch: expected {dim}, got {}",
                q.len()
            )));
        }

        // Copy query so we can release the GIL during search.
        let query: Arc<[f32]> = q.iter().copied().collect::<Vec<_>>().into();

        let ids = py.allow_threads(move || -> Result<Vec<u32>, PyDiskAnnError> {
            let search_params = SearchParams::new(k, l_search, None).map_err(ANNError::from)?;

            let mut ids = vec![0u32; k];
            let mut distances = vec![0.0f32; k];
            let mut out = search_output_buffer::IdDistance::new(&mut ids, &mut distances);

            index
                .search(&FullPrecision, &DefaultContext, query.as_ref(), &search_params, &mut out)
                .map_err(PyDiskAnnError::Ann)?;

            Ok(ids)
        })?;

        Ok(ids.into_pyarray_bound(py))
    }

    fn batch_search<'py>(
        &self,
        py: Python<'py>,
        xq: PyReadonlyArray2<'py, f32>,
        k: usize,
        l_search: usize,
    ) -> Result<Bound<'py, PyArray2<u32>>, PyDiskAnnError> {
        let Some(dim) = self.dim else {
            return Err(PyDiskAnnError::InvalidParameter(
                "fit() must be called before batch_search()".into(),
            ));
        };
        let index = self.index.as_ref().ok_or_else(|| {
            PyDiskAnnError::InvalidParameter("fit() must be called before batch_search()".into())
        })?;

        if k == 0 {
            return Err(PyDiskAnnError::InvalidParameter("k must be > 0".into()));
        }
        if l_search < k {
            return Err(PyDiskAnnError::InvalidParameter(
                "l_search must be >= k".into(),
            ));
        }

        let xq = xq.as_array();
        let n_queries = xq.shape()[0];
        if xq.shape()[1] != dim {
            return Err(PyDiskAnnError::InvalidParameter(format!(
                "query dim mismatch: expected {dim}, got {}",
                xq.shape()[1]
            )));
        }

        let queries: Vec<f32> = xq.iter().copied().collect();

        let results = py.allow_threads(move || -> Result<Vec<u32>, PyDiskAnnError> {
            let search_params = SearchParams::new(k, l_search, None).map_err(ANNError::from)?;

            let mut all = Vec::with_capacity(n_queries * k);
            let mut ids = vec![0u32; k];
            let mut distances = vec![0.0f32; k];

            for qi in 0..n_queries {
                let start = qi * dim;
                let end = start + dim;
                let query: &[f32] = &queries[start..end];

                ids.fill(0);
                distances.fill(0.0);
                let mut out = search_output_buffer::IdDistance::new(&mut ids, &mut distances);

                index
                    .search(&FullPrecision, &DefaultContext, query, &search_params, &mut out)
                    .map_err(PyDiskAnnError::Ann)?;

                all.extend_from_slice(&ids);
            }

            Ok(all)
        })?;

        // Shape into (n_queries, k)
        let array = ndarray::Array2::from_shape_vec((n_queries, k), results)
            .map_err(|e| PyDiskAnnError::InvalidParameter(e.to_string()))?;

        Ok(array.into_pyarray_bound(py))
    }
}

#[pymodule]
fn diskann_rs_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Index>()?;
    Ok(())
}
