<script lang="ts">
  import { Tabs } from "radix-svelte";
  import TabTrigger from "./TabTrigger.svelte";
  import { readSettingsStore } from "./settings-store";
  import IconBxsPalette from "~icons/bx/bxs-palette";
  import IconBxsTimer from "~icons/bx/bxs-timer";
  import IconBxsAlarm from "~icons/bx/bxs-alarm";
  import IconBxsWindowAlt from "~icons/bx/bxs-window-alt";
  import IconBxCog from "~icons/bx/bx-cog";
  import IconBxSun from "~icons/bx/bx-sun";
  import IconBxMoon from "~icons/bx/bx-moon";
  import IconBxWindow from "~icons/bx/bx-window";
  import IconBxWindows from "~icons/bx/bx-windows";
  import IconCircleFlagsGb from "~icons/circle-flags/gb";
  import IconCircleFlagsRu from "~icons/circle-flags/ru";
  import Select from "./Select.svelte";
  import ToggleGroup from "./ToggleGroup.svelte";
  import Green from "./colors/Green.svelte";
  import Switch from "./Switch.svelte";
  import Slider from "./Slider.svelte";
  import Input from "./Input.svelte";
  import Button from "./Button.svelte";
  import RadioGroup from "./RadioGroup.svelte";

  const parameters = new URLSearchParams(location.search);
  const settingsFilePath = parameters.get("path") ?? "settings.json";
</script>

<Tabs.Root
  value="language-appearance"
  class="flex h-full w-full py-4 font-mono"
  orientation="vertical"
>
  <Tabs.List
    class="flex flex-col gap-2 border-r border-r-slate-200 px-6 py-4 font-medium"
  >
    <TabTrigger value="language-appearance">
      <IconBxsPalette class="h-5 w-5" />
      Language & appearance
    </TabTrigger>
    <TabTrigger value="short-breaks"
      ><IconBxsTimer class="h-5 w-5" />Short breaks</TabTrigger
    >
    <TabTrigger value="long-breaks"
      ><IconBxsAlarm class="h-5 w-5" />Long breaks</TabTrigger
    >
    <TabTrigger value="whu-in-your-system"
      ><IconBxsWindowAlt class="h-5 w-5" />WHU in your system</TabTrigger
    >
  </Tabs.List>
  <Tabs.Content
    class="flex-1 flex flex-col gap-6 px-6 py-4"
    value="language-appearance"
  >
    <Select
      label="Language"
      name="language"
      options={[
        { value: "en", label: "English", icon: IconCircleFlagsGb },
        { value: "ru", label: "Русский / Russian", icon: IconCircleFlagsRu },
      ]}
    />
    <Select
      label="Accent color"
      name="accent-color"
      options={[{ value: "green", label: "Green", icon: Green }]}
    />
    <ToggleGroup
      label="Light preference"
      name="light-preference"
      options={[
        { value: "auto", label: "Auto", icon: IconBxCog },
        { value: "light", label: "Light", icon: IconBxSun },
        { value: "dark", label: "Dark", icon: IconBxMoon },
      ]}
    />
  </Tabs.Content>
  <Tabs.Content
    class="flex-1 flex flex-col gap-6 px-6 py-4"
    value="short-breaks"
  >
    <Switch
      rootName="short-breaks-enabled"
      label="Short breaks enabled"
      description="Appear quick and often, help against eye strain."
    />
    <Slider name="short-break-period" label="Time between breaks" message="20 minutes" />
    <RadioGroup
      label="How it appears"
      name="short-break-appearance"
      options={[
        {
          value: "notification",
          label: "Notification",
        },
        {
          value: "full-screen",
          label: "Full-screen popup",
        },
      ]}
    />
    <Slider name="short-break-notification" label="Notify before break" message="20 minutes" />
    <Switch
      rootName="short-breaks-postponable"
      label="Postponing allowed"
      description="If you need to finish something quickly but then still take a break afterwards."
    />
    <Switch
      rootName="short-breaks-early-finishable"
      label="Early finishing allowed"
      description="If you want to decide when the break is over. Disabling this is recommended for discipline."
    />
    <Input
      name="short-breaks-passphrase"
      label="Passphrase to force-remove a break"
      description="Type this on the keyboard when the break is shown to make it go away."
    />
    <div class="flex-1" />
    <div class="px-6 pt-4 -mx-6 -mb-4 border-t border-t-slate-200 flex flex-col">
      <Button>Show a test break</Button>
    </div>
  </Tabs.Content>
  <Tabs.Content
    class="flex-1 flex flex-col gap-6 px-6 py-4"
    value="whu-in-your-system"
  >
    <Switch
      rootName="launch-on-startup"
      label="Launch on startup"
      description="When you turn on your device, WHU will start automatically"
    />
    <Switch
      rootName="update-automatically"
      label="Update automatically"
      description="Updates will be installed transparently with no actions required"
    />
    <ToggleGroup
      label="Where to show breaks"
      name="break-location"
      options={[
        {
          value: "active-monitor",
          label: "Active monitor",
          icon: IconBxWindow,
        },
        { value: "all-monitors", label: "All monitors", icon: IconBxWindows },
      ]}
    />
    <Switch
      rootName="monitor-inactivity"
      label="Monitor inactivity"
      description="If you’re away from your device, that is considered a natural break, so you don’t get interrupted right after you come back."
    />
    <Switch rootName="ignore-dnd" label="Show breaks in Do Not Disturb mode" />
  </Tabs.Content>
</Tabs.Root>

<!-- {#await readSettingsStore(settingsFilePath) then { values, save }}
  <form on:submit|preventDefault={save}>
    <fieldset>
      <legend>Short breaks</legend>
      <label for="short_break_period">Period</label>
      <input
        id="short_break_period"
        name="short_break_period"
        type="number"
        value={values.shortBreakPeriod}
      />
    </fieldset>

    <button type="submit">Save</button>
  </form>
{/await} -->

<style>
  :global(html, body, #app) {
    height: 100vh;
    margin: 0;
  }
</style>
