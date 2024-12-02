use super::{
    cost_aggregation::CostAggregation, cost_error::CostError,
    network::network_cost_rate::NetworkCostRate, vehicle::vehicle_cost_rate::VehicleCostRate,
};
use crate::model::{road_network::Edge, traversal::state::state_variable::StateVar, unit::Cost};

/// steps through each state variable and assigns vehicle costs related to that variable
/// due to an edge access + traversal event.
///
/// # Arguments
/// * `prev_state` - the state before beginning the traversal
/// * `next_state` - the state after traversal
/// * `indices`    - feature names and corresponding state indices
pub fn calculate_vehicle_costs(
    state_sequence: (&[StateVar], &[StateVar]),
    indices: &[(String, usize)],
    weights: &[f64],
    rates: &[VehicleCostRate],
    cost_aggregation: &CostAggregation,
) -> Result<Cost, CostError> {
    let (prev_state, next_state) = state_sequence;
    let costs = indices.iter().map(|(name, state_idx)| {
        // compute delta
        let prev_state_var = prev_state
            .get(*state_idx)
            .ok_or_else(|| CostError::StateIndexOutOfBounds(*state_idx, name.clone()))?;
        let next_state_var = next_state
            .get(*state_idx)
            .ok_or_else(|| CostError::StateIndexOutOfBounds(*state_idx, name.clone()))?;
        let delta: StateVar = *next_state_var - *prev_state_var;

        // collect weight and vehicle cost rate
        let mapping = rates.get(*state_idx).ok_or_else(|| {
            CostError::CostVectorOutOfBounds(*state_idx, String::from("vehicle_rates"))
        })?;
        let weight = weights
            .get(*state_idx)
            .ok_or_else(|| CostError::CostVectorOutOfBounds(*state_idx, String::from("weights")))?;

        // compute cost
        let delta_cost = mapping.map_value(delta);
        let cost = delta_cost * weight;
        Ok((name, cost))
    });

    cost_aggregation.agg_iter(costs)
}

pub fn calculate_network_traversal_costs(
    state_sequence: (&[StateVar], &[StateVar]),
    edge: &Edge,
    indices: &[(String, usize)],
    weights: &[f64],
    rates: &[NetworkCostRate],
    cost_aggregation: &CostAggregation,
) -> Result<Cost, CostError> {
    let (prev_state, next_state) = state_sequence;
    let costs = indices.iter().map(|(name, state_idx)| {
        let prev_state_var = prev_state
            .get(*state_idx)
            .ok_or_else(|| CostError::StateIndexOutOfBounds(*state_idx, name.clone()))?;
        let next_state_var = next_state
            .get(*state_idx)
            .ok_or_else(|| CostError::StateIndexOutOfBounds(*state_idx, name.clone()))?;

        // determine weight and access cost for this state feature
        let weight = weights
            .get(*state_idx)
            .ok_or_else(|| CostError::CostVectorOutOfBounds(*state_idx, String::from("weights")))?;
        let rate = rates.get(*state_idx).ok_or_else(|| {
            CostError::CostVectorOutOfBounds(*state_idx, String::from("network_cost_rate"))
        })?;
        let access_cost = rate.traversal_cost(*prev_state_var, *next_state_var, edge)?;
        let cost = access_cost * weight;
        Ok((name, cost))
    });

    cost_aggregation.agg_iter(costs)
}

pub fn calculate_network_access_costs(
    state_sequence: (&[StateVar], &[StateVar]),
    edge_sequence: (&Edge, &Edge),
    indices: &[(String, usize)],
    weights: &[f64],
    rates: &[NetworkCostRate],
    cost_aggregation: &CostAggregation,
) -> Result<Cost, CostError> {
    let (prev_state, next_state) = state_sequence;
    let (prev_edge, next_edge) = edge_sequence;
    let costs = indices.iter().map(|(name, idx)| match rates.get(*idx) {
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
            let coefficient = weights.get(*idx).unwrap_or(&1.0);
            let cost = access_cost * coefficient;
            Ok((name, cost))
        }
    });

    cost_aggregation.agg_iter(costs)
}
