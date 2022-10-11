#![allow(unused)]

use std::{env, path::PathBuf};

use log::{debug, info};
use time::UtcOffset;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    filter,
    filter::Targets,
    fmt::{time::OffsetTime, MakeWriter},
    layer::SubscriberExt,
    reload,
    reload::Handle,
    util::SubscriberInitExt,
    Layer, Registry,
};

pub type LogHandle = Handle<Targets, Registry>;

pub fn init_logger_new(debug: bool) -> (Option<WorkerGuard>, Option<LogHandle>) {
    use tracing_subscriber::{filter, fmt, prelude::*, reload};
    let filter = match debug {
        true => filter::LevelFilter::DEBUG,
        false => filter::LevelFilter::INFO,
    };
    let (filter, reload_handle) = reload::Layer::new(filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();
    debug!("This will be ignored");
    reload_handle.modify(|filter| *filter = filter::LevelFilter::INFO);
    info!("This will be logged");

    // let timer = OffsetTime::new(
    //     UtcOffset::from_hms(8, 0, 0).unwrap(),
    //     time::format_description::well_known::Rfc3339,
    // );
    // // let stdout_log = tracing_subscriber::fmt::layer().with_timer(timer);
    // let reg = tracing_subscriber::registry();
    //
    // // let base_filter =
    // //     filter::Targets::new().with_target(PKG_NAME.replace('-', "_"), filter::LevelFilter::INFO);
    //
    // let filtered_layer = tracing_subscriber::fmt::layer().with_filter(filter::LevelFilter::INFO);
    // let (filter, reload_handle) = reload::Layer::new(filtered_layer);
    // info!("no");
    // debug!("no");
    // reg.with(filter).init();
    // reload_handle.modify(|layer| *layer.filter_mut() = filter::LevelFilter::DEBUG);
    // debug!("Debug mode is on");

    (None, None)
}

pub fn init_logger(bin_name: &str, debug: bool) -> (Option<WorkerGuard>, Option<LogHandle>) {
    let timer = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::well_known::Rfc3339,
    );
    let stdout_log = tracing_subscriber::fmt::layer().with_timer(timer.clone());
    let reg = tracing_subscriber::registry();
    let base_filter = filter::Targets::new()
        .with_target(bin_name, filter::LevelFilter::DEBUG)
        // TODO: do not hardcode piam_proxy_core
        .with_target("piam_proxy_core", filter::LevelFilter::DEBUG);
    let (filter, reload_handle) = reload::Layer::new(base_filter.clone());

    if debug {
        let file_appender =
            tracing_appender::rolling::daily(log_path(), format!("{}.log", bin_name));
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let file_filter = tracing_subscriber::fmt::layer()
            .with_timer(timer)
            .with_writer(non_blocking.make_writer())
            .with_filter(base_filter);

        reg.with(stdout_log.with_filter(filter).and_then(file_filter))
            .init();
        debug!("Debug mode is on");
        return (Some(guard), Some(reload_handle));
    } else {
        reg.with(stdout_log.with_filter(filter::LevelFilter::INFO))
            .init();
    }

    (None, None)
}

pub fn change_debug(handle: &LogHandle, debug: &str) -> bool {
    panic!("TODO: ");
    let base_filter = filter::Targets::new().with_target("foo", filter::LevelFilter::DEBUG);
    handle.modify(|filter| *filter = base_filter);
    true
}

fn log_path() -> PathBuf {
    if dev_mode() {
        return std::env::current_dir().unwrap();
    }
    PathBuf::from(r"/opt/logs/apps/")
}

fn dev_mode() -> bool {
    env::args().nth(1) == Some("dev".into())
}
