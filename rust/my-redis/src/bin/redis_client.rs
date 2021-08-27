use bytes::Bytes;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Vec<u8>,
        resp: Responder<()>,
    }
}

use tokio::sync::oneshot;
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {

    // Create a new channel with a capacity of at most 32 
    let (tx, mut rx) = mpsc::channel(32); // The channel is created with a capacity of 32. If messages are sent faster than they are received, the channel will store them. Once the 32 messages are stored in the channel, calling send(...).await will go to sleep until a message has been removed by the receiver.

    use mini_redis::client;
    let mut client = client::connect(("127.0.0.1", 6379)).await.unwrap();

    // 接受来自server的信息
    let manager = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // ignore errors
                    let _ = resp.send(res);
                },
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val.into()).await;
                    // ignore errors
                    let _ = resp.send(res);
                },
                _ => {},
            }
        }
    });

    // 创建两个线程用来向server发送信息
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

