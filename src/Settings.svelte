<script lang="ts">
  import { readSettingsStore } from "./settings-store";

  const parameters = new URLSearchParams(location.search);
  const settingsFilePath = parameters.get("path") ?? "settings.json";
</script>

{#await readSettingsStore(settingsFilePath) then { values, save }}
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
{/await}
