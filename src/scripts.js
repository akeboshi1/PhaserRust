if (window.Worker) {
    //加载worker，注意第二个参数要有，否则无法导入 module
    const myWorker = new Worker('./js/worker.js');
    setTimeout(() => {
        myWorker.postMessage(["wasm"]);
        console.log("setTimeout send to worker");
    }, 1000); //1秒后发送消息，用于在编译完成后使 worker 收到消息。
    myWorker.onmessage = (m) => {
        const data = m.data;
        console.log('data from wasm: ' + data); //拿到 wasm 计算的结果
    }
}