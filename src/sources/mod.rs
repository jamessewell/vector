use futures::Future;
#[cfg(feature = "docker")]
pub mod docker;
#[cfg(feature = "file")]
pub mod file;
pub mod journald;
#[cfg(feature = "rdkafka")]
pub mod kafka;
pub mod statsd;
pub mod stdin;
#[cfg(feature = "syslog")]
pub mod syslog;
pub mod tcp;
pub mod udp;
mod util;
pub mod vector;

pub type Source = Box<dyn Future<Item = (), Error = ()> + Send>;
