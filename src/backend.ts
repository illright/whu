import { WebviewWindow } from "@tauri-apps/api/window";

export function closeWindow() {
  const window = WebviewWindow.getByLabel("main");
  window?.close();
}
