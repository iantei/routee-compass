use super::search_error::SearchError;
use crate::model::{
    access::access_model::AccessModel,
    cost::cost_model::CostModel,
    frontier::frontier_model::FrontierModel,
    road_network::{graph::Graph, vertex_id::VertexId},
    state::state_model::StateModel,
    termination::termination_model::TerminationModel,
    traversal::{state::state_variable::StateVar, traversal_model::TraversalModel},
    unit::Cost,
};
use std::sync::Arc;

/// instances of read-only objects used for a search that have
/// been prepared for a specific query.
pub struct SearchInstance {
    pub directed_graph: Arc<Graph>,
    pub state_model: Arc<StateModel>,
    pub traversal_model: Arc<dyn TraversalModel>,
    pub access_model: Arc<dyn AccessModel>,
    pub cost_model: CostModel,
    pub frontier_model: Arc<dyn FrontierModel>,
    pub termination_model: Arc<TerminationModel>,
}

impl SearchInstance {
    /// approximates the traversal state delta between two vertices and uses
    /// the result to compute a cost estimate.
    pub fn estimate_traversal_cost(
        &self,
        src: VertexId,
        dst: VertexId,
        state: &[StateVar],
    ) -> Result<Cost, SearchError> {
        let src = self.directed_graph.get_vertex(src)?;
        let dst = self.directed_graph.get_vertex(dst)?;
        let mut dst_state = state.to_vec();

        self.traversal_model
            .estimate_traversal((src, dst), &mut dst_state, &self.state_model)?;
        let cost_estimate = self.cost_model.cost_estimate(state, &dst_state)?;
        Ok(cost_estimate)
    }
}
