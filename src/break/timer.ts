import { linear } from "svelte/easing";
import { tweened } from "svelte/motion";
import { get, type Readable } from "svelte/store";
import { formatDuration } from "date-fns";
import { enGB as en, ru } from "date-fns/locale";
import { _, locale } from "svelte-i18n";

const locales = { en, ru };

export interface Timer extends Readable<number> {
  start: () => void;
  completed: Promise<void>;
}

export function createTimer(durationMs: number): Timer {
  const timer = tweened(durationMs, {
    duration: durationMs,
    easing: linear,
  });

  return {
    completed: new Promise<void>((resolve) => setTimeout(resolve, durationMs)),
    start: () => timer.set(0),
    subscribe: timer.subscribe,
  };
}

const listFormatter = new Intl.ListFormat("en", {
  style: "long",
  type: "conjunction",
});

export function formatTime(timeSeconds: number) {
  const duration = {
    hours: Math.floor(timeSeconds / 3600),
    minutes: Math.floor(timeSeconds / 60),
    seconds: Math.floor(timeSeconds % 60),
  };

  return listFormatter.format(
    formatDuration(duration, {
      format: [
        duration.hours !== 0 && "hours",
        duration.hours < 2 && duration.minutes !== 0 && "minutes",
        duration.hours === 0 && duration.minutes < 2 && "seconds",
      ].filter(Boolean),
      zero: true,
      delimiter: "#",
      locale: locales[(get(locale) ?? "en") as keyof typeof locales],
    }).split("#"),
  );
}
