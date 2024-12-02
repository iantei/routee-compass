use crate::model::network::{edge_id::EdgeId, vertex_id::VertexId};

#[derive(thiserror::Error, Debug)]
pub enum GraphError {
    #[error("edge attribute not found for edge {0}")]
    EdgeNotFound(EdgeId),
    #[error("vertex attribute not found for vertex {0}")]
    VertexNotFound(VertexId),
    #[error("Error with graph attribute {0}: {1}")]
    AttributeError(String, String),
    #[error("error with provided dataset: {0}")]
    DatasetError(String),
    #[error("failure reading graph data from file: {source}")]
    IOError {
        #[from]
        source: std::io::Error,
    },
    #[error("failure reading graph data from CSV: {source}")]
    CsvError {
        #[from]
        source: csv::Error,
    },
    #[error("internal error: adjacency list missing vertex {0}")]
    AdjacencyVertexMissing(VertexId),
    #[error("error creating progress bar for {0}: {1}")]
    ProgressBarBuildError(String, String),
}
