use super::energy_traversal_model::EnergyTraversalModel;
use super::vehicle::VehicleType;
use routee_compass_core::model::traversal::traversal_model::TraversalModel;
use routee_compass_core::model::traversal::traversal_model_error::TraversalModelError;
use routee_compass_core::model::traversal::traversal_model_service::TraversalModelService;
use routee_compass_core::model::unit::*;
use routee_compass_core::util::fs::read_decoders;
use routee_compass_core::util::fs::read_utils;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct EnergyModelService {
    pub time_model_service: Arc<dyn TraversalModelService>,
    pub time_model_speed_unit: SpeedUnit,
    pub grade_table: Arc<Option<Box<[Grade]>>>,
    pub grade_table_grade_unit: GradeUnit,
    pub time_unit: TimeUnit,
    pub distance_unit: DistanceUnit,
    pub vehicle_library: HashMap<String, Arc<dyn VehicleType>>,
}

impl EnergyModelService {
    #[allow(clippy::too_many_arguments)]
    pub fn new<P: AsRef<Path>>(
        time_model_service: Arc<dyn TraversalModelService>,
        time_model_speed_unit: SpeedUnit,
        grade_table_path_option: &Option<P>,
        grade_table_grade_unit: GradeUnit,
        output_time_unit_option: Option<TimeUnit>,
        output_distance_unit_option: Option<DistanceUnit>,
        vehicle_library: HashMap<String, Arc<dyn VehicleType>>,
    ) -> Result<Self, TraversalModelError> {
        let output_time_unit = output_time_unit_option.unwrap_or(BASE_TIME_UNIT);
        let output_distance_unit = output_distance_unit_option.unwrap_or(BASE_DISTANCE_UNIT);

        let grade_table: Arc<Option<Box<[Grade]>>> = match grade_table_path_option {
            Some(gtp) => Arc::new(Some(
                read_utils::read_raw_file(gtp, read_decoders::default, None).map_err(|e| {
                    TraversalModelError::FileReadError(gtp.as_ref().to_path_buf(), e.to_string())
                })?,
            )),
            None => Arc::new(None),
        };

        Ok(EnergyModelService {
            time_model_service,
            time_model_speed_unit,
            grade_table,
            grade_table_grade_unit,
            time_unit: output_time_unit,
            distance_unit: output_distance_unit,
            vehicle_library,
        })
    }
}

impl TraversalModelService for EnergyModelService {
    fn build(
        &self,
        parameters: &serde_json::Value,
    ) -> Result<Arc<dyn TraversalModel>, TraversalModelError> {
        let arc_self = Arc::new(self.clone());
        let model = EnergyTraversalModel::new(arc_self, parameters)?;
        Ok(Arc::new(model))
    }
}
