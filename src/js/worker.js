import("../../pkg/wasm").then(module => {
    onmessage = function (m) {
        module.action("wasm");
        run();
    }
    function run() {
        postMessage("wasm back");
    }
})

