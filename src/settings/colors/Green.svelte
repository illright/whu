<script lang="ts">
  import { twJoin } from "tailwind-merge";

  let _class: string | null = null;

  export { _class as class };
</script>

<div class={twJoin("w-4 h-4 rounded-full bg-green-500", _class)} />
