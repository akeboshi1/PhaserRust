onmessage = function (m) {
    console.log("test11");
    import("../../pkg/wasm").then(module => {
        module.action("wasm");
        module.test().then((data) => {
            postMessage(data);
        });
        module.my_async_test().then((a) => {
            console.log("my_async_test complete:" + a);
        })
        module.get_from_js().then((a) => {
            console.log(a);
        });
        let interval = new module.TestInterval(1000,"123123");
        //https://user-images.githubusercontent.com/18412751/144263975-3b6b42e4-be34-4341-943c-5e851b99e233.png
        module.loadTest("http://localhost:8080/assets/test.txt", (url) => {
            // url = ProgressEvent
            console.log("loadTest123:", url);
            this.postMessage(url.target.response);
            // todo cancel interval code
            // interval.cancel();
        }).then((request) => {
            self.request = request;
            console.log("load complete", this);
        });
        const num = module.wasm_add(10, 5);
        run(num);
        function run(num) {
            postMessage("wasm back");
            postMessage(num);
        }
    })
}




