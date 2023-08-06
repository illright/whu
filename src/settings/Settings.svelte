<script lang="ts">
  import { Tabs } from "radix-svelte";
  import { _, isLoading } from "svelte-i18n";
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

<svelte:head>
  {#if !$isLoading}
    <title>{$_("settings.title")} — WHU</title>
  {/if}
</svelte:head>

{#if !$isLoading}
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
        {$_("settings.tabs.language-appearance")}
      </TabTrigger>
      <TabTrigger value="short-breaks"
        ><IconBxsTimer class="h-5 w-5" />{$_(
          "settings.tabs.short-breaks",
        )}</TabTrigger
      >
      <TabTrigger value="long-breaks"
        ><IconBxsAlarm class="h-5 w-5" />{$_(
          "settings.tabs.long-breaks",
        )}</TabTrigger
      >
      <TabTrigger value="whu-in-your-system"
        ><IconBxsWindowAlt class="h-5 w-5" />{$_(
          "settings.tabs.whu-in-your-system",
        )}</TabTrigger
      >
    </Tabs.List>
    <Tabs.Content
      class="flex-1 flex flex-col gap-6 px-6 py-4"
      value="language-appearance"
    >
      <Select
        label={$_("settings.language-appearance.language")}
        name="language"
        options={[
          { value: "en", label: "English", icon: IconCircleFlagsGb },
          { value: "ru", label: "Русский / Russian", icon: IconCircleFlagsRu },
        ]}
      />
      <Select
        label={$_("settings.language-appearance.accent-color")}
        name="accent-color"
        options={[
          { value: "green", label: $_("settings.colors.green"), icon: Green },
        ]}
      />
      <ToggleGroup
        label={$_("settings.language-appearance.light-preference")}
        name="light-preference"
        options={[
          {
            value: "auto",
            label: $_("settings.light-preference.auto"),
            icon: IconBxCog,
          },
          {
            value: "light",
            label: $_("settings.light-preference.light"),
            icon: IconBxSun,
          },
          {
            value: "dark",
            label: $_("settings.light-preference.dark"),
            icon: IconBxMoon,
          },
        ]}
      />
    </Tabs.Content>
    <Tabs.Content
      class="flex-1 flex flex-col gap-6 px-6 py-4"
      value="short-breaks"
    >
      <Switch
        rootName="short-breaks-enabled"
        label={$_("settings.breaks.short-breaks-enabled.label")}
        description={$_("settings.breaks.short-breaks-enabled.description")}
      />
      <Slider
        name="short-break-period"
        label={$_("settings.breaks.period")}
        message="20 minutes"
      />
      <RadioGroup
        label={$_("settings.breaks.appearance")}
        name="short-break-appearance"
        options={[
          {
            value: "notification",
            label: $_("settings.break-appearance.notification"),
          },
          {
            value: "full-screen",
            label: $_("settings.break-appearance.full-screen"),
          },
        ]}
      />
      <Slider
        name="short-break-notification"
        label={$_("settings.breaks.notification")}
        message="20 minutes"
      />
      <Switch
        rootName="short-breaks-postponable"
        label={$_("settings.breaks.postponable.label")}
        description={$_("settings.breaks.postponable.description")}
      />
      <Switch
        rootName="short-breaks-early-finishable"
        label={$_("settings.breaks.early-finishable.label")}
        description={$_("settings.breaks.early-finishable.description")}
      />
      <Input
        name="short-breaks-passphrase"
        label={$_("settings.breaks.passphrase.label")}
        description={$_("settings.breaks.passphrase.description")}
      />
      <div class="flex-1" />
      <div
        class="px-6 pt-4 -mx-6 -mb-4 border-t border-t-slate-200 flex flex-col"
      >
        <Button>{$_("settings.breaks.show-test-break")}</Button>
      </div>
    </Tabs.Content>
    <Tabs.Content
      class="flex-1 flex flex-col gap-6 px-6 py-4"
      value="whu-in-your-system"
    >
      <Switch
        rootName="launch-on-startup"
        label={$_("settings.whu-in-your-system.launch-on-startup.label")}
        description={$_(
          "settings.whu-in-your-system.launch-on-startup.description",
        )}
      />
      <Switch
        rootName="auto-update"
        label={$_("settings.whu-in-your-system.auto-update.label")}
        description={$_("settings.whu-in-your-system.auto-update.description")}
      />
      <ToggleGroup
        label={$_("settings.whu-in-your-system.break-location")}
        name="break-location"
        options={[
          {
            value: "active-display",
            label: $_("settings.break-location.active-display"),
            icon: IconBxWindow,
          },
          {
            value: "all-displays",
            label: $_("settings.break-location.all-displays"),
            icon: IconBxWindows,
          },
        ]}
      />
      <Switch
        rootName="monitor-inactivity"
        label={$_("settings.whu-in-your-system.monitor-inactivity.label")}
        description={$_(
          "settings.whu-in-your-system.monitor-inactivity.description",
        )}
      />
      <Switch
        rootName="ignore-dnd"
        label={$_("settings.whu-in-your-system.ignore-dnd")}
      />
    </Tabs.Content>
  </Tabs.Root>
{/if}

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
