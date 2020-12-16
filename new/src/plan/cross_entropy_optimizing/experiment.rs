use super::ceo::CEO;
use super::fcn::FCN;
use super::world::World;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Experiment {
    pub fcn: FCN,
    pub ceo: CEO,
    pub world: World,
}
