addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const { greet, foo, Foo } = wasm_bindgen;
  await wasm_bindgen(wasm)
  const greeting = greet(request.url)
  let fooResult = foo();
  
  // Will return pointer memory info instead of serialized JSON 
  console.log("foo=", JSON.stringify(fooResult));
  return new Response(fooResult.bar, {status: 200})
}
