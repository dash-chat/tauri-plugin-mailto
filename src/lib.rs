use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};
pub use models::MailtoRequest;

#[cfg(desktop)]
use desktop::Mailto;
#[cfg(mobile)]
use mobile::Mailto;

pub trait MailtoExt<R: Runtime> {
    fn mailto(&self) -> &Mailto<R>;
}

impl<R: Runtime, T: Manager<R>> MailtoExt<R> for T {
    fn mailto(&self) -> &Mailto<R> {
        self.state::<Mailto<R>>().inner()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("mailto")
        .invoke_handler(tauri::generate_handler![commands::mailto])
        .setup(|app, api| {
            #[cfg(mobile)]
            let mailto = mobile::init(app, api)?;
            #[cfg(desktop)]
            let mailto = desktop::init(app, api)?;
            app.manage(mailto);
            Ok(())
        })
        .build()
}
