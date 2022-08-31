import { Request } from "https://deno.land/std@0.148.0/http/mod.ts";

export function init(data: unknown) {
  console.log(import.meta.url);
  console.log("init");
}

export function call(request: Request) {
  return new Response("hello test1.ts", { status: 200 });
}
