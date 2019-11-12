use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    conditions::{Condition, ConditionConfig, ConditionDescription},
    Event,
};

//------------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum CheckFieldsPredicateArg {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

pub trait CheckFieldsPredicate: std::fmt::Debug + Send + Sync {
    fn check(&self, e: &Event) -> bool;
}

#[derive(Debug, Clone)]
struct EqualsPredicate {
    target: String,
    arg: String,
}

impl CheckFieldsPredicate for EqualsPredicate {
    fn check(&self, e: &Event) -> bool {
        false
    }
}

fn build_predicate(
    target: String,
    predicate: String,
    arg: &CheckFieldsPredicateArg,
) -> Result<Box<dyn CheckFieldsPredicate>, Vec<String>> {
    Ok(Box::new(EqualsPredicate {
        target: target,
        arg: predicate,
    }))
}

fn build_predicates(
    map: &IndexMap<String, CheckFieldsPredicateArg>,
) -> Result<Vec<Box<dyn CheckFieldsPredicate>>, Vec<String>> {
    let mut predicates: Vec<Box<dyn CheckFieldsPredicate>> = Vec::new();
    let mut errors = Vec::new();

    for (target_pred, arg) in map {
        if target_pred
            .rfind('.')
            .and_then(|i| {
                if i > 0 && i < target_pred.len() - 1 {
                    Some(i)
                } else {
                    None
                }
            })
            .and_then(|i| {
                let mut target = target_pred.clone();
                let pred = target.split_off(i);
                match build_predicate(target, pred, arg) {
                    Ok(pred) => predicates.push(pred),
                    Err(errs) => errors.extend(errs),
                }
                Some(())
            })
            .is_none()
        {
            errors.push(format!("predicate not found in check_fields value '{}', format must be <target>.<predicate>", target_pred));
        }
    }

    if errors.is_empty() {
        Ok(predicates)
    } else {
        Err(errors)
    }
}

//------------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckFieldsConfig {
    #[serde(flatten, default)]
    predicates: IndexMap<String, CheckFieldsPredicateArg>,
}

inventory::submit! {
    ConditionDescription::new::<CheckFieldsConfig>("check_fields")
}

#[typetag::serde(name = "check_fields")]
impl ConditionConfig for CheckFieldsConfig {
    fn build(&self) -> crate::Result<Box<dyn Condition>> {
        build_predicates(&self.predicates)
            .map(|preds| -> Box<dyn Condition> { Box::new(CheckFields { predicates: preds }) })
            .map_err(|errs| {
                if errs.len() > 1 {
                    let mut err_fmt = errs.join("\n");
                    err_fmt.insert_str(0, "failed to parse predicates:\n");
                    err_fmt
                } else {
                    errs[0].clone()
                }
                .into()
            })
    }
}

//------------------------------------------------------------------------------

pub struct CheckFields {
    predicates: Vec<Box<dyn CheckFieldsPredicate>>,
}

impl Condition for CheckFields {
    fn init(&mut self, _: &IndexMap<String, Arc<dyn Condition>>) -> crate::Result<()> {
        Ok(())
    }

    fn check(&self, e: &Event) -> bool {
        self.predicates.iter().find(|p| !p.check(e)).is_none()
    }
}

//------------------------------------------------------------------------------
