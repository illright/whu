import Settings from "./Settings.svelte";
import "../i18n";
import "../tailwind.css";

const app = new Settings({
  target: document.getElementById("app")!,
});

export default app;
