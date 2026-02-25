use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::MailtoRequest;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_mailto);

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Mailto<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("org.dashchat.mailto", "MailtoPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_mailto)?;
    Ok(Mailto(handle))
}

pub struct Mailto<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Mailto<R> {
    pub fn send(&self, request: MailtoRequest) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("mailto", request)
            .map_err(Into::into)
    }
}
