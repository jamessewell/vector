use serde::{Deserialize, Serialize};

use crate::{
    conditions::{Condition, ConditionDefinition},
    Event,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StaticConfig {
    value: bool,
}

impl Condition for StaticConfig {
    fn check(&self, _: &Event) -> Result<bool, String> {
        return Ok(self.value);
    }
}

inventory::submit! {
    ConditionDefinition::new::<StaticConfig>("static")
}