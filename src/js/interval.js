export function setToken(val) {
    self.intervalToken = val;
    return val;
}
export class Interval {
    constructor() {
        this._token = 0;
    }

    get token() {
        return this._token;
    }

    set token(n) {
        return this._token = n;
    }

    render() {
        return `My number is: ${this.number}`;
    }
}