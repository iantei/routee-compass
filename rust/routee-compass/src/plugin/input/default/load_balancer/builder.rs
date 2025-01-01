use super::{plugin::LoadBalancerPlugin, weight_heuristic::WeightHeuristic};
use crate::{
    app::compass::model::builders::InputPluginBuilder,
    app::compass::{CompassConfigurationError, ConfigJsonExtensions},
    plugin::input::input_plugin::InputPlugin,
};
use std::sync::Arc;

pub struct LoadBalancerBuilder {}

impl InputPluginBuilder for LoadBalancerBuilder {
    fn build(
        &self,
        params: &serde_json::Value,
    ) -> Result<Arc<dyn InputPlugin>, CompassConfigurationError> {
        let heuristic =
            params.get_config_serde::<WeightHeuristic>(&"weight_heuristic", &"load_balancer")?;
        Ok(Arc::new(LoadBalancerPlugin { heuristic }))
    }
}
