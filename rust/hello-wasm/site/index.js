const js = import('./node_modules/@trdthg/hello-wasm/hello_wasm.js')
js.then(js => {
    js.greet("WebAssembly")
})