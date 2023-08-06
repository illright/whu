<script lang="ts">
  export let value: number;
  export let ariaLabelledby: string;
</script>

<div
  class="absolute bottom-0 left-0 h-1.5 w-full"
  role="progressbar"
  aria-labelledby={ariaLabelledby}
  aria-valuenow={value}
  aria-valuemin={0}
  aria-valuemax={1}
>
  <div class="track" style:width="{value * 100}%" />
</div>

<style>
  .track {
    height: 100%;
    background-color: var(--primary);
    box-shadow: 0px 4px 11px 5px rgba(var(--primary-components), 0.6);
    border-top-right-radius: 2px;
  }
</style>
