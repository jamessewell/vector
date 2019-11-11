use crate::topology::config::component::ComponentDescription;
use crate::Event;
use inventory;

pub mod static_value;

pub trait Condition {
    fn check(&self, e: &Event) -> bool;
}

#[typetag::serde(tag = "type")]
pub trait ConditionConfig: std::fmt::Debug {
    fn build(&self) -> crate::Result<Box<dyn Condition>>;
}

pub type ConditionDescription = ComponentDescription<Box<dyn ConditionConfig>>;

inventory::collect!(ConditionDescription);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_types() {
        assert_eq!(ConditionDescription::types(), ["static"]);
    }
}
