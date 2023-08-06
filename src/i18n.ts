import { register, init, getLocaleFromQueryString } from "svelte-i18n";

register("en", () => import("../src-tauri/locales/en.yaml"));
register("ru", () => import("../src-tauri/locales/ru.yaml"));

init({
  fallbackLocale: "en",
  initialLocale: getLocaleFromQueryString("lang"),
});
