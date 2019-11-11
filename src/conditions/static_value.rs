use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    conditions::{Condition, ConditionConfig, ConditionDescription},
    Event,
};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct StaticConfig {
    value: bool,
}

impl StaticConfig {
    pub fn new(v: bool) -> Self {
        Self { value: v }
    }
}

impl Condition for StaticConfig {
    fn init(&mut self, _: &IndexMap<String, Arc<dyn Condition>>) -> crate::Result<()> {
        Ok(())
    }

    fn check(&self, _: &Event) -> bool {
        self.value
    }
}

#[typetag::serde(name = "static")]
impl ConditionConfig for StaticConfig {
    fn build(&self) -> crate::Result<Box<dyn Condition>> {
        Ok(Box::new(self.clone()))
    }
}

inventory::submit! {
    ConditionDescription::new::<StaticConfig>("static")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Event;

    #[test]
    fn check_static_true() {
        assert_eq!(StaticConfig { value: true }.check(&Event::from("")), true);
    }

    #[test]
    fn check_static_false() {
        assert_eq!(StaticConfig { value: false }.check(&Event::from("")), false);
    }
}
