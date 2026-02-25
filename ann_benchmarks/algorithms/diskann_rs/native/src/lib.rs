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

use std::{
    fs,
    io::Write,
    num::NonZeroUsize,
    path::Path,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use numpy::{IntoPyArray, PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use rayon::prelude::*;

use diskann::{
    graph::{config as graph_config, search_output_buffer, DiskANNIndex, SearchParams},
    provider::DefaultContext,
    ANNError,
};
use diskann_providers::{
    model::IndexConfiguration,
    model::graph::traits::AdHoc,
    model::graph::provider::async_::{
        common::{FullPrecision, Hybrid, NoDeletes, NoStore, Quantized},
        inmem::{
            CreateFullPrecision, DefaultProvider, DefaultProviderParameters, DefaultQuant, SQStore,
            WithBits,
        },
    },
    model::{pq::{self, FixedChunkPQTable, GeneratePivotArguments, NUM_PQ_CENTROIDS, NUM_KMEANS_REPS_PQ}},
    storage::{
        get_disk_index_compressed_pq_file, get_disk_index_file, get_disk_index_pq_pivot_file,
        load_fp_index, load_pq_index, AsyncIndexMetadata, FileStorageProvider, SaveWith,
    },
};
use diskann_vector::distance::Metric;
use diskann_quantization::scalar::train::ScalarQuantizationParameters;
use diskann_utils::views::MatrixView;
use diskann_providers::storage::StorageWriteProvider;

use diskann_disk::{
    disk_index_build_parameter::{MemoryBudget, NumPQChunks},
    DiskIndexBuildParameters, QuantizationType,
};
use diskann_disk::build::builder::build::DiskIndexBuilder;
use diskann_disk::data_model::CachingStrategy;
use diskann_disk::search::provider::disk_provider::DiskIndexSearcher;
use diskann_disk::search::provider::disk_vertex_provider_factory::DiskVertexProviderFactory;
use diskann_disk::storage::DiskIndexWriter;
use diskann_disk::storage::disk_index_reader::DiskIndexReader;
use diskann_disk::utils::aligned_file_reader::AlignedFileReaderFactory;

type IndexType = DiskANNIndex<
    diskann_providers::model::graph::provider::async_::inmem::FullPrecisionProvider<
        f32,
        NoStore,
        NoDeletes,
        DefaultContext,
    >,
>;

type PQIndexType = DiskANNIndex<
    diskann_providers::model::graph::provider::async_::inmem::FullPrecisionProvider<
        f32,
        DefaultQuant,
        NoDeletes,
        DefaultContext,
    >,
>;

type ScalarQuantIndexType<const NBITS: usize> = DiskANNIndex<
    diskann_providers::model::graph::provider::async_::inmem::FullPrecisionProvider<
        f32,
        SQStore<NBITS>,
        NoDeletes,
        DefaultContext,
    >,
>;

#[derive(Clone)]
enum SphericalIndexInner {
    Bits1(Arc<ScalarQuantIndexType<1>>),
    Bits2(Arc<ScalarQuantIndexType<2>>),
    Bits4(Arc<ScalarQuantIndexType<4>>),
}

fn meta_path(prefix: &str) -> PathBuf {
    PathBuf::from(format!("{prefix}.meta.json"))
}

fn pq_meta_path(prefix: &str) -> PathBuf {
    PathBuf::from(format!("{prefix}.pq.meta.json"))
}

fn spherical_meta_path(prefix: &str) -> PathBuf {
    PathBuf::from(format!("{prefix}.spherical.meta.json"))
}

fn pq_disk_meta_path(prefix: &str) -> PathBuf {
    PathBuf::from(format!("{prefix}.pq.disk.meta.json"))
}

fn ensure_parent_dir(prefix: &str) -> Result<(), PyDiskAnnError> {
    let prefix_path = Path::new(prefix);
    if let Some(parent) = prefix_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| PyDiskAnnError::InvalidParameter(format!("create_dir_all failed: {e}")))?;
        }
    }
    Ok(())
}

fn write_meta(
    prefix: &str,
    metric: Metric,
    l_build: usize,
    max_outdegree: usize,
    alpha: f32,
    dim: usize,
    n_points: usize,
    max_degree: u32,
) -> Result<(), PyDiskAnnError> {
    ensure_parent_dir(prefix)?;
    let value = serde_json::json!({
        "metric": metric.to_string(),
        "l_build": l_build,
        "max_outdegree": max_outdegree,
        "alpha": alpha,
        "dim": dim,
        "n_points": n_points,
        "max_degree": max_degree,
    });
    let bytes = serde_json::to_vec_pretty(&value)
        .map_err(|e| PyDiskAnnError::InvalidParameter(format!("serialize meta failed: {e}")))?;
    fs::write(meta_path(prefix), bytes)
        .map_err(|e| PyDiskAnnError::InvalidParameter(format!("write meta failed: {e}")))?;
    Ok(())
}

fn read_meta(prefix: &str) -> Result<(Metric, usize, usize, f32, usize, usize, u32), PyDiskAnnError> {
    let bytes = fs::read(meta_path(prefix))
        .map_err(|e| PyDiskAnnError::InvalidParameter(format!("read meta failed: {e}")))?;
    let v: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| PyDiskAnnError::InvalidParameter(format!("parse meta failed: {e}")))?;

    let metric_str = v
        .get("metric")
        .and_then(|x| x.as_str())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.metric missing".into()))?;
    let metric = Metric::from_str(metric_str)
        .map_err(|_| PyDiskAnnError::InvalidMetric(metric_str.to_string()))?;

    let l_build = v
        .get("l_build")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.l_build missing".into()))?
        as usize;
    let max_outdegree = v
        .get("max_outdegree")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.max_outdegree missing".into()))?
        as usize;
    let alpha = v
        .get("alpha")
        .and_then(|x| x.as_f64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.alpha missing".into()))?
        as f32;
    let dim = v
        .get("dim")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.dim missing".into()))?
        as usize;
    let n_points = v
        .get("n_points")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.n_points missing".into()))?
        as usize;
    let max_degree = v
        .get("max_degree")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.max_degree missing".into()))?
        as u32;

    Ok((metric, l_build, max_outdegree, alpha, dim, n_points, max_degree))
}

fn write_kind_meta(
    path: PathBuf,
    prefix: &str,
    kind: &str,
    metric: Metric,
    l_build: usize,
    max_outdegree: usize,
    alpha: f32,
    dim: usize,
    n_points: usize,
    max_degree: u32,
    extra: serde_json::Value,
) -> Result<(), PyDiskAnnError> {
    ensure_parent_dir(prefix)?;
    let mut obj = serde_json::json!({
        "kind": kind,
        "metric": metric.to_string(),
        "l_build": l_build,
        "max_outdegree": max_outdegree,
        "alpha": alpha,
        "dim": dim,
        "n_points": n_points,
        "max_degree": max_degree,
    });

    if let (Some(base), serde_json::Value::Object(extra_obj)) = (obj.as_object_mut(), extra) {
        for (k, v) in extra_obj {
            base.insert(k, v);
        }
    }

    let bytes = serde_json::to_vec_pretty(&obj)
        .map_err(|e| PyDiskAnnError::InvalidParameter(format!("serialize meta failed: {e}")))?;
    fs::write(path, bytes)
        .map_err(|e| PyDiskAnnError::InvalidParameter(format!("write meta failed: {e}")))?;
    Ok(())
}

fn read_kind_meta(
    path: PathBuf,
) -> Result<(String, Metric, usize, usize, f32, usize, usize, u32, serde_json::Value), PyDiskAnnError>
{
    let bytes = fs::read(&path)
        .map_err(|e| PyDiskAnnError::InvalidParameter(format!("read meta failed: {e}")))?;
    let v: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| PyDiskAnnError::InvalidParameter(format!("parse meta failed: {e}")))?;

    let kind = v
        .get("kind")
        .and_then(|x| x.as_str())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.kind missing".into()))?
        .to_string();

    let metric_str = v
        .get("metric")
        .and_then(|x| x.as_str())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.metric missing".into()))?;
    let metric = Metric::from_str(metric_str)
        .map_err(|_| PyDiskAnnError::InvalidMetric(metric_str.to_string()))?;

    let l_build = v
        .get("l_build")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.l_build missing".into()))?
        as usize;
    let max_outdegree = v
        .get("max_outdegree")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.max_outdegree missing".into()))?
        as usize;
    let alpha = v
        .get("alpha")
        .and_then(|x| x.as_f64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.alpha missing".into()))?
        as f32;
    let dim = v
        .get("dim")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.dim missing".into()))?
        as usize;
    let n_points = v
        .get("n_points")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.n_points missing".into()))?
        as usize;
    let max_degree = v
        .get("max_degree")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.max_degree missing".into()))?
        as u32;

    Ok((kind, metric, l_build, max_outdegree, alpha, dim, n_points, max_degree, v))
}

fn write_pq_disk_meta(
    prefix: &str,
    metric: Metric,
    l_build: usize,
    max_outdegree: usize,
    alpha: f32,
    dim: usize,
    n_points: usize,
    max_degree: u32,
    num_pq_chunks: usize,
    build_memory_gb: f64,
    search_pq_chunks: usize,
    search_io_limit: usize,
    build_quantization: QuantizationType,
) -> Result<(), PyDiskAnnError> {
    write_kind_meta(
        pq_disk_meta_path(prefix),
        prefix,
        "pq_disk",
        metric,
        l_build,
        max_outdegree,
        alpha,
        dim,
        n_points,
        max_degree,
        serde_json::json!({
            "num_pq_chunks": num_pq_chunks,
            "build_memory_gb": build_memory_gb,
            "search_pq_chunks": search_pq_chunks,
            "search_io_limit": search_io_limit,
            "build_quantization": build_quantization.to_string(),
        }),
    )
}

fn read_pq_disk_meta(
    prefix: &str,
) -> Result<(Metric, usize, usize, f32, usize, usize, u32, usize, f64, usize, usize, QuantizationType), PyDiskAnnError>
{
    let (_kind, metric, l_build, max_outdegree, alpha, dim, n_points, max_degree, extra) =
        read_kind_meta(pq_disk_meta_path(prefix))?;

    let num_pq_chunks = extra
        .get("num_pq_chunks")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.num_pq_chunks missing".into()))?
        as usize;
    let build_memory_gb = extra
        .get("build_memory_gb")
        .and_then(|x| x.as_f64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.build_memory_gb missing".into()))?;
    let search_pq_chunks = extra
        .get("search_pq_chunks")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.search_pq_chunks missing".into()))?
        as usize;
    let search_io_limit = extra
        .get("search_io_limit")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.search_io_limit missing".into()))?
        as usize;
    let build_quantization_str = extra
        .get("build_quantization")
        .and_then(|x| x.as_str())
        .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.build_quantization missing".into()))?;
    let build_quantization = QuantizationType::from_str(build_quantization_str).map_err(|e| {
        PyDiskAnnError::InvalidParameter(format!("invalid meta.build_quantization: {e}"))
    })?;

    Ok((
        metric,
        l_build,
        max_outdegree,
        alpha,
        dim,
        n_points,
        max_degree,
        num_pq_chunks,
        build_memory_gb,
        search_pq_chunks,
        search_io_limit,
        build_quantization,
    ))
}

type DiskData = AdHoc<f32, u32>;

#[pyclass]
struct PQDiskIndex {
    metric: Metric,
    l_build: usize,
    max_outdegree: usize,
    alpha: f32,
    num_pq_chunks: usize,

    build_memory_gb: f64,
    search_pq_chunks: usize,
    search_io_limit: usize,
    build_quantization: QuantizationType,
    block_size: usize,

    l_search: usize,
    dim: Option<usize>,
    n_points: Option<usize>,
    searcher: Option<Arc<DiskIndexSearcher<DiskData, DiskVertexProviderFactory<DiskData, AlignedFileReaderFactory>>>>,
}

#[pymethods]
impl PQDiskIndex {
    #[new]
    #[pyo3(signature = (
        metric,
        l_build,
        max_outdegree,
        alpha,
        num_pq_chunks,
        build_memory_gb=16.0,
        search_pq_chunks=None,
        search_io_limit=1_000_000,
        build_quantization=None,
        block_size=4096,
    ))]
    fn new(
        metric: String,
        l_build: usize,
        max_outdegree: usize,
        alpha: f32,
        num_pq_chunks: usize,
        build_memory_gb: f64,
        search_pq_chunks: Option<usize>,
        search_io_limit: usize,
        build_quantization: Option<String>,
        block_size: usize,
    ) -> Result<Self, PyDiskAnnError> {
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
        if !build_memory_gb.is_finite() || build_memory_gb <= 0.0 {
            return Err(PyDiskAnnError::InvalidParameter(
                "build_memory_gb must be finite and > 0".into(),
            ));
        }
        if num_pq_chunks == 0 {
            return Err(PyDiskAnnError::InvalidParameter(
                "num_pq_chunks must be > 0".into(),
            ));
        }
        if search_io_limit == 0 {
            return Err(PyDiskAnnError::InvalidParameter(
                "search_io_limit must be > 0".into(),
            ));
        }

        let search_pq_chunks = search_pq_chunks.unwrap_or(num_pq_chunks);
        if search_pq_chunks == 0 {
            return Err(PyDiskAnnError::InvalidParameter(
                "search_pq_chunks must be > 0".into(),
            ));
        }

        let build_quantization = match build_quantization {
            Some(s) => QuantizationType::from_str(&s)
                .map_err(|e| PyDiskAnnError::InvalidParameter(format!("invalid build_quantization: {e}")))?,
            None => QuantizationType::PQ { num_chunks: num_pq_chunks },
        };

        Ok(Self {
            metric,
            l_build,
            max_outdegree,
            alpha,
            num_pq_chunks,
            build_memory_gb,
            search_pq_chunks,
            search_io_limit,
            build_quantization,
            block_size,
            l_search: 10,
            dim: None,
            n_points: None,
            searcher: None,
        })
    }

    #[staticmethod]
    fn load(prefix: String) -> Result<Self, PyDiskAnnError> {
        let (
            metric,
            l_build,
            max_outdegree,
            alpha,
            dim,
            n_points,
            _max_degree,
            num_pq_chunks,
            build_memory_gb,
            search_pq_chunks,
            search_io_limit,
            build_quantization,
        ) = read_pq_disk_meta(&prefix)?;

        let mut this = Self::new(
            metric.to_string(),
            l_build,
            max_outdegree,
            alpha,
            num_pq_chunks,
            build_memory_gb,
            Some(search_pq_chunks),
            search_io_limit,
            Some(build_quantization.to_string()),
            4096,
        )?;

        this.dim = Some(dim);
        this.n_points = Some(n_points);
        this.searcher = Some(Arc::new(Self::create_searcher(&prefix, metric, search_io_limit)?));
        Ok(this)
    }

    fn set_l_search(&mut self, l_search: usize) {
        self.l_search = l_search;
    }

    fn fit<'py>(
        &mut self,
        _py: Python<'py>,
        x: PyReadonlyArray2<'py, f32>,
        prefix: String,
    ) -> Result<(), PyDiskAnnError> {
        let x = x.as_array();
        let n_points = x.shape()[0];
        let dim = x.shape()[1];
        if n_points == 0 || dim == 0 {
            return Err(PyDiskAnnError::InvalidParameter("X must be non-empty".into()));
        }
        if n_points > (u32::MAX as usize) {
            return Err(PyDiskAnnError::InvalidParameter(
                "X has too many rows for u32 ids".into(),
            ));
        }

        ensure_parent_dir(&prefix)?;

        // Write dataset to a DiskANN-compatible .bin file (u32 npts, u32 dim, then row-major f32 data).
        let dataset_path = format!("{prefix}.disk.dataset.bin");
        {
            let mut w = FileStorageProvider
                .create_for_write(&dataset_path)
                .map_err(|e| PyDiskAnnError::InvalidParameter(format!("create dataset file failed: {e}")))?;
            w.write_all(&(n_points as u32).to_le_bytes())
                .map_err(|e| PyDiskAnnError::InvalidParameter(format!("write dataset header failed: {e}")))?;
            w.write_all(&(dim as u32).to_le_bytes())
                .map_err(|e| PyDiskAnnError::InvalidParameter(format!("write dataset header failed: {e}")))?;

            let flat: Vec<f32> = x.iter().copied().collect();
            w.write_all(bytemuck::cast_slice(&flat))
                .map_err(|e| PyDiskAnnError::InvalidParameter(format!("write dataset body failed: {e}")))?;
            w.flush()
                .map_err(|e| PyDiskAnnError::InvalidParameter(format!("flush dataset failed: {e}")))?;
        }

        let prune_kind = graph_config::PruneKind::from_metric(self.metric);
        let config = graph_config::Builder::new_with(
            self.max_outdegree,
            graph_config::MaxDegree::default_slack(),
            self.l_build,
            prune_kind,
            |b| {
                b.alpha(self.alpha);
            },
        )
        .build()
        .map_err(ANNError::from)
        .map_err(PyDiskAnnError::Ann)?;

        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let index_config = IndexConfiguration::new(
            self.metric,
            dim,
            n_points,
            diskann::utils::ONE,
            num_threads,
            config,
        );

        let mem_budget = MemoryBudget::try_from_gb(self.build_memory_gb)
            .map_err(ANNError::from)
            .map_err(PyDiskAnnError::Ann)?;
        let pq_chunks = NumPQChunks::new_with(self.search_pq_chunks, dim)
            .map_err(ANNError::from)
            .map_err(PyDiskAnnError::Ann)?;

        let disk_build_param = DiskIndexBuildParameters::new(mem_budget, self.build_quantization, pq_chunks);

        let index_writer = DiskIndexWriter::new(
            dataset_path,
            prefix.clone(),
            None,
            self.block_size,
        )
        .map_err(PyDiskAnnError::Ann)?;

        let mut builder = DiskIndexBuilder::<DiskData, _>::new(
            &FileStorageProvider,
            disk_build_param,
            index_config,
            index_writer,
        )
        .map_err(PyDiskAnnError::Ann)?;

        builder.build().map_err(PyDiskAnnError::Ann)?;

        // Use the disk index file header's max-degree for metadata.
        let max_degree = self.max_outdegree as u32;
        write_pq_disk_meta(
            &prefix,
            self.metric,
            self.l_build,
            self.max_outdegree,
            self.alpha,
            dim,
            n_points,
            max_degree,
            self.num_pq_chunks,
            self.build_memory_gb,
            self.search_pq_chunks,
            self.search_io_limit,
            self.build_quantization,
        )?;

        self.dim = Some(dim);
        self.n_points = Some(n_points);
        self.searcher = Some(Arc::new(Self::create_searcher(&prefix, self.metric, self.search_io_limit)?));
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
        let searcher = self.searcher.as_ref().ok_or_else(|| {
            PyDiskAnnError::InvalidParameter("fit() must be called before search()".into())
        })?;

        if k == 0 {
            return Err(PyDiskAnnError::InvalidParameter("k must be > 0".into()));
        }
        if l_search < k {
            return Err(PyDiskAnnError::InvalidParameter("l_search must be >= k".into()));
        }

        let q = q.as_array();
        if q.len() != dim {
            return Err(PyDiskAnnError::InvalidParameter(format!(
                "query dim mismatch: expected {dim}, got {}",
                q.len()
            )));
        }

        let query: Vec<f32> = q.iter().copied().collect();
        let ids = py.allow_threads(|| -> Result<Vec<u32>, PyDiskAnnError> {
            let res = searcher
                .search(&query, k as u32, l_search as u32, None, None, false)
                .map_err(PyDiskAnnError::Ann)?;
            Ok(res.results.into_iter().map(|r| r.vertex_id).collect())
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
        let searcher = self.searcher.as_ref().ok_or_else(|| {
            PyDiskAnnError::InvalidParameter("fit() must be called before batch_search()".into())
        })?;

        if k == 0 {
            return Err(PyDiskAnnError::InvalidParameter("k must be > 0".into()));
        }
        if l_search < k {
            return Err(PyDiskAnnError::InvalidParameter("l_search must be >= k".into()));
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
        let searcher = searcher.clone();
        let results = py.allow_threads(move || -> Result<Vec<u32>, PyDiskAnnError> {
            let mut all = vec![0u32; n_queries * k];
            all.par_chunks_mut(k)
                .enumerate()
                .try_for_each(|(qi, out_slice)| {
                    let start = qi * dim;
                    let end = start + dim;
                    let query: &[f32] = &queries[start..end];
                    let res = searcher
                        .search(query, k as u32, l_search as u32, None, None, false)
                        .map_err(PyDiskAnnError::Ann)?;
                    for (dst, item) in out_slice.iter_mut().zip(res.results.iter()) {
                        *dst = item.vertex_id;
                    }
                    Ok::<(), PyDiskAnnError>(())
                })?;
            Ok(all)
        })?;

        let array = ndarray::Array2::from_shape_vec((n_queries, k), results)
            .map_err(|e| PyDiskAnnError::InvalidParameter(e.to_string()))?;
        Ok(array.into_pyarray_bound(py))
    }
}

impl PQDiskIndex {
    fn create_searcher(
        prefix: &str,
        metric: Metric,
        search_io_limit: usize,
    ) -> Result<DiskIndexSearcher<DiskData, DiskVertexProviderFactory<DiskData, AlignedFileReaderFactory>>, PyDiskAnnError>
    {
        let disk_index_file = get_disk_index_file(prefix);
        let pq_pivot_file = get_disk_index_pq_pivot_file(prefix);
        let pq_compressed_file = get_disk_index_compressed_pq_file(prefix);

        let disk_index_reader = DiskIndexReader::<f32>::new::<FileStorageProvider>(
            pq_pivot_file,
            pq_compressed_file,
            &FileStorageProvider,
        )
        .map_err(PyDiskAnnError::Ann)?;

        let reader_factory = AlignedFileReaderFactory::new(disk_index_file);
        let vertex_provider_factory = DiskVertexProviderFactory::<DiskData, _>::new(
            reader_factory,
            CachingStrategy::None,
        )
        .map_err(PyDiskAnnError::Ann)?;

        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| PyDiskAnnError::InvalidParameter(format!("failed to create tokio runtime: {e}")))?;

        DiskIndexSearcher::<DiskData, _>::new(
            num_threads,
            search_io_limit,
            &disk_index_reader,
            vertex_provider_factory,
            metric,
            Some(rt),
        )
        .map_err(PyDiskAnnError::Ann)
    }
}

fn env_num_threads() -> Option<usize> {
    let s = std::env::var("DISKANN_RS_NUM_THREADS").ok()?;
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    match s.parse::<usize>() {
        Ok(n) if n > 0 => Some(n),
        _ => None,
    }
}

fn default_num_threads() -> usize {
    env_num_threads()
        .or_else(|| std::thread::available_parallelism().ok().map(|n| n.get()))
        .unwrap_or(1)
}

fn env_fit_batch_size() -> Option<usize> {
    let s = std::env::var("DISKANN_RS_FIT_BATCH_SIZE").ok()?;
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    match s.parse::<usize>() {
        Ok(n) if n > 0 => Some(n),
        _ => None,
    }
}

fn maybe_report_fit_progress(
    label: &str,
    i: usize,
    n_points: usize,
    t0: &std::time::Instant,
    last_report: &mut std::time::Instant,
) {
    // Print at most once every ~5 seconds, and only after some progress.
    if (i + 1) % 1_000 != 0 {
        return;
    }
    let now = std::time::Instant::now();
    if now.duration_since(*last_report).as_secs_f64() < 5.0 {
        return;
    }

    let done = i + 1;
    let elapsed = now.duration_since(*t0).as_secs_f64();
    let rate = (done as f64) / elapsed.max(1e-9);
    println!(
        "[diskann_rs_native] {label} fit progress: {done}/{n_points} ({rate:.1} vec/s, elapsed={elapsed:.1}s)"
    );
    let _ = std::io::stdout().flush();
    *last_report = now;
}

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
    n_points: Option<usize>,

    rt: tokio::runtime::Runtime,

    index: Option<Arc<IndexType>>,
}

#[pyclass]
struct PQIndex {
    metric: Metric,
    l_build: usize,
    max_outdegree: usize,
    alpha: f32,

    l_search: usize,
    dim: Option<usize>,
    n_points: Option<usize>,

    num_pq_chunks: usize,
    num_centers: usize,
    max_k_means_reps: usize,
    translate_to_center: bool,
    rng_seed: u64,

    rt: tokio::runtime::Runtime,
    index: Option<Arc<PQIndexType>>,
}

#[pyclass]
struct SphericalIndex {
    metric: Metric,
    l_build: usize,
    max_outdegree: usize,
    alpha: f32,

    l_search: usize,
    dim: Option<usize>,
    n_points: Option<usize>,

    nbits: usize,
    rng_seed: u64,

    rt: tokio::runtime::Runtime,
    index: Option<SphericalIndexInner>,
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

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| PyDiskAnnError::InvalidParameter(format!("failed to create tokio runtime: {e}")))?;

        Ok(Self {
            metric,
            l_build,
            max_outdegree,
            alpha,
            l_search: 10,
            dim: None,
            n_points: None,
            rt,
            index: None,
        })
    }

    #[staticmethod]
    fn load(prefix: String) -> Result<Self, PyDiskAnnError> {
        let (metric, l_build, max_outdegree, alpha, dim, n_points, max_degree) = read_meta(&prefix)?;

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| PyDiskAnnError::InvalidParameter(format!("failed to create tokio runtime: {e}")))?;
        let handle = rt.handle().clone();

        let prune_kind = graph_config::PruneKind::from_metric(metric);
        let config = graph_config::Builder::new_with(
            max_outdegree,
            graph_config::MaxDegree::new(max_degree as usize),
            l_build,
            prune_kind,
            |b| {
                b.alpha(alpha);
            },
        )
        .build()
        .map_err(ANNError::from)
        .map_err(PyDiskAnnError::Ann)?;

        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let index_config = IndexConfiguration::new(
            metric,
            dim,
            n_points,
            diskann::utils::ONE,
            num_threads,
            config,
        );

        let index = handle
            .block_on(async {
                load_fp_index::<f32, _, NoStore>(&FileStorageProvider, &prefix, index_config)
                    .await
            })
            .map_err(PyDiskAnnError::Ann)?;

        Ok(Self {
            metric,
            l_build,
            max_outdegree,
            alpha,
            l_search: 10,
            dim: Some(dim),
            n_points: Some(n_points),
            rt,
            index: Some(Arc::new(index)),
        })
    }

    fn save(&self, prefix: String) -> Result<(), PyDiskAnnError> {
        let Some(dim) = self.dim else {
            return Err(PyDiskAnnError::InvalidParameter(
                "fit() must be called before save()".into(),
            ));
        };
        let Some(n_points) = self.n_points else {
            return Err(PyDiskAnnError::InvalidParameter(
                "fit() must be called before save()".into(),
            ));
        };
        let index = self.index.as_ref().ok_or_else(|| {
            PyDiskAnnError::InvalidParameter("fit() must be called before save()".into())
        })?;

        ensure_parent_dir(&prefix)?;

        let prune_kind = graph_config::PruneKind::from_metric(self.metric);
        let config = graph_config::Builder::new_with(
            self.max_outdegree,
            graph_config::MaxDegree::default_slack(),
            self.l_build,
            prune_kind,
            |b| {
                b.alpha(self.alpha);
            },
        )
        .build()
        .map_err(ANNError::from)
        .map_err(PyDiskAnnError::Ann)?;

        let max_degree = config.max_degree_u32().get();

        let handle = self.rt.handle().clone();
        let index = index.clone();
        handle
            .block_on(async {
                let metadata = AsyncIndexMetadata::new(&prefix);
                index
                    .save_with(&FileStorageProvider, &metadata)
                    .await
                    .map_err(PyDiskAnnError::Ann)?;
                Ok::<(), PyDiskAnnError>(())
            })?;

        write_meta(
            &prefix,
            self.metric,
            self.l_build,
            self.max_outdegree,
            self.alpha,
            dim,
            n_points,
            max_degree,
        )?;

        Ok(())
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

        // Build uses multi_insert; parallelism is controlled by both the tokio runtime and
        // the graph config's max_minibatch_par.
        let num_threads = default_num_threads();
        // Chosen to balance overhead and parallelism. This is intentionally conservative
        // (multi_insert is documented as optimized for "small" batches).
        // Can be overridden via env var DISKANN_RS_FIT_BATCH_SIZE.
        let fit_batch_size = env_fit_batch_size().unwrap_or(20_000);

        let prune_kind = graph_config::PruneKind::from_metric(self.metric);
        let config = graph_config::Builder::new_with(
            self.max_outdegree,
            graph_config::MaxDegree::default_slack(),
            self.l_build,
            prune_kind,
            |b| {
                b.alpha(self.alpha);
                b.max_minibatch_par(num_threads);
            },
        )
        .build()
        .map_err(ANNError::from)
        .map_err(PyDiskAnnError::Ann)?;

        let params = DefaultProviderParameters {
            max_points: n_points,
            frozen_points: diskann::utils::ONE,
            dim,
            metric: self.metric,
            prefetch_lookahead: None,
            prefetch_cache_line_level: None,
            max_degree: config.max_degree_u32().get(),
        };

        let fp_precursor = CreateFullPrecision::<f32>::new(dim, None);
        let provider = DefaultProvider::new_empty(params, fp_precursor, NoStore, NoDeletes)
            .map_err(PyDiskAnnError::Ann)?;

        let index = Arc::new(IndexType::new(
            config,
            provider,
            NonZeroUsize::new(num_threads),
        ));

        println!(
            "[diskann_rs_native] fp fit insert start: n_points={n_points} dim={dim} threads={num_threads} batch={fit_batch_size}"
        );
        let _ = std::io::stdout().flush();
        let handle = self.rt.handle().clone();
        let index_clone = index.clone();

        let t0 = std::time::Instant::now();
        let mut last_report = t0;
        let mut start_i = 0usize;
        while start_i < n_points {
            let end_i = (start_i + fit_batch_size).min(n_points);

            let mut vectors: Vec<diskann::utils::async_tools::VectorIdBoxSlice<u32, f32>> =
                Vec::with_capacity(end_i - start_i);
            for i in start_i..end_i {
                let row = x.row(i);
                vectors.push(diskann::utils::async_tools::VectorIdBoxSlice::new(
                    i as u32,
                    row.to_vec().into_boxed_slice(),
                ));
            }
            let vectors = vectors.into_boxed_slice();

            // Release the GIL while we do the heavy async work.
            let handle2 = handle.clone();
            let index2 = index_clone.clone();
            py.allow_threads(move || -> Result<(), PyDiskAnnError> {
                handle2.block_on(async {
                    index2
                        .multi_insert(FullPrecision, &DefaultContext, vectors)
                        .await
                        .map_err(PyDiskAnnError::Ann)
                })
            })?;

            // Update progress based on total inserted so far.
            maybe_report_fit_progress("fp", end_i - 1, n_points, &t0, &mut last_report);
            start_i = end_i;
        }

        self.dim = Some(dim);
        self.n_points = Some(n_points);
        self.index = Some(index);

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

        let handle = self.rt.handle().clone();
        let index = index.clone();

        let ids = py.allow_threads(move || -> Result<Vec<u32>, PyDiskAnnError> {
            handle
                .block_on(async {
                    let search_params = SearchParams::new(k, l_search, None).map_err(ANNError::from)?;

                    let mut ids = vec![0u32; k];
                    let mut distances = vec![0.0f32; k];
                    let mut out = search_output_buffer::IdDistance::new(&mut ids, &mut distances);

                    index
                        .search(
                            &FullPrecision,
                            &DefaultContext,
                            query.as_ref(),
                            &search_params,
                            &mut out,
                        )
                        .await
                        .map_err(PyDiskAnnError::Ann)?;

                    Ok::<Vec<u32>, PyDiskAnnError>(ids)
                })
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

        let handle = self.rt.handle().clone();
        let index = index.clone();
        let results = py.allow_threads(move || -> Result<Vec<u32>, PyDiskAnnError> {
            let search_params = SearchParams::new(k, l_search, None).map_err(ANNError::from)?;
            
            let mut all = vec![0u32; n_queries * k];
            
            all.par_chunks_mut(k).enumerate().try_for_each(|(qi, out_slice)| {
                let start = qi * dim;
                let end = start + dim;
                let query: &[f32] = &queries[start..end];

                let mut distances = vec![0.0f32; k];
                let mut out = search_output_buffer::IdDistance::new(out_slice, &mut distances);

                handle.block_on(async {
                    index
                        .search(&FullPrecision, &DefaultContext, query, &search_params, &mut out)
                        .await
                }).map_err(PyDiskAnnError::Ann)?;
                Ok::<(), PyDiskAnnError>(())
            })?;

            Ok(all)
        })?;

        // Shape into (n_queries, k)
        let array = ndarray::Array2::from_shape_vec((n_queries, k), results)
            .map_err(|e| PyDiskAnnError::InvalidParameter(e.to_string()))?;

        Ok(array.into_pyarray_bound(py))
    }
}

#[pymethods]
impl PQIndex {
    #[new]
    fn new(
        metric: String,
        l_build: usize,
        max_outdegree: usize,
        alpha: f32,
        num_pq_chunks: usize,
        num_centers: Option<usize>,
        max_k_means_reps: Option<usize>,
        translate_to_center: Option<bool>,
        rng_seed: Option<u64>,
    ) -> Result<Self, PyDiskAnnError> {
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
        if num_pq_chunks == 0 {
            return Err(PyDiskAnnError::InvalidParameter(
                "num_pq_chunks must be > 0".into(),
            ));
        }

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| PyDiskAnnError::InvalidParameter(format!("failed to create tokio runtime: {e}")))?;

        Ok(Self {
            metric,
            l_build,
            max_outdegree,
            alpha,
            l_search: 10,
            dim: None,
            n_points: None,
            num_pq_chunks,
            num_centers: num_centers.unwrap_or(NUM_PQ_CENTROIDS),
            max_k_means_reps: max_k_means_reps.unwrap_or(NUM_KMEANS_REPS_PQ),
            translate_to_center: translate_to_center.unwrap_or(true),
            rng_seed: rng_seed.unwrap_or(0),
            rt,
            index: None,
        })
    }

    #[staticmethod]
    fn load(prefix: String) -> Result<Self, PyDiskAnnError> {
        let (kind, metric, l_build, max_outdegree, alpha, dim, n_points, max_degree, meta) = {
            let (kind, metric, l_build, max_outdegree, alpha, dim, n_points, max_degree, raw) =
                read_kind_meta(pq_meta_path(&prefix))?;
            (kind, metric, l_build, max_outdegree, alpha, dim, n_points, max_degree, raw)
        };

        if kind != "pq" {
            return Err(PyDiskAnnError::InvalidParameter(format!(
                "expected pq meta, got kind={kind}"
            )));
        }

        let num_pq_chunks = meta
            .get("num_pq_chunks")
            .and_then(|x| x.as_u64())
            .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.num_pq_chunks missing".into()))?
            as usize;
        let num_centers = meta
            .get("num_centers")
            .and_then(|x| x.as_u64())
            .unwrap_or(NUM_PQ_CENTROIDS as u64) as usize;
        let max_k_means_reps = meta
            .get("max_k_means_reps")
            .and_then(|x| x.as_u64())
            .unwrap_or(NUM_KMEANS_REPS_PQ as u64) as usize;
        let translate_to_center = meta
            .get("translate_to_center")
            .and_then(|x| x.as_bool())
            .unwrap_or(true);
        let rng_seed = meta.get("rng_seed").and_then(|x| x.as_u64()).unwrap_or(0);
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| PyDiskAnnError::InvalidParameter(format!("failed to create tokio runtime: {e}")))?;
        let handle = rt.handle().clone();

        let prune_kind = graph_config::PruneKind::from_metric(metric);
        let config = graph_config::Builder::new_with(
            max_outdegree,
            graph_config::MaxDegree::new(max_degree as usize),
            l_build,
            prune_kind,
            |b| {
                b.alpha(alpha);
            },
        )
        .build()
        .map_err(ANNError::from)
        .map_err(PyDiskAnnError::Ann)?;

        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let index_config = IndexConfiguration::new(
            metric,
            dim,
            n_points,
            diskann::utils::ONE,
            num_threads,
            config,
        );

        let index = handle
            .block_on(async { load_pq_index::<f32, _>(&FileStorageProvider, &prefix, index_config).await })
            .map_err(PyDiskAnnError::Ann)?;

        Ok(Self {
            metric,
            l_build,
            max_outdegree,
            alpha,
            l_search: 10,
            dim: Some(dim),
            n_points: Some(n_points),
            num_pq_chunks,
            num_centers,
            max_k_means_reps,
            translate_to_center,
            rng_seed,
            rt,
            index: Some(Arc::new(index)),
        })
    }

    fn save(&self, prefix: String) -> Result<(), PyDiskAnnError> {
        let Some(dim) = self.dim else {
            return Err(PyDiskAnnError::InvalidParameter(
                "fit() must be called before save()".into(),
            ));
        };
        let Some(n_points) = self.n_points else {
            return Err(PyDiskAnnError::InvalidParameter(
                "fit() must be called before save()".into(),
            ));
        };
        let index = self.index.as_ref().ok_or_else(|| {
            PyDiskAnnError::InvalidParameter("fit() must be called before save()".into())
        })?;

        ensure_parent_dir(&prefix)?;

        let prune_kind = graph_config::PruneKind::from_metric(self.metric);
        let config = graph_config::Builder::new_with(
            self.max_outdegree,
            graph_config::MaxDegree::default_slack(),
            self.l_build,
            prune_kind,
            |b| {
                b.alpha(self.alpha);
            },
        )
        .build()
        .map_err(ANNError::from)
        .map_err(PyDiskAnnError::Ann)?;

        let max_degree = config.max_degree_u32().get();
        let handle = self.rt.handle().clone();
        let index = index.clone();
        handle
            .block_on(async {
                let metadata = AsyncIndexMetadata::new(&prefix);
                index
                    .save_with(&FileStorageProvider, &metadata)
                    .await
                    .map_err(PyDiskAnnError::Ann)?;
                Ok::<(), PyDiskAnnError>(())
            })?;

        write_kind_meta(
            pq_meta_path(&prefix),
            &prefix,
            "pq",
            self.metric,
            self.l_build,
            self.max_outdegree,
            self.alpha,
            dim,
            n_points,
            max_degree,
            serde_json::json!({
                "num_pq_chunks": self.num_pq_chunks,
                "num_centers": self.num_centers,
                "max_k_means_reps": self.max_k_means_reps,
                "translate_to_center": self.translate_to_center,
                "rng_seed": self.rng_seed,
            }),
        )?;

        Ok(())
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
            return Err(PyDiskAnnError::InvalidParameter("X must be non-empty".into()));
        }
        if n_points > (u32::MAX as usize) {
            return Err(PyDiskAnnError::InvalidParameter(
                "X has too many rows for u32 ids".into(),
            ));
        }
        if self.num_pq_chunks > dim {
            return Err(PyDiskAnnError::InvalidParameter(format!(
                "num_pq_chunks ({}) must be <= dim ({dim})",
                self.num_pq_chunks
            )));
        }

        let data: Vec<f32> = x.iter().copied().collect();

        // Can be overridden via env var DISKANN_RS_FIT_BATCH_SIZE.
        // Keep consistent with the FP index default.
        let fit_batch_size = env_fit_batch_size().unwrap_or(20_000);

        // Train PQ pivots from a subset of data.
        let num_train = std::cmp::min(n_points, 50_000);
        let train_data = &data[..num_train * dim];
        let args = GeneratePivotArguments::new(
            num_train,
            dim,
            self.num_centers,
            self.num_pq_chunks,
            self.max_k_means_reps,
            self.translate_to_center,
        )
        .map_err(|e| PyDiskAnnError::InvalidParameter(format!("invalid PQ args: {e}")))?;

        let mut centroid = vec![0.0f32; dim];
        let mut offsets = vec![0usize; self.num_pq_chunks + 1];
        let mut pivots = vec![0.0f32; self.num_centers * dim];
        let mut rng = StdRng::seed_from_u64(self.rng_seed);
        let mut cancel = false;

        pq::generate_pq_pivots_from_membuf(
            &args,
            train_data,
            &mut centroid,
            &mut offsets,
            &mut pivots,
            &mut rng,
            &mut cancel,
            default_num_threads(),
        )
        .map_err(PyDiskAnnError::Ann)?;

        let pq_table = FixedChunkPQTable::new(
            dim,
            pivots.into_boxed_slice(),
            centroid.into_boxed_slice(),
            offsets.into_boxed_slice(),
            None,
        )
        .map_err(PyDiskAnnError::Ann)?;

        let prune_kind = graph_config::PruneKind::from_metric(self.metric);
        let config = graph_config::Builder::new_with(
            self.max_outdegree,
            graph_config::MaxDegree::default_slack(),
            self.l_build,
            prune_kind,
            |b| {
                b.alpha(self.alpha);
                b.max_minibatch_par(default_num_threads());
            },
        )
        .build()
        .map_err(ANNError::from)
        .map_err(PyDiskAnnError::Ann)?;

        let params = DefaultProviderParameters {
            max_points: n_points,
            frozen_points: diskann::utils::ONE,
            dim,
            metric: self.metric,
            prefetch_lookahead: None,
            prefetch_cache_line_level: None,
            max_degree: config.max_degree_u32().get(),
        };

        let fp_precursor = CreateFullPrecision::<f32>::new(dim, None);
        let provider = DefaultProvider::new_empty(params, fp_precursor, pq_table, NoDeletes)
            .map_err(PyDiskAnnError::Ann)?;

        let num_threads = default_num_threads();
        let index = Arc::new(PQIndexType::new(
            config,
            provider,
            NonZeroUsize::new(num_threads),
        ));

        println!(
            "[diskann_rs_native] pq fit insert start: n_points={n_points} dim={dim} threads={num_threads} batch={fit_batch_size}"
        );
        let _ = std::io::stdout().flush();
        let handle = self.rt.handle().clone();
        let index_clone = index.clone();

        // Release the GIL while we do the heavy build work.
        let data = data;
        py.allow_threads(move || -> Result<(), PyDiskAnnError> {
            let t0 = std::time::Instant::now();
            let mut last_report = t0;
            let mut start_i = 0usize;

            while start_i < n_points {
                let end_i = (start_i + fit_batch_size).min(n_points);

                let mut vectors: Vec<diskann::utils::async_tools::VectorIdBoxSlice<u32, f32>> =
                    Vec::with_capacity(end_i - start_i);
                for i in start_i..end_i {
                    let start = i * dim;
                    let end = start + dim;
                    let row: &[f32] = &data[start..end];
                    vectors.push(diskann::utils::async_tools::VectorIdBoxSlice::new(
                        i as u32,
                        row.to_vec().into_boxed_slice(),
                    ));
                }
                let vectors = vectors.into_boxed_slice();

                handle.block_on(async {
                    let strategy = Hybrid::new(None);
                    index_clone
                        .multi_insert(strategy, &DefaultContext, vectors)
                        .await
                        .map_err(PyDiskAnnError::Ann)
                })?;

                maybe_report_fit_progress("pq", end_i - 1, n_points, &t0, &mut last_report);
                start_i = end_i;
            }

            Ok(())
        })?;

        self.dim = Some(dim);
        self.n_points = Some(n_points);
        self.index = Some(index);
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
            return Err(PyDiskAnnError::InvalidParameter("l_search must be >= k".into()));
        }

        let q = q.as_array();
        if q.len() != dim {
            return Err(PyDiskAnnError::InvalidParameter(format!(
                "query dim mismatch: expected {dim}, got {}",
                q.len()
            )));
        }

        let query: Arc<[f32]> = q.iter().copied().collect::<Vec<_>>().into();
        let handle = self.rt.handle().clone();
        let index = index.clone();

        let ids = py.allow_threads(move || -> Result<Vec<u32>, PyDiskAnnError> {
            handle.block_on(async {
                let search_params = SearchParams::new(k, l_search, None).map_err(ANNError::from)?;
                let mut ids = vec![0u32; k];
                let mut distances = vec![0.0f32; k];
                let mut out = search_output_buffer::IdDistance::new(&mut ids, &mut distances);

                let strategy = Hybrid::new(None);

                index
                    .search(&strategy, &DefaultContext, query.as_ref(), &search_params, &mut out)
                    .await
                    .map_err(PyDiskAnnError::Ann)?;
                Ok::<Vec<u32>, PyDiskAnnError>(ids)
            })
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
            return Err(PyDiskAnnError::InvalidParameter("l_search must be >= k".into()));
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
        let handle = self.rt.handle().clone();
        let index = index.clone();
        let results = py.allow_threads(move || -> Result<Vec<u32>, PyDiskAnnError> {
            let search_params = SearchParams::new(k, l_search, None).map_err(ANNError::from)?;
            let strategy = Hybrid::new(None);
            
            let mut all = vec![0u32; n_queries * k];
            
            all.par_chunks_mut(k).enumerate().try_for_each(|(qi, out_slice)| {
                let start = qi * dim;
                let end = start + dim;
                let query: &[f32] = &queries[start..end];

                let mut distances = vec![0.0f32; k];
                let mut out = search_output_buffer::IdDistance::new(out_slice, &mut distances);

                handle.block_on(async {
                    index
                        .search(&strategy, &DefaultContext, query, &search_params, &mut out)
                        .await
                }).map_err(PyDiskAnnError::Ann)?;
                Ok::<(), PyDiskAnnError>(())
            })?;

            Ok(all)
        })?;

        let array = ndarray::Array2::from_shape_vec((n_queries, k), results)
            .map_err(|e| PyDiskAnnError::InvalidParameter(e.to_string()))?;
        Ok(array.into_pyarray_bound(py))
    }
}

#[pymethods]
impl SphericalIndex {
    #[new]
    fn new(
        metric: String,
        l_build: usize,
        max_outdegree: usize,
        alpha: f32,
        nbits: usize,
        rng_seed: Option<u64>,
    ) -> Result<Self, PyDiskAnnError> {
        let metric = Metric::from_str(&metric)
            .map_err(|_| PyDiskAnnError::InvalidMetric(metric.clone()))?;

        // Spherical quantization only supports SquaredL2 and InnerProduct.
        // Cosine similarity is equivalent to inner product on L2-normalized vectors,
        // so we treat cosine as inner product internally.
        let metric = match metric {
            Metric::Cosine => Metric::InnerProduct,
            m => m,
        };

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
        if !matches!(nbits, 1 | 2 | 4) {
            return Err(PyDiskAnnError::InvalidParameter(
                "nbits must be one of: 1, 2, 4".into(),
            ));
        }

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| PyDiskAnnError::InvalidParameter(format!("failed to create tokio runtime: {e}")))?;

        Ok(Self {
            metric,
            l_build,
            max_outdegree,
            alpha,
            l_search: 10,
            dim: None,
            n_points: None,
            nbits,
            rng_seed: rng_seed.unwrap_or(0),
            rt,
            index: None,
        })
    }

    #[staticmethod]
    fn load(prefix: String) -> Result<Self, PyDiskAnnError> {
        let (kind, metric, l_build, max_outdegree, alpha, dim, n_points, max_degree, meta) = {
            let (kind, metric, l_build, max_outdegree, alpha, dim, n_points, max_degree, raw) =
                read_kind_meta(spherical_meta_path(&prefix))?;
            (kind, metric, l_build, max_outdegree, alpha, dim, n_points, max_degree, raw)
        };

        if kind != "spherical" {
            return Err(PyDiskAnnError::InvalidParameter(format!(
                "expected spherical meta, got kind={kind}"
            )));
        }
        let nbits = meta
            .get("nbits")
            .and_then(|x| x.as_u64())
            .ok_or_else(|| PyDiskAnnError::InvalidParameter("meta.nbits missing".into()))?
            as usize;
        let rng_seed = meta.get("rng_seed").and_then(|x| x.as_u64()).unwrap_or(0);

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| PyDiskAnnError::InvalidParameter(format!("failed to create tokio runtime: {e}")))?;
        let handle = rt.handle().clone();

        let prune_kind = graph_config::PruneKind::from_metric(metric);
        let config = graph_config::Builder::new_with(
            max_outdegree,
            graph_config::MaxDegree::new(max_degree as usize),
            l_build,
            prune_kind,
            |b| {
                b.alpha(alpha);
            },
        )
        .build()
        .map_err(ANNError::from)
        .map_err(PyDiskAnnError::Ann)?;

        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let index_config = IndexConfiguration::new(
            metric,
            dim,
            n_points,
            diskann::utils::ONE,
            num_threads,
            config,
        );

        let index = match nbits {
            1 => {
                let index = handle
                    .block_on(async {
                        load_fp_index::<f32, _, SQStore<1>>(&FileStorageProvider, &prefix, index_config)
                            .await
                    })
                    .map_err(PyDiskAnnError::Ann)?;
                SphericalIndexInner::Bits1(Arc::new(index))
            }
            2 => {
                let index = handle
                    .block_on(async {
                        load_fp_index::<f32, _, SQStore<2>>(&FileStorageProvider, &prefix, index_config)
                            .await
                    })
                    .map_err(PyDiskAnnError::Ann)?;
                SphericalIndexInner::Bits2(Arc::new(index))
            }
            4 => {
                let index = handle
                    .block_on(async {
                        load_fp_index::<f32, _, SQStore<4>>(&FileStorageProvider, &prefix, index_config)
                            .await
                    })
                    .map_err(PyDiskAnnError::Ann)?;
                SphericalIndexInner::Bits4(Arc::new(index))
            }
            _ => {
                return Err(PyDiskAnnError::InvalidParameter(
                    "nbits must be one of: 1, 2, 4".into(),
                ))
            }
        };

        Ok(Self {
            metric,
            l_build,
            max_outdegree,
            alpha,
            l_search: 10,
            dim: Some(dim),
            n_points: Some(n_points),
            nbits,
            rng_seed,
            rt,
            index: Some(index),
        })
    }

    fn save(&self, prefix: String) -> Result<(), PyDiskAnnError> {
        let Some(dim) = self.dim else {
            return Err(PyDiskAnnError::InvalidParameter(
                "fit() must be called before save()".into(),
            ));
        };
        let Some(n_points) = self.n_points else {
            return Err(PyDiskAnnError::InvalidParameter(
                "fit() must be called before save()".into(),
            ));
        };
        let index = self.index.as_ref().ok_or_else(|| {
            PyDiskAnnError::InvalidParameter("fit() must be called before save()".into())
        })?;

        ensure_parent_dir(&prefix)?;

        let prune_kind = graph_config::PruneKind::from_metric(self.metric);
        let config = graph_config::Builder::new_with(
            self.max_outdegree,
            graph_config::MaxDegree::default_slack(),
            self.l_build,
            prune_kind,
            |b| {
                b.alpha(self.alpha);
            },
        )
        .build()
        .map_err(ANNError::from)
        .map_err(PyDiskAnnError::Ann)?;

        let max_degree = config.max_degree_u32().get();
        let handle = self.rt.handle().clone();
        let index = index.clone();
        handle
            .block_on(async {
                let metadata = AsyncIndexMetadata::new(&prefix);
                match index {
                    SphericalIndexInner::Bits1(i) => {
                        i.save_with(&FileStorageProvider, &metadata)
                            .await
                            .map_err(PyDiskAnnError::Ann)?;
                    }
                    SphericalIndexInner::Bits2(i) => {
                        i.save_with(&FileStorageProvider, &metadata)
                            .await
                            .map_err(PyDiskAnnError::Ann)?;
                    }
                    SphericalIndexInner::Bits4(i) => {
                        i.save_with(&FileStorageProvider, &metadata)
                            .await
                            .map_err(PyDiskAnnError::Ann)?;
                    }
                }
                Ok::<(), PyDiskAnnError>(())
            })?;

        write_kind_meta(
            spherical_meta_path(&prefix),
            &prefix,
            "spherical",
            self.metric,
            self.l_build,
            self.max_outdegree,
            self.alpha,
            dim,
            n_points,
            max_degree,
            serde_json::json!({
                "nbits": self.nbits,
                "rng_seed": self.rng_seed,
            }),
        )?;

        Ok(())
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
            return Err(PyDiskAnnError::InvalidParameter("X must be non-empty".into()));
        }
        if n_points > (u32::MAX as usize) {
            return Err(PyDiskAnnError::InvalidParameter(
                "X has too many rows for u32 ids".into(),
            ));
        }

        let data: Vec<f32> = x.iter().copied().collect();

        // Can be overridden via env var DISKANN_RS_FIT_BATCH_SIZE.
        // Keep consistent with the FP index default.
        let fit_batch_size = env_fit_batch_size().unwrap_or(20_000);

        let prune_kind = graph_config::PruneKind::from_metric(self.metric);
        let config = graph_config::Builder::new_with(
            self.max_outdegree,
            graph_config::MaxDegree::default_slack(),
            self.l_build,
            prune_kind,
            |b| {
                b.alpha(self.alpha);
                b.max_minibatch_par(default_num_threads());
            },
        )
        .build()
        .map_err(ANNError::from)
        .map_err(PyDiskAnnError::Ann)?;

        let params = DefaultProviderParameters {
            max_points: n_points,
            frozen_points: diskann::utils::ONE,
            dim,
            metric: self.metric,
            prefetch_lookahead: None,
            prefetch_cache_line_level: None,
            max_degree: config.max_degree_u32().get(),
        };

        fn build_index_1(
            config: graph_config::Config,
            params: DefaultProviderParameters,
            data: &[f32],
            n_points: usize,
            dim: usize,
            rt: &tokio::runtime::Runtime,
            fit_batch_size: usize,
        ) -> Result<Arc<ScalarQuantIndexType<1>>, PyDiskAnnError> {
            let dataset = MatrixView::try_from(data, n_points, dim).map_err(|e| {
                PyDiskAnnError::InvalidParameter(format!("invalid dataset layout: {e}"))
            })?;
            let quantizer = ScalarQuantizationParameters::default().train(dataset);
            let quant_precursor = WithBits::<1>::new(quantizer);

            let fp_precursor = CreateFullPrecision::<f32>::new(dim, None);
            let provider =
                DefaultProvider::new_empty(params, fp_precursor, quant_precursor, NoDeletes)
                    .map_err(PyDiskAnnError::Ann)?;

            let num_threads = default_num_threads();
            let index = Arc::new(ScalarQuantIndexType::<1>::new(
                config,
                provider,
                NonZeroUsize::new(num_threads),
            ));

            println!(
                "[diskann_rs_native] spherical(nbits=1) fit insert start: n_points={n_points} dim={dim} threads={num_threads} batch={fit_batch_size}"
            );
            let _ = std::io::stdout().flush();

            let handle = rt.handle().clone();
            let index_clone = index.clone();

            handle
                .block_on(async {
                    let t0 = std::time::Instant::now();
                    let mut last_report = t0;
                    let max_in_flight = (num_threads * 4).max(1).min(fit_batch_size.max(1));

                    let mut join_set: tokio::task::JoinSet<Result<(), PyDiskAnnError>> =
                        tokio::task::JoinSet::new();
                    let mut completed = 0usize;
                    for i in 0..n_points {
                        let start = i * dim;
                        let end = start + dim;
                        let row_vec: Vec<f32> = data[start..end].to_vec();
                        let id = i as u32;
                        let index_task = index_clone.clone();

                        join_set.spawn(async move {
                            index_task
                                .insert(Quantized, &DefaultContext, &id, &row_vec)
                                .await
                                .map_err(PyDiskAnnError::Ann)
                        });

                        if join_set.len() >= max_in_flight {
                            if let Some(res) = join_set.join_next().await {
                                res.map_err(|e| {
                                    PyDiskAnnError::InvalidParameter(format!(
                                        "insert task join error: {e}"
                                    ))
                                })??;
                                completed += 1;
                                maybe_report_fit_progress(
                                    "spherical(1)",
                                    completed - 1,
                                    n_points,
                                    &t0,
                                    &mut last_report,
                                );
                            }
                        }
                    }

                    while let Some(res) = join_set.join_next().await {
                        res.map_err(|e| {
                            PyDiskAnnError::InvalidParameter(format!(
                                "insert task join error: {e}"
                            ))
                        })??;
                        completed += 1;
                        maybe_report_fit_progress(
                            "spherical(1)",
                            completed - 1,
                            n_points,
                            &t0,
                            &mut last_report,
                        );
                    }

                    Ok::<(), PyDiskAnnError>(())
                })?;

            Ok(index)
        }

        fn build_index_2(
            config: graph_config::Config,
            params: DefaultProviderParameters,
            data: &[f32],
            n_points: usize,
            dim: usize,
            rt: &tokio::runtime::Runtime,
            fit_batch_size: usize,
        ) -> Result<Arc<ScalarQuantIndexType<2>>, PyDiskAnnError> {
            let dataset = MatrixView::try_from(data, n_points, dim).map_err(|e| {
                PyDiskAnnError::InvalidParameter(format!("invalid dataset layout: {e}"))
            })?;
            let quantizer = ScalarQuantizationParameters::default().train(dataset);
            let quant_precursor = WithBits::<2>::new(quantizer);

            let fp_precursor = CreateFullPrecision::<f32>::new(dim, None);
            let provider =
                DefaultProvider::new_empty(params, fp_precursor, quant_precursor, NoDeletes)
                    .map_err(PyDiskAnnError::Ann)?;

            let num_threads = default_num_threads();
            let index = Arc::new(ScalarQuantIndexType::<2>::new(
                config,
                provider,
                NonZeroUsize::new(num_threads),
            ));

            println!(
                "[diskann_rs_native] spherical(nbits=2) fit insert start: n_points={n_points} dim={dim} threads={num_threads} batch={fit_batch_size}"
            );
            let _ = std::io::stdout().flush();

            let handle = rt.handle().clone();
            let index_clone = index.clone();

            handle
                .block_on(async {
                    let t0 = std::time::Instant::now();
                    let mut last_report = t0;
                    let max_in_flight = (num_threads * 4).max(1).min(fit_batch_size.max(1));

                    let mut join_set: tokio::task::JoinSet<Result<(), PyDiskAnnError>> =
                        tokio::task::JoinSet::new();
                    let mut completed = 0usize;
                    for i in 0..n_points {
                        let start = i * dim;
                        let end = start + dim;
                        let row_vec: Vec<f32> = data[start..end].to_vec();
                        let id = i as u32;
                        let index_task = index_clone.clone();

                        join_set.spawn(async move {
                            index_task
                                .insert(Quantized, &DefaultContext, &id, &row_vec)
                                .await
                                .map_err(PyDiskAnnError::Ann)
                        });

                        if join_set.len() >= max_in_flight {
                            if let Some(res) = join_set.join_next().await {
                                res.map_err(|e| {
                                    PyDiskAnnError::InvalidParameter(format!(
                                        "insert task join error: {e}"
                                    ))
                                })??;
                                completed += 1;
                                maybe_report_fit_progress(
                                    "spherical(2)",
                                    completed - 1,
                                    n_points,
                                    &t0,
                                    &mut last_report,
                                );
                            }
                        }
                    }

                    while let Some(res) = join_set.join_next().await {
                        res.map_err(|e| {
                            PyDiskAnnError::InvalidParameter(format!(
                                "insert task join error: {e}"
                            ))
                        })??;
                        completed += 1;
                        maybe_report_fit_progress(
                            "spherical(2)",
                            completed - 1,
                            n_points,
                            &t0,
                            &mut last_report,
                        );
                    }

                    Ok::<(), PyDiskAnnError>(())
                })?;

            Ok(index)
        }

        fn build_index_4(
            config: graph_config::Config,
            params: DefaultProviderParameters,
            data: &[f32],
            n_points: usize,
            dim: usize,
            rt: &tokio::runtime::Runtime,
            fit_batch_size: usize,
        ) -> Result<Arc<ScalarQuantIndexType<4>>, PyDiskAnnError> {
            let dataset = MatrixView::try_from(data, n_points, dim).map_err(|e| {
                PyDiskAnnError::InvalidParameter(format!("invalid dataset layout: {e}"))
            })?;
            let quantizer = ScalarQuantizationParameters::default().train(dataset);
            let quant_precursor = WithBits::<4>::new(quantizer);

            let fp_precursor = CreateFullPrecision::<f32>::new(dim, None);
            let provider =
                DefaultProvider::new_empty(params, fp_precursor, quant_precursor, NoDeletes)
                    .map_err(PyDiskAnnError::Ann)?;

            let num_threads = default_num_threads();
            let index = Arc::new(ScalarQuantIndexType::<4>::new(
                config,
                provider,
                NonZeroUsize::new(num_threads),
            ));

            println!(
                "[diskann_rs_native] spherical(nbits=4) fit insert start: n_points={n_points} dim={dim} threads={num_threads} batch={fit_batch_size}"
            );
            let _ = std::io::stdout().flush();

            let handle = rt.handle().clone();
            let index_clone = index.clone();

            handle
                .block_on(async {
                    let t0 = std::time::Instant::now();
                    let mut last_report = t0;
                    let max_in_flight = (num_threads * 4).max(1).min(fit_batch_size.max(1));

                    let mut join_set: tokio::task::JoinSet<Result<(), PyDiskAnnError>> =
                        tokio::task::JoinSet::new();
                    let mut completed = 0usize;
                    for i in 0..n_points {
                        let start = i * dim;
                        let end = start + dim;
                        let row_vec: Vec<f32> = data[start..end].to_vec();
                        let id = i as u32;
                        let index_task = index_clone.clone();

                        join_set.spawn(async move {
                            index_task
                                .insert(Quantized, &DefaultContext, &id, &row_vec)
                                .await
                                .map_err(PyDiskAnnError::Ann)
                        });

                        if join_set.len() >= max_in_flight {
                            if let Some(res) = join_set.join_next().await {
                                res.map_err(|e| {
                                    PyDiskAnnError::InvalidParameter(format!(
                                        "insert task join error: {e}"
                                    ))
                                })??;
                                completed += 1;
                                maybe_report_fit_progress(
                                    "spherical(4)",
                                    completed - 1,
                                    n_points,
                                    &t0,
                                    &mut last_report,
                                );
                            }
                        }
                    }

                    while let Some(res) = join_set.join_next().await {
                        res.map_err(|e| {
                            PyDiskAnnError::InvalidParameter(format!(
                                "insert task join error: {e}"
                            ))
                        })??;
                        completed += 1;
                        maybe_report_fit_progress(
                            "spherical(4)",
                            completed - 1,
                            n_points,
                            &t0,
                            &mut last_report,
                        );
                    }

                    Ok::<(), PyDiskAnnError>(())
                })?;

            Ok(index)
        }

        // Release the GIL while we train quantizer + build the index.
        let nbits = self.nbits;
        let rt_ref = &self.rt;
        let data_ref = data;
        let index = py.allow_threads(move || -> Result<SphericalIndexInner, PyDiskAnnError> {
            match nbits {
                1 => Ok(SphericalIndexInner::Bits1(build_index_1(
                    config.clone(),
                    params.clone(),
                    data_ref.as_slice(),
                    n_points,
                    dim,
                    rt_ref,
                    fit_batch_size,
                )?)),
                2 => Ok(SphericalIndexInner::Bits2(build_index_2(
                    config.clone(),
                    params.clone(),
                    data_ref.as_slice(),
                    n_points,
                    dim,
                    rt_ref,
                    fit_batch_size,
                )?)),
                4 => Ok(SphericalIndexInner::Bits4(build_index_4(
                    config,
                    params,
                    data_ref.as_slice(),
                    n_points,
                    dim,
                    rt_ref,
                    fit_batch_size,
                )?)),
                _ => Err(PyDiskAnnError::InvalidParameter(
                    "nbits must be one of: 1, 2, 4".into(),
                )),
            }
        })?;

        self.dim = Some(dim);
        self.n_points = Some(n_points);
        self.index = Some(index);
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
            return Err(PyDiskAnnError::InvalidParameter("l_search must be >= k".into()));
        }

        let q = q.as_array();
        if q.len() != dim {
            return Err(PyDiskAnnError::InvalidParameter(format!(
                "query dim mismatch: expected {dim}, got {}",
                q.len()
            )));
        }

        let query: Arc<[f32]> = q.iter().copied().collect::<Vec<_>>().into();
        let handle = self.rt.handle().clone();
        let index = index.clone();
        let ids = py.allow_threads(move || -> Result<Vec<u32>, PyDiskAnnError> {
            handle.block_on(async {
                let search_params = SearchParams::new(k, l_search, None).map_err(ANNError::from)?;
                let mut ids = vec![0u32; k];
                let mut distances = vec![0.0f32; k];
                let mut out = search_output_buffer::IdDistance::new(&mut ids, &mut distances);

                match index {
                    SphericalIndexInner::Bits1(i) => {
                        i.search(&Quantized, &DefaultContext, query.as_ref(), &search_params, &mut out)
                            .await
                            .map_err(PyDiskAnnError::Ann)?;
                    }
                    SphericalIndexInner::Bits2(i) => {
                        i.search(&Quantized, &DefaultContext, query.as_ref(), &search_params, &mut out)
                            .await
                            .map_err(PyDiskAnnError::Ann)?;
                    }
                    SphericalIndexInner::Bits4(i) => {
                        i.search(&Quantized, &DefaultContext, query.as_ref(), &search_params, &mut out)
                            .await
                            .map_err(PyDiskAnnError::Ann)?;
                    }
                }
                Ok::<Vec<u32>, PyDiskAnnError>(ids)
            })
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
            return Err(PyDiskAnnError::InvalidParameter("l_search must be >= k".into()));
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
        let handle = self.rt.handle().clone();
        let index = index.clone();
        let results = py.allow_threads(move || -> Result<Vec<u32>, PyDiskAnnError> {
            let search_params = SearchParams::new(k, l_search, None).map_err(ANNError::from)?;
            
            let mut all = vec![0u32; n_queries * k];
            
            all.par_chunks_mut(k).enumerate().try_for_each(|(qi, out_slice)| {
                let start = qi * dim;
                let end = start + dim;
                let query: &[f32] = &queries[start..end];

                let mut distances = vec![0.0f32; k];
                let mut out = search_output_buffer::IdDistance::new(out_slice, &mut distances);

                handle.block_on(async {
                    match &index {
                        SphericalIndexInner::Bits1(i) => {
                            i.search(&Quantized, &DefaultContext, query, &search_params, &mut out).await
                        }
                        SphericalIndexInner::Bits2(i) => {
                            i.search(&Quantized, &DefaultContext, query, &search_params, &mut out).await
                        }
                        SphericalIndexInner::Bits4(i) => {
                            i.search(&Quantized, &DefaultContext, query, &search_params, &mut out).await
                        }
                    }
                }).map_err(PyDiskAnnError::Ann)?;
                Ok::<(), PyDiskAnnError>(())
            })?;
            
            Ok(all)
        })?;

        let array = ndarray::Array2::from_shape_vec((n_queries, k), results)
            .map_err(|e| PyDiskAnnError::InvalidParameter(e.to_string()))?;
        Ok(array.into_pyarray_bound(py))
    }
}

#[pymodule]
fn diskann_rs_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Index>()?;
    m.add_class::<PQIndex>()?;
    m.add_class::<PQDiskIndex>()?;
    m.add_class::<SphericalIndex>()?;
    Ok(())
}
