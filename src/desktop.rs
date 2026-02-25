use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::MailtoRequest;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Mailto<R>> {
    Ok(Mailto(app.clone()))
}

pub struct Mailto<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Mailto<R> {
    pub fn send(&self, request: MailtoRequest) -> crate::Result<()> {
        let MailtoRequest {
            email,
            subject,
            body,
            attachments,
        } = &request;

        if cfg!(target_os = "linux") {
            let mut cmd = std::process::Command::new("xdg-email");
            if let Some(subject) = subject {
                cmd.arg("--subject").arg(subject);
            }
            if let Some(body) = body {
                cmd.arg("--body").arg(body);
            }
            if let Some(attachments) = attachments {
                for path in attachments {
                    cmd.arg("--attach").arg(path);
                }
            }
            cmd.arg(email);
            cmd.spawn()?;
        } else {
            // macOS and Windows: use `open` crate with mailto: URL (no attachment support)
            let mut url = format!("mailto:{}", email);
            let mut params = Vec::new();
            if let Some(subject) = subject {
                params.push(format!(
                    "subject={}",
                    urlencoding(subject)
                ));
            }
            if let Some(body) = body {
                params.push(format!(
                    "body={}",
                    urlencoding(body)
                ));
            }
            if !params.is_empty() {
                url.push('?');
                url.push_str(&params.join("&"));
            }
            open::that(url)?;
        }

        Ok(())
    }
}

fn urlencoding(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            ' ' => result.push_str("%20"),
            '&' => result.push_str("%26"),
            '=' => result.push_str("%3D"),
            '?' => result.push_str("%3F"),
            '#' => result.push_str("%23"),
            '+' => result.push_str("%2B"),
            '%' => result.push_str("%25"),
            '\n' => result.push_str("%0A"),
            '\r' => result.push_str("%0D"),
            _ => result.push(c),
        }
    }
    result
}
