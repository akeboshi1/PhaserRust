export class TestScene extends Phaser.Scene {
    constructor(config) {
        super(config);
    }

    init(data) {
        this.worker = data.worker;
    }

    preload(){
        this.load.setPath('assets/demos/');
        // this.load.image("test","assets/test.png");
        this.load.spine('spineboy', 'spineboy.json', 'spineboy.atlas');
    }

    create() {
        const grap = this.add.graphics(undefined);
        grap.fillStyle(0xffcc00, 1);
        grap.fillCircle(100, 100, 15);


        this.startAnim = 'idle'

		this.spineBoy = this.add.spine(400, 600, "spineboy", "idle", true);
		this.cursors = this.input.keyboard.createCursorKeys()
        this.animationNames=[];
		this.initializeAnimationsState(this.spineBoy);

        this.input.on("pointerdown",()=>{
            this.startAnim = this.animationNames[3];
            this.spineBoy.play("run",true);
        },this);

        this.input.on("pointerup",()=>{
            this.startAnim = this.animationNames[0];
            this.spineBoy.play("idle",true);
        },this);
        // this.add.image(450,350,"test");

        setTimeout(() => {
            console.log("setTimeout send to worker");
            this.worker.postMessage(["wasm"]);
        }, 1000); //1秒后发送消息，用于在编译完成后使 worker 收到消息。
        this.worker.onmessage = (m) => {
            const data = m.data;
            console.log('data from wasm: ' + data); //拿到 wasm 计算的结果
            if(m === "test.png"){
                //this.textures.addImage("test.png",);
            }
        }
    }

    initializeAnimationsState(spineGO){
		const startAnim = spineGO.getCurrentAnimation().name

		spineGO.getAnimationList().forEach((name, idx) => {
			this.animationNames.push(name)
			if (name === startAnim)
			{
				this.animationIndex = idx
			}
		})
	}

    changeAnimation(index)
	{
		const name = this.animationNames[index]
		this.spineBoy.play(name, true)
		this.animNameLabel.text = name
	}
}