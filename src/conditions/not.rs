use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    conditions::{static_value::StaticConfig, Condition, ConditionConfig, ConditionDescription},
    Event,
};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct NotConfig {
    child: String,
}

#[typetag::serde(name = "not")]
impl ConditionConfig for NotConfig {
    fn build(&self) -> crate::Result<Box<dyn Condition>> {
        Ok(Box::new(Not {
            child_name: self.child.clone(),
            child: Arc::new(StaticConfig::new(false)),
        }))
    }
}

pub struct Not {
    child_name: String,
    child: Arc<dyn Condition>,
}

impl Condition for Not {
    fn init(&mut self, siblings: &IndexMap<String, Arc<dyn Condition>>) -> crate::Result<()> {
        siblings
            .get(&self.child_name)
            .map(|c| {
                self.child = Arc::clone(c);
                ()
            })
            .ok_or(format!("failed to find condition '{}'", self.child_name).into())
    }

    fn check(&self, e: &Event) -> bool {
        self.child.check(e)
    }
}

inventory::submit! {
    ConditionDescription::new::<NotConfig>("not")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Event;

    #[test]
    fn check_child_static_true() {
        let mut v: IndexMap<String, Arc<dyn Condition>> = IndexMap::new();
        v.insert("foo".to_owned(), Arc::new(StaticConfig::new(true)));

        let mut not = NotConfig {
            child: "foo".to_owned(),
        }
        .build()
        .unwrap();
        not.init(&v).unwrap();

        assert_eq!(not.check(&Event::from("")), true);
    }

    #[test]
    fn check_child_static_false() {
        let mut v: IndexMap<String, Arc<dyn Condition>> = IndexMap::new();
        v.insert("foo".to_owned(), Arc::new(StaticConfig::new(false)));

        let mut not = NotConfig {
            child: "foo".to_owned(),
        }
        .build()
        .unwrap();
        not.init(&v).unwrap();

        assert_eq!(not.check(&Event::from("")), false);
    }

    #[test]
    fn check_not_childless() {
        let mut not = NotConfig {
            child: "foo".to_owned(),
        }
        .build()
        .unwrap();
        assert_eq!(
            not.init(&IndexMap::new()).unwrap_err().to_string(),
            "failed to find condition 'foo'"
        );
    }
}
