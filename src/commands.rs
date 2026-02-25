use tauri::{command, AppHandle, Runtime};

use crate::models::MailtoRequest;
use crate::MailtoExt;
use crate::Result;

#[command]
pub(crate) async fn mailto<R: Runtime>(
    app: AppHandle<R>,
    request: MailtoRequest,
) -> Result<()> {
    app.mailto().send(request)
}
