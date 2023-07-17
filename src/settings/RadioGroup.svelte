<script lang="ts">
  import { createLabel } from "@melt-ui/svelte";
  import type { RadioGroupRootProps } from "radix-svelte";
  import { RadioGroup as RadioGroupPrimitive } from "radix-svelte";

  export let label: string;
  export let name: string;
  export let value: RadioGroupRootProps["value"] = undefined;
  export let options: Array<{ value: string; label: string }>;

  const asLabel = createLabel();
</script>

<div>
  <label class="block mb-1 text-sm font-semibold" for={name} use:asLabel>
    {label}
  </label>
  <RadioGroupPrimitive.Root
    bind:value
    class="grid gap-2"
    id={name}
    {...$$restProps}
    tabindex={undefined}
  >
    {#each options as option (option.value)}
      <label class="flex gap-2 items-center">
        <RadioGroupPrimitive.Item
          value={option.value}
          class="aspect-square h-4 w-4 rounded-full border border-primary text-primary ring-offset-background focus:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
        >
          <RadioGroupPrimitive.Indicator class="flex items-center justify-center">
            <div class="h-2 w-2 bg-current rounded-full" />
          </RadioGroupPrimitive.Indicator>
        </RadioGroupPrimitive.Item>
        {option.label}
      </label>
    {/each}
  </RadioGroupPrimitive.Root>
</div>
