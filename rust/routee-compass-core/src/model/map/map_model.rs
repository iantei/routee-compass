use super::map_error::MapError;
use super::map_model_config::MapModelConfig;
use super::matching_type::MatchingType;
use super::spatial_index::SpatialIndex;
use super::{geometry_model::GeometryModel, matching_type::MapInputResult};
use crate::algorithm::search::search_instance::SearchInstance;
use crate::model::network::{EdgeId, Graph};
use geo::LineString;
use std::sync::Arc;

pub struct MapModel {
    pub matching_type: MatchingType,
    pub spatial_index: SpatialIndex,
    pub geometry_model: GeometryModel,
    pub queries_without_destinations: bool,
}

impl MapModel {
    pub fn new(graph: Arc<Graph>, config: MapModelConfig) -> Result<MapModel, MapError> {
        match config {
            MapModelConfig::VertexMapModelConfig {
                tolerance,
                geometry_input_file,
                queries_without_destinations,
                matching_type,
            } => {
                let spatial_index =
                    SpatialIndex::new_vertex_oriented(&graph.clone().vertices, tolerance);
                let geometry_model = match geometry_input_file {
                    None => GeometryModel::new_from_vertices(graph),
                    Some(file) => GeometryModel::new_from_edges(&file, graph.clone()),
                }?;
                let map_model = MapModel {
                    matching_type: matching_type.unwrap_or_default(),
                    spatial_index,
                    geometry_model,
                    queries_without_destinations,
                };
                Ok(map_model)
            }
            MapModelConfig::EdgeMapModelConfig {
                tolerance,
                geometry_input_file,
                queries_without_destinations,
                matching_type,
            } => {
                let geometry_model =
                    GeometryModel::new_from_edges(&geometry_input_file, graph.clone())?;
                let spatial_index =
                    SpatialIndex::new_edge_oriented(graph.clone(), &geometry_model, tolerance);
                let map_model = MapModel {
                    matching_type: matching_type.unwrap_or_default(),
                    spatial_index,
                    geometry_model,
                    queries_without_destinations,
                };
                Ok(map_model)
            }
        }
    }

    pub fn get<'a>(&'a self, edge_id: &EdgeId) -> Result<&'a LineString<f32>, MapError> {
        self.geometry_model.get(edge_id)
    }

    pub fn map_match(
        &self,
        query: &mut serde_json::Value,
        si: &SearchInstance,
    ) -> Result<(), MapError> {
        self.matching_type.process_origin(
            self,
            si.frontier_model.clone(),
            si.directed_graph.clone(),
            query,
        )?;
        match self.matching_type.process_destination(
            self,
            si.frontier_model.clone(),
            si.directed_graph.clone(),
            query,
        )? {
            MapInputResult::NotFound if !self.queries_without_destinations => {
                Err(MapError::DestinationsRequired(self.matching_type.clone()))
            }
            _ => Ok(()),
        }
    }
}
