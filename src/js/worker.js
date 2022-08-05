onmessage = function (m) {
    console.log("test11");
    import("../../pkg/wasm").then(module => {
    module.action("wasm");
        module.test().then((data) => {
            postMessage(data);
        });
        module.addThreadTest().then(()=>{
            
        });
        const num = module.wasm_add(10, 5);
        run(num);
        function run(num) {
            postMessage("wasm back");
            postMessage(num);
        }
    })
}



