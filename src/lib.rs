#[macro_use]
extern crate enum_primitive;

mod plugin;
mod pool;

use crate::plugin::Plugin;
use crate::pool::GarbageCollectedPool;
use samp::initialize_plugin;
use std::sync::{Arc, Mutex};

initialize_plugin!(
    natives: [
            Plugin::yaml_parse,
            Plugin::yaml_stringify,
            Plugin::yaml_node_type,
            Plugin::yaml_object,
            Plugin::yaml_int,
            Plugin::yaml_bool,
            Plugin::yaml_float,
            Plugin::yaml_string,
            Plugin::yaml_array,
            Plugin::yaml_append,
            Plugin::yaml_set_object,
            Plugin::yaml_set_int,
            Plugin::yaml_set_float,
            Plugin::yaml_set_bool,
            Plugin::yaml_set_string,
            Plugin::yaml_get_object,
            Plugin::yaml_get_int,
            Plugin::yaml_get_float,
            Plugin::yaml_get_bool,
            Plugin::yaml_get_string,
            Plugin::yaml_get_array,
            Plugin::yaml_array_length,
            Plugin::yaml_array_object,
            Plugin::yaml_get_node_int,
            Plugin::yaml_get_node_float,
            Plugin::yaml_get_node_bool,
            Plugin::yaml_get_node_string,
            Plugin::yaml_toggle_gc,
            Plugin::yaml_cleanup
    ],
    {
        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);

        samp::encoding::set_default_encoding(samp::encoding::WINDOWS_1251);

        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[pawn-yaml] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .apply();

        Plugin {
            yaml_nodes: Arc::new(Mutex::new(GarbageCollectedPool::default())),
        }
    }
);
