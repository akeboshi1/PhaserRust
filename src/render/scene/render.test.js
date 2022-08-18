export function renderTest(str) {
    console.log(`RenderTest ${str}`);
    return str;
}

export class RenderTest {
    constructor() {
        this._str = "RenderTest";
    }

    get str() {
        return this._str;
    }

    set str(n) {
        return this._str = n;
    }

    render() {
        return `My string is: ${this.str}`;
    }
}
