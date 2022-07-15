if (window.Worker) {
    const myWorker = new Worker(new URL('./js/worker.js', import.meta.url));
    setTimeout(() => {
        myWorker.postMessage(["wasm"]);
        console.log("setTimeout send to worker");
    }, 1000); //1秒后发送消息，用于在编译完成后使 worker 收到消息。
    myWorker.onmessage = (m) => {
        const data = m.data;
        console.log('data from wasm: ' + data); //拿到 wasm 计算的结果
    }
}