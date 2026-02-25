# tauri-plugin-mailto

A Tauri 2 plugin for composing emails with optional attachments. Opens the platform's native email client with pre-filled recipient, subject, body, and file attachments.

## Platform Support

| Platform | Email Compose | Attachments |
|----------|:---:|:---:|
| Linux | `xdg-email` | Yes |
| macOS | `mailto:` URL | No |
| Windows | `mailto:` URL | No |
| Android | Native intent chooser (email apps only) | Yes |
| iOS | `MFMailComposeViewController` / `mailto:` fallback | Yes (via compose view) |

## Installation

### Rust

Add the plugin to your `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-mailto = { git = "https://github.com/nicepkg/tauri-plugin-mailto" }
```

Register it in your Tauri app:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_mailto::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### JavaScript

Install the guest bindings:

```bash
npm install tauri-plugin-mailto-api
```

## Usage

```typescript
import { mailto } from "tauri-plugin-mailto-api";

await mailto({
  email: "recipient@example.com",
  subject: "Hello",
  body: "Message body",
  attachments: ["/path/to/file.pdf"],
});
```

### Options

| Field | Type | Required | Description |
|-------|------|:---:|-------------|
| `email` | `string` | Yes | Recipient email address |
| `subject` | `string` | No | Email subject line |
| `body` | `string` | No | Email body text |
| `attachments` | `string[]` | No | Absolute paths to files to attach |

## Permissions

Add the default permission to your Tauri capability file:

```json
{
  "permissions": ["mailto:default"]
}
```

## License

AGPL-3.0
