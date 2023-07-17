<script lang="ts">
  import IconBrain from "~icons/bxs/brain";
  import IconBody from "~icons/bx/body";
  import IconConversation from "~icons/bx/conversation";
  import IconDumbbell from "~icons/bx/dumbbell";

  const options = [
    {
      value: "discreet",
      icon: IconBrain,
    },
    {
      value: "stretching",
      icon: IconBody,
    },
    {
      value: "social",
      icon: IconConversation,
    },
    {
      value: "exercise",
      icon: IconDumbbell,
    },
  ];
</script>

<div class="hint-types flex justify-between gap-10 text-[--gray-600]">
  {#each options as { value, icon }}
    <label>
      <input class="peer sr-only" type="radio" name="hint-type" {value} />
      <svelte:component
        this={icon}
        class="w-9 h-9 cursor-pointer peer-checked:text-[--primary]"
      />
    </label>
  {/each}
</div>
