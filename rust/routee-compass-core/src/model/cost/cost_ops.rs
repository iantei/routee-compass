use super::{
    cost_aggregation::CostAggregation, cost_error::CostError,
    network::network_cost_rate::NetworkCostRate, vehicle::vehicle_cost_rate::VehicleCostRate,
};
use crate::model::{property::edge::Edge, traversal::state::state_variable::StateVar, unit::Cost};

pub fn calculate_vehicle_costs(
    prev_state: &[StateVar],
    next_state: &[StateVar],
    state_variable_indices: &[(String, usize)],
    state_variable_coefficients: &[f64],
    rates: &[VehicleCostRate],
    cost_aggregation: &CostAggregation,
) -> Result<Cost, CostError> {
    let costs = state_variable_indices.iter().map(|(name, idx)| {
        let prev_state_var = prev_state
            .get(*idx)
            .ok_or_else(|| CostError::StateIndexOutOfBounds(*idx, name.clone()))?;
        let next_state_var = next_state
            .get(*idx)
            .ok_or_else(|| CostError::StateIndexOutOfBounds(*idx, name.clone()))?;
        let delta: StateVar = *next_state_var - *prev_state_var;
        let mapping = rates
            .get(*idx)
            .ok_or_else(|| CostError::StateVariableNotFound(name.clone()))?;
        let coefficient = state_variable_coefficients.get(*idx).unwrap_or(&1.0);
        let delta_cost = mapping.map_value(delta);
        let cost = delta_cost * coefficient;
        Ok((name, cost))
    });
    let cost = cost_aggregation.agg_iter(costs);
    cost
}

pub fn calculate_network_traversal_costs(
    prev_state: &[StateVar],
    next_state: &[StateVar],
    edge: &Edge,
    state_variable_indices: &[(String, usize)],
    state_variable_coefficients: &[f64],
    rates: &[NetworkCostRate],
    cost_aggregation: &CostAggregation,
) -> Result<Cost, CostError> {
    let costs = state_variable_indices
        .iter()
        .map(|(name, idx)| match rates.get(*idx) {
            None => Ok((name, Cost::ZERO)),
            Some(m) => {
                let prev_state_var = prev_state
                    .get(*idx)
                    .ok_or_else(|| CostError::StateIndexOutOfBounds(*idx, name.clone()))?;
                let next_state_var = next_state
                    .get(*idx)
                    .ok_or_else(|| CostError::StateIndexOutOfBounds(*idx, name.clone()))?;
                let coefficient = state_variable_coefficients.get(*idx).unwrap_or(&1.0);
                let traversal_cost = m.traversal_cost(*prev_state_var, *next_state_var, edge)?;
                let cost = traversal_cost * coefficient;
                Ok((name, cost))
            }
        });
    let cost = cost_aggregation.agg_iter(costs);
    cost
}

pub fn calculate_network_access_costs(
    prev_state: &[StateVar],
    next_state: &[StateVar],
    prev_edge: &Edge,
    next_edge: &Edge,
    state_variable_indices: &[(String, usize)],
    state_variable_coefficients: &[f64],
    rates: &[NetworkCostRate],
    cost_aggregation: &CostAggregation,
) -> Result<Cost, CostError> {
    let costs = state_variable_indices
        .iter()
        .map(|(name, idx)| match rates.get(*idx) {
            None => Ok((name, Cost::ZERO)),
            Some(m) => {
                let prev_state_var = prev_state
                    .get(*idx)
                    .ok_or_else(|| CostError::StateIndexOutOfBounds(*idx, name.clone()))?;
                let next_state_var = next_state
                    .get(*idx)
                    .ok_or_else(|| CostError::StateIndexOutOfBounds(*idx, name.clone()))?;
                let access_cost =
                    m.access_cost(*prev_state_var, *next_state_var, prev_edge, next_edge)?;
                let coefficient = state_variable_coefficients.get(*idx).unwrap_or(&1.0);
                let cost = access_cost * coefficient;
                Ok((name, cost))
            }
        });
    let cost = cost_aggregation.agg_iter(costs);
    cost
}
