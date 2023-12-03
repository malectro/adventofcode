import { TextLineStream } from "https://deno.land/std@0.208.0/streams/mod.ts";

export function streamInputLines() {
  return Deno.stdin.readable.pipeThrough(
    new TextDecoderStream(),
  ).pipeThrough(new TextLineStream());
}
