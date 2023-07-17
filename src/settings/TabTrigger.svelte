<script lang="ts">
  import { Tabs } from "radix-svelte";

  export let value: string;
</script>

<Tabs.Trigger
  class="py-1.5 pl-2 pr-4 rounded-md font-normal flex gap-3 items-center data-[state=active]:bg-slate-100 hover:bg-slate-50 leading-5 transition-colors"
  {value}
>
  <slot />
</Tabs.Trigger>
