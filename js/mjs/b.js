import { Public, Private } from "./a.mjs";

let instence = new Public();

instence.method()
console.log(instence.me);

let privateInstance = new Private();

Private.prototype.#method()