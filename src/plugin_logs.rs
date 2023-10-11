use extism::{CurrentPlugin, InternalExt, UserData, Val};

/// Make an debug log from a plugin
pub(crate) fn plugin_debug_log(
    plugin: &mut CurrentPlugin,
    inputs: &[Val],
    _outputs: &mut [Val],
    _user_data: UserData,
) -> Result<(), extism::Error> {
    let message: String = plugin
        .memory_read_str(inputs[0].i64().unwrap().try_into().unwrap())
        .unwrap()
        .to_string();
    log::debug!(target: "matricks::plugin", "{message}");
    Ok(())
}

/// Make an info log from a plugin
pub(crate) fn plugin_info_log(
    plugin: &mut CurrentPlugin,
    inputs: &[Val],
    _outputs: &mut [Val],
    _user_data: UserData,
) -> Result<(), extism::Error> {
    let message: String = plugin
        .memory_read_str(inputs[0].i64().unwrap().try_into().unwrap())
        .unwrap()
        .to_string();
    log::info!(target: "matricks::plugin", "{message}");
    Ok(())
}

/// Make a warn log from a plugin
pub(crate) fn plugin_warn_log(
    plugin: &mut CurrentPlugin,
    inputs: &[Val],
    _outputs: &mut [Val],
    _user_data: UserData,
) -> Result<(), extism::Error> {
    let message: String = plugin
        .memory_read_str(inputs[0].i64().unwrap().try_into().unwrap())
        .unwrap()
        .to_string();
    log::warn!(target: "matricks::plugin", "{message}");
    Ok(())
}

/// Make an error log from a plugin
pub(crate) fn plugin_error_log(
    plugin: &mut CurrentPlugin,
    inputs: &[Val],
    _outputs: &mut [Val],
    _user_data: UserData,
) -> Result<(), extism::Error> {
    let message: String = plugin
        .memory_read_str(inputs[0].i64().unwrap().try_into().unwrap())
        .unwrap()
        .to_string();
    log::error!(target: "matricks::plugin", "{message}");
    Ok(())
}
