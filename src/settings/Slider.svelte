<script lang="ts">
  import { createLabel } from "@melt-ui/svelte";
  import { Slider as SliderPrimitive, type SliderRootProps } from "radix-svelte";
  import { twJoin } from "tailwind-merge";

  export let label: string;
  export let name: string;
  export let message: string | null = null;

	let className: string | undefined | null = undefined;
	export { className as class };
  export let value: SliderRootProps["value"] = 0;
  const asLabel = createLabel();
</script>

<div class="grid grid-cols-[1fr,120px] pb-2">
  <label
    class="block text-sm font-semibold mb-2 col-span-2"
    for={name}
    use:asLabel
  >
    {label}
  </label>
  <SliderPrimitive.Root
    bind:value
    id={name}
    class={twJoin(
      "relative flex w-full touch-none select-none items-center",
      className
    )}
    aria-valuetext={message}
    {...$$restProps}
  >
    <SliderPrimitive.Track
      class="relative h-2 w-full grow overflow-hidden rounded-full bg-secondary"
    >
      <SliderPrimitive.Range class="absolute h-full bg-primary" />
    </SliderPrimitive.Track>
    <SliderPrimitive.Thumb
      class="block h-5 w-5 rounded-full border-2 border-primary bg-background ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
    />
  </SliderPrimitive.Root>
  {#if message !== null}
    <p class="text-slate-400 text-xs text-right">{message}</p>
  {/if}
</div>


