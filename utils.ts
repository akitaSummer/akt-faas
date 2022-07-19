import "https://deno.land/x/fitch@v1.0.0/polyfill.ts";

export const importUrl = async (url: URL): Promise<unknown> => {
  if (url.protocol === "file:") {
    return await import(url.pathname);
  } else {
    return await import(url.href);
  }
};

// 读文件
export const exist = async (
  base: string,
  path: string
): Promise<URL | undefined> => {
  try {
    const url = new URL(path, base);
    const exist = await fetch(url, { method: "HEAD" });
    if (exist.ok) {
      return url;
    }
  } catch (e) {
    console.error(`found ${base} ${path} error`);
    console.error(e);
  }
  return undefined;
};

// 拼后缀
export const existWithExt = async (
  base: string,
  path: string,
  ext: string
): Promise<URL | undefined> => {
  return await exist(base, `${path}.${ext}`);
};

// 处理url
export const scriptExist = async (
  base: string,
  path: string
): Promise<URL | undefined> => {
  if (path.endsWith("/")) {
    return await scriptExist(base, `${path}index`);
  } else {
    const ts = await existWithExt(base, path, "ts");
    if (ts) {
      return ts;
    }
    const js = await existWithExt(base, path, "js");
    if (js) {
      return js;
    }
  }

  return undefined;
};
