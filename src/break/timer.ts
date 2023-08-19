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

export function formatTimeRemaining(timeSeconds: number) {
  const lang = get(locale) ?? "en";
  const $_ = get(_);
  const pluralRules = new Intl.PluralRules(lang);
  const duration = {
    minutes: Math.floor(timeSeconds / 60),
    seconds: Math.floor(timeSeconds % 60),
  };

  if (duration.minutes >= 60) {
    console.error("Not designed to handle durations longer than 1 hour");
  }

  const formatted = listFormatter.format(
    formatDuration(duration, {
      format: [
        duration.minutes !== 0 && "minutes",
        duration.minutes < 2 && "seconds",
      ].filter(Boolean),
      zero: true,
      delimiter: "#",
      locale: locales[lang as keyof typeof locales],
    }).split("#"),
  );

  return `${formatted} ${$_(
    `break.remaining.${pluralRules.select(
      duration.minutes || duration.seconds,
    )}`,
  )}`;
}
