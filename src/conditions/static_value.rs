use serde::{Deserialize, Serialize};

use crate::{
    conditions::{Condition, ConditionConfig, ConditionDescription},
    Event,
};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct StaticConfig {
    value: bool,
}

impl Condition for StaticConfig {
    fn check(&self, _: &Event) -> bool {
        return self.value;
    }
}

#[typetag::serde(name = "static")]
impl ConditionConfig for StaticConfig {
    fn build(&self) -> crate::Result<Box<dyn Condition>> {
        return Ok(Box::new(self.clone()));
    }
}

inventory::submit! {
    ConditionDescription::new::<StaticConfig>("static")
}