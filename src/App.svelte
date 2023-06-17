<script lang="ts">
  import { WebviewWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import { createTimer } from "./timer";

  const durationMs = 20 * 1000;
  const timer = createTimer(durationMs);

  onMount(timer.start);

  timer.completed.then(() => {
    const window = WebviewWindow.getByLabel("main");
    window.close();
  });
</script>

<main>
  <p>{Math.ceil($timer / 1000)} second(s) remaining</p>
  <progress value={$timer / durationMs} />
</main>
