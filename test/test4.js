export function init(data) {
  console.log("init");
}

export function call(request) {
  return new Response("hello test4.js", { status: 200 });
}
