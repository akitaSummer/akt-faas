import { serve } from "https://deno.land/std@0.148.0/http/server.ts";
import { config } from "https://deno.land/x/dotenv/mod.ts";
import { Controller } from "./controller.ts";

// 读取.env文件
config({ export: true });

const port = Deno.env.get("port") ?? "3000";
const remote = Deno.env.get("remote");
const refreshTime = Deno.env.get("refresh_time") ?? 10 * 60;

if (!remote) {
  console.error("Need remote");
  Deno.exit(0);
}

const controller = new Controller(remote, +refreshTime);

serve(
  async (request) => {
    try {
      const base = request.headers.get("host") ?? "http://0.0.0.0/";
      const path = new URL(request.url, base).pathname;
      return await controller.call(path, request);
    } catch (e) {
      console.error("handle request error!");
      return new Response("handle request error!", { status: 500 });
    }
  },
  { hostname: "0.0.0.0", port: +port }
);
