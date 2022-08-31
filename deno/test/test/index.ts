import { Request } from "https://deno.land/std@0.148.0/http/mod.ts";

export function call(request: Request) {
  return new Response("hello test/index.ts", { status: 200 });
}
