import { Request } from "https://deno.land/std@0.148.0/http/mod.ts";
import { scriptExist, importUrl } from "./utils.ts";

interface FunctionMoudle {
  init(data: unknown): Promise<unknown>;
  call(request: Request): Promise<Response>;
  unInit(data: unknown): Promise<unknown>;
}

export class Controller {
  remote: string;
  functions: Map<string, FunctionMoudle>;
  refreshId: number;
  index: number;
  data: unknown;

  constructor(remote: string, refreshTime: number) {
    this.remote = remote;
    this.functions = new Map<string, FunctionMoudle>();
    this.data = {};
    this.refreshId = 0;
    this.beginRefresh(refreshTime);
    this.index = 0;
  }

  find = async (filePath: string): Promise<FunctionMoudle | undefined> => {
    if (this.functions.has(filePath)) {
      return this.functions.get(filePath);
    } else {
      return await this.load(filePath);
    }
  };

  call = async (filePath: string, request: Request): Promise<Response> => {
    try {
      const module = await this.find(filePath);
      if (module) {
        return await module.call(request);
      } else {
        return new Response("no found", { status: 404 });
      }
    } catch (e) {
      console.error(e);
      return new Response("error", { status: 500 });
    }
  };

  // 加载
  load = async (
    filePath: string,
    functions: Map<string, FunctionMoudle> = this.functions
  ): Promise<FunctionMoudle | undefined> => {
    // 读取文件路径
    const truePath = await scriptExist(this.remote, `.${filePath}`);
    let module = undefined;
    try {
      // 干掉过去的
      if (functions.has(filePath)) {
        const fn = functions.get(filePath);
        if (fn && fn.unInit) {
          try {
            fn.unInit(this.data);
          } catch (e) {
            console.error(e);
          }
        }
        functions.delete(filePath);
      }
      if (truePath) {
        module = (await importUrl(truePath)) as FunctionMoudle;
        if (!module.call) {
          module = undefined;
        } else if (module.init) {
          module.init(this.data);
          functions.set(filePath, module);
        }
      }
    } catch (e) {
      console.error(e);
    }

    return module;
  };

  // 刷新
  refresh = async () => {
    const functions = new Map<string, FunctionMoudle>();
    for (const filePath of this.functions.keys()) {
      await this.load(filePath, functions);
    }
    this.functions.clear();
    this.functions = functions;
  };

  // 启动计时器
  beginRefresh = (refreshTime: number) => {
    if (this.refreshId == 0) {
      if (refreshTime > 60) {
        this.refreshId = setInterval(() => {
          this.refresh();
        }, refreshTime * 1000);
      }
    }
  };
}
