use crate::topology::config::{
    component::ExampleError, GlobalOptions, SinkDescription, SourceDescription,
    TransformDescription,
};
use indexmap::IndexMap;
use serde::Serialize;
use std::collections::BTreeMap;
use structopt::StructOpt;
use toml::Value;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct Opts {
    target: String,
}

pub fn cmd(opts: &Opts) -> exitcode::ExitCode {
    exitcode::OK
}
