<script lang="ts">
  import type { SvelteComponent } from "svelte";
  import { twJoin } from "tailwind-merge";
  import { createSelect, createLabel } from "@melt-ui/svelte";
  import IconBxCheck from "~icons/bx/check";
  import IconBxChevronDown from "~icons/bx/chevron-down";

  export let label: string;
  export let name: string;
  export let options: Array<{
    value: string;
    label: string;
    icon: typeof SvelteComponent<{ class?: string | null }>;
  }>;

  const {
    label: labelStore,
    trigger,
    menu,
    option,
    isSelected,
    value,
  } = createSelect({ label, name });
  const asLabel = createLabel();

  $: selectedIcon = options.find((option) => option.value === $value)?.icon;
</script>

<div>
  <label class="block mb-1 text-sm font-semibold" for={name} use:asLabel
    >{label}</label
  >
  <button
    class="flex items-center gap-3 h-10 w-full rounded-md border border-input bg-transparent px-4 py-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
    {...$trigger}
    use:trigger
    aria-label={label}
    id={name}
  >
    {#if selectedIcon !== undefined}
      <svelte:component this={selectedIcon} class="shadow rounded-full" />
    {/if}
    {$labelStore}
    <span class="flex-1" />
    <IconBxChevronDown />
  </button>

  <div
    class={twJoin(
      "menu font-mono bg-background shadow-lg border-input border",
      "z-10 flex max-h-[360px] flex-col gap-1 overflow-y-auto rounded-md p-1"
    )}
    {...$menu}
    use:menu
  >
    {#each options as item}
      <div
        class={twJoin(
          "flex items-center gap-3 cursor-pointer rounded-md py-1 px-3 text-foreground",
          "focus:text-accent-foreground focus:z-10 focus:outline-none",
          "data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground",
          "data-[selected]:bg-accent data-[selected]:text-accent-foreground"
        )}
        {...$option({
          value: item.value,
          label: item.label,
        })}
        use:option
      >
        <svelte:component this={item.icon} class="shadow rounded-full" />
        {item.label}
        <span class="flex-1" />
        {#if $isSelected(item.value)}
          <div class="text-blue-500">
            <IconBxCheck />
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>
