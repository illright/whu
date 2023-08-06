<script lang="ts">
  import { onMount } from "svelte";

  import { createTimer, formatTime } from "./timer";
  import { closeWindow } from "../backend";
  import HintTypePicker from "./HintTypePicker.svelte";
  import { _, isLoading } from "svelte-i18n";

  const parameters = new URLSearchParams(location.search);

  const durationMs = parseInt(parameters.get("duration") || "20", 10) * 1000;
  const timer = createTimer(durationMs);

  onMount(timer.start);
  timer.completed.then(closeWindow);
</script>

{#if !$isLoading}
  <main class="font-mono">
    <h1 class="mb-7 font-sans text-xl font-semibold">
      {parameters.get("title")}
    </h1>
    <p class="mb-14 max-w-[60%] text-center text-lg">
      {parameters.get("description")}
    </p>
    <HintTypePicker />
    <footer class="absolute bottom-0 left-0 w-full">
      <div class="flex justify-between">
        <span id="remaining-time" class="px-16 py-12"
          >{formatTime(Math.ceil($timer / 1000))} {$_("break.remaining")}</span
        >

        <button class="px-16 py-12" on:click={closeWindow}
          >{$_("break.skip")} — <kbd class="font-medium">Cmd+X</kbd></button
        >
      </div>
      <div
        class="progress absolute bottom-0 left-0 h-1.5 w-full"
        role="progressbar"
        aria-labelledby="remaining-time"
        aria-valuenow={$timer / durationMs}
        aria-valuemin={0}
        aria-valuemax={1}
      >
        <div class="track" style:width="{($timer / durationMs) * 100}%" />
      </div>
    </footer>
  </main>
{/if}

<style>
  :global(html, body, #app) {
    height: 100vh;
    margin: 0;
  }

  main {
    --primary-components: 47, 183, 25;
    --primary: rgb(var(--primary-components));
    --text-components: 25, 32, 26;
    --text: rgb(var(--text-components));
    --gray-600: #484848;
    --gray-300: #605e5e;
    --background: #fff;

    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    position: relative;
    background-color: var(--background);
    color: var(--text);

    font-weight: 300;
    letter-spacing: -0.2px;
  }

  @media (prefers-color-scheme: dark) {
    main {
      --primary-components: 31, 206, 2;
      --text-components: 240, 246, 249;
      --gray-600: #999999;
      --gray-300: #bdc0c1;
      --background: #111c20;
    }
  }

  footer button:hover {
    background-color: rgba(var(--text-components), 1%);
  }

  .progress .track {
    height: 100%;
    background-color: var(--primary);
    box-shadow: 0px 4px 11px 5px rgba(var(--primary-components), 0.6);
    border-top-right-radius: 2px;
  }
</style>
