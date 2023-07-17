<script lang="ts">
  import { createLabel } from "@melt-ui/svelte";
  import { ToggleGroup } from "radix-svelte";
  import type { SvelteComponent } from "svelte";

  export let label: string;
  export let name: string;
  export let options: Array<{
    value: string;
    label: string;
    icon: typeof SvelteComponent;
  }>;

  const asLabel = createLabel();
</script>

<div>
  <div use:asLabel id="{name}-label" class="mb-1 text-sm font-semibold">
    {label}
  </div>
  <ToggleGroup.Root
    class="flex rounded-md border border-slate-300 h-10 p-1 gap-2"
    aria-labelledby="{name}-label"
  >
    {#each options as option (option.value)}
      <ToggleGroup.Item
        value={option.value}
        class="flex-1 flex items-center justify-center gap-2 whitespace-nowrap rounded-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent hover:bg-accent/50 transition-colors"
      >
        <svelte:component this={option.icon} />
        {option.label}
      </ToggleGroup.Item>
    {/each}
  </ToggleGroup.Root>
</div>
