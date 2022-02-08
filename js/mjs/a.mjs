const privates = new WeakMap();

export function Public() {
    const me = {
        id: 1,
        name: 'name'
    }

    privates.set(this, me);
}

Public.prototype.method = function () {
    const me = privates.get(this);
    console.log(me.id, me.name);
}

export class Private {
    #me = {
        id: 1,
        name: 'name'
    }

    #method() {
        console.log(this.#me);
    }
}