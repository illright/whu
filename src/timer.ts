import { linear } from "svelte/easing";
import { tweened } from "svelte/motion";
import type { Readable } from "svelte/store";

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
