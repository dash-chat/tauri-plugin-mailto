import { invoke } from "@tauri-apps/api/core";

export interface MailtoOptions {
  email: string;
  subject?: string;
  body?: string;
  attachments?: string[];
}

export async function mailto(options: MailtoOptions): Promise<void> {
  await invoke("plugin:mailto|mailto", { request: options });
}
