<script lang="ts">
  import { createLabel } from "@melt-ui/svelte";
  import type { HTMLInputAttributes } from "svelte/elements";

  export let name: string;
  export let label: string;
  export let description: string;

  export let value: HTMLInputAttributes["value"] = undefined;

  const asLabel = createLabel();
</script>

<div>
  <label class="block mb-1 text-sm font-semibold" for={name} use:asLabel>
    {label}
  </label>
  <input
    id={name}
    {name}
    aria-describedby={description !== null ? `${name}-description` : undefined}
    class="flex h-10 w-full rounded-md border border-slate-200 bg-transparent px-3 py-2 text-sm placeholder:text-slate-600 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-green-500 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 tracking-individual"
    bind:value
    on:blur
    on:change
    on:click
    on:focus
    on:keydown
    on:keypress
    on:keyup
    on:mouseover
    on:mouseenter
    on:mouseleave
    on:paste
    on:input
    {...$$restProps}
  />
  {#if description !== null}
    <p class="text-xs text-neutral-500 mt-1.5" id="{name}-description">
      {description}
    </p>
  {/if}
</div>
