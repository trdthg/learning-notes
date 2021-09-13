use bytes::Bytes;
use tokio::sync::mpsc;

use mini_redis::client;
use tokio::sync::oneshot;

// tokio prodeaded 4 kinds of channel: 1. mpsc n->1, 2. oneshot 1->1, 3. broadcast n->n, 4. watch 1->n
// 一些细节
// 1. oneshot doesn't require .await, it can complete immediately
// 2. when oneshot's receiver is dropped(which is an acceptable event), send will return a Err

// don't introduce unbounded queue
// loop {
//    // eager await and callback may cause unbounded queue
//    async_op();
// }
// loop {
//    // Will not repeat until `async_op` completes
//    async_op().await;
// }
// 注意bound
#[derive(Debug)]
enum Command {
    //  为了能够找回原本的task, 这里的resp是作为标记存在的 oneshot
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Vec<u8>,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // Create a new channel with a capacity of at most 32
    // The channel is created with a capacity of 32.
    // If messages are sent faster than they are received, the channel will store them.
    // Once the 32 messages are stored in the channel, calling send(...).await will go to sleep until a message has been removed by the receiver.
    // 如果发送消息没有被接受就会阻塞在channel内, 如果队列已满, 则channel不会再发送新的消息
    let (tx, mut rx) = mpsc::channel(32);

    let mut client = client::connect(("127.0.0.1", 6379)).await.unwrap();

    // 接受来自各个task需要发出的信息, 并将结果返回给各个task,
    let manager = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // ignore errors
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val.into()).await;
                    // ignore errors
                    let _ = resp.send(res);
                }
                _ => {}
            }
        }
    });

    // 创建两个线程制造并发环境, 向server发送cmd, 但是由于client没有实现clone, 只能通过channel多对一通过过同一个client发送请求
    // 其他方案
    // open a new connection in every new spawn
    // we couldn't use mutex as '.await' will need to be callled with the lock hold
    // tokio's mutex is not good too, it would only allow in-flight request
    let (tx1, tx2) = (tx.clone(), tx.clone());
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: String::from("hello"),
            resp: resp_tx,
        };
        tx1.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("Got: {:?}", res);
    });
    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: String::from("hello"),
            val: "bar".into(),
            resp: resp_tx,
        };
        tx2.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("Got: {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
