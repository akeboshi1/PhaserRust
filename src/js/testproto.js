export class TestProto {
    constructor() {
        this._token = 0;
    }

    get token() {
        return this._token;
    }

    set token(n) {
        return this._token = n;
    }

    property(ptr) {
        const obj = Object.getOwnPropertyDescriptor(ptr, '_pbClass');
        console.log("ptr:===",obj.value);
        return obj.value;
    }

    render() {
        return `My number is: ${this.number}`;
    }
}