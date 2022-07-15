import("../../pkg/wasm").then(module => {
    onmessage = function (m) {
        module.action("wasm");
        const num = module.wasm_add(10, 5);
        run(num);
    }
    function run(num) {
        postMessage("wasm back");
        postMessage(num);
    }
})

