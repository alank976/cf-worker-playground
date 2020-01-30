addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const { demo_js_log, handle_and_log_request } = wasm_bindgen;
  await wasm_bindgen(wasm)
  demo_js_log(request.url);
  console.log("request in js world=", JSON.stringify(request));

  let resp = handle_and_log_request(request);
  return new Response(JSON.stringify(resp), { status: 200 })
}
