<script lang="ts">
  import { onMount } from "svelte";
  import IconBrain from "~icons/bxs/brain";
  import IconBody from "~icons/bx/body";
  import IconConversation from "~icons/bx/conversation";
  import IconDumbbell from "~icons/bx/dumbbell";

  import { createTimer, formatTime } from "./timer";
  import { closeWindow } from "./backend";

  const durationMs = 60 * 2 * 1000;
  const timer = createTimer(durationMs);

  onMount(timer.start);
  timer.completed.then(closeWindow);
</script>

<main>
  <h1>Breathing</h1>
  <p class="hint-text">
    Take 5 minutes to breathe deeply and focus on exhaling. It is known to
    induce a sense of calmness and help you relieve stress.
  </p>
  <div class="hint-types">
    <label>
      <input type="radio" name="hint-type" value="discreet" />
      <IconBrain />
    </label>
    <label>
      <input type="radio" name="hint-type" value="stretching" />
      <IconBody />
    </label>
    <label>
      <input type="radio" name="hint-type" value="social" />
      <IconConversation />
    </label>
    <label>
      <input type="radio" name="hint-type" value="exercise" />
      <IconDumbbell />
    </label>
  </div>
  <footer>
    <div class="text">
      <span id="remaining-time"
        >{formatTime(Math.ceil($timer / 1000))} remaining</span
      >

      <button on:click={closeWindow}>Skip — <kbd>Cmd+X</kbd></button>
    </div>
    <div
      class="progress"
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

<style>
  @import "@fontsource/work-sans/600";
  @import "@fontsource/fira-code/300";
  @import "@fontsource/fira-code/500";

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

    font-family: "Fira Code", monospace;
    font-weight: 300;
    font-size: 20px;
    letter-spacing: -0.2px;
  }

  @media (prefers-color-scheme: dark) {
    main {
      --primary-components: 31, 206, 2;
      --text-components: 240, 246, 249;
      --gray-600: #999999;
      --gray-300: #BDC0C1;
      --background: #111C20;
    }
  }

  h1 {
    font-family: "Work Sans", sans-serif;
    font-weight: 600;
    font-size: 40px;
    letter-spacing: 0.2px;
    margin-bottom: 28px;
  }

  .hint-text {
    line-height: 32px;
    text-align: center;
    max-width: 60%;
    margin-bottom: 56px;
  }

  .hint-types {
    display: flex;
    justify-content: space-between;
    gap: 40px;
    color: var(--gray-600);
  }

  .hint-types input {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }

  .hint-types :global(svg) {
    width: 36px;
    height: 36px;
    cursor: pointer;
  }

  .hint-types :checked + :global(svg) {
    color: var(--primary);
  }

  footer {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    font-size: 16px;
    line-height: normal;
  }

  footer .text {
    display: flex;
    justify-content: space-between;
  }

  footer .text > * {
    padding: 48px 64px;
  }

  footer button {
    background: none;
    padding: 0;
    border: none;
    color: inherit;
    font-size: inherit;
    font-family: inherit;
    font-weight: inherit;
    cursor: pointer;
  }

  footer button:hover {
    background-color: rgba(var(--text-components), 1%);
  }

  footer kbd {
    font-size: inherit;
    font-family: inherit;
    font-weight: 500;
  }

  .progress {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 6px;
  }

  .progress .track {
    height: 100%;
    background-color: var(--primary);
    box-shadow: 0px 4px 11px 5px rgba(var(--primary-components), 0.6);
    border-top-right-radius: 2px;
  }
</style>
