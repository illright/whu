import { Store } from "tauri-plugin-store-api";

export async function readSettingsStore(path: string) {
  const store = new Store(path);

  const shortBreakPeriod =
    (await store.get<number>("short_break_period")) ?? 20 * 60;

  return {
    values: { shortBreakPeriod },
    async save(e: SubmitEvent) {
      const formValues = new FormData(e.target as HTMLFormElement);

      const shortBreakPeriod = parseInt(
        (formValues.get("short_break_period") as string) ?? "",
        10
      );
      if (!Number.isNaN(shortBreakPeriod)) {
        await store.set("short_break_period", shortBreakPeriod);
      }

      await store.save();
    },
  };
}
