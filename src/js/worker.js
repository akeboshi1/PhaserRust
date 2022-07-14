onmessage = function (m) {
    console.log("worker onmessage");
    const data = m.data;
    const js = import('../../lib/index_bg');
    js.then((rust)=>{
       run();
    })

    function run() {
        const res = greet(data[0]); //调用具体的 wasm 函数
        postMessage(res);
        console.log("run worker");
    }
}