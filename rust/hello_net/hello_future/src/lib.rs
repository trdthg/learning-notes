#![allow(dead_code, unused_variables)]

use futures::Future;

fn ma() {
    let x = foo1();
}

async fn foo1() -> usize {
    0
}

fn foo2() -> impl Future<Output = usize> {
    async {
        0
    }
}
