async function handleRequest(request) {
    return new Response("Hello world!", {
        headers: { "content-type": "text/plain" },
    })
}

addEventListener("fetch", async function (event) {
    event.respondWith(handleRequest(event.request))
})