use std::collections::{HashMap, HashSet};

use routee_compass_core::model::cost::{
    network::network_cost_mapping::NetworkCostMapping,
    vehicle::vehicle_cost_mapping::VehicleCostMapping,
};

/// collects the keys from the vehicle mappings, to compile the
/// complete collection of state variable names.
pub fn state_variable_names(
    vehicle_mapping: &Option<HashMap<String, VehicleCostMapping>>,
    network_mapping: &Option<HashMap<String, NetworkCostMapping>>,
) -> Option<HashSet<String>> {
    match (&vehicle_mapping, &network_mapping) {
        (None, None) => None,
        (None, Some(nm)) => Some(nm.keys().cloned().collect::<HashSet<_>>()),
        (Some(vm), None) => Some(vm.keys().cloned().collect::<HashSet<_>>()),
        (Some(vm), Some(nm)) => {
            let key_set = vm
                .keys()
                .cloned()
                .chain(nm.keys().cloned())
                .collect::<HashSet<_>>();
            Some(key_set)
        }
    }
}
