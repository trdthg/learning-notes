use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};

// 争夺不剧烈时 it is an acceptable strategy
// Using a blocking mutex to guard short critical sections is an acceptable strategy when contention is minimal.
// 但是竞争激烈时就不好用了
// When a lock is contended, the thread executing the task must block and wait on the mutex.
// This will not only block the current task but it will also block all other tasks scheduled on the current thread.
// 所以接下来可以尝试使用hash分配多个 Vec<Mutex<HashMap>>
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;
// let shared = db[hash(key) % db.len()].lock().unwrap()

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let port = 8080;
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    println!("started listener on 127.0.0.1:{}", port);

    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    let shareddb: ShardedDb = Arc::new(vec![
        Mutex::new(HashMap::new()),
        Mutex::new(HashMap::new()),
        Mutex::new(HashMap::new()),
        Mutex::new(HashMap::new()),
        Mutex::new(HashMap::new()),
    ]);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("{}", addr);

        let db = db.clone();
        let shareddb = shareddb.clone();
        // A new task is spawned for each inbound socket. The socket is moved to the new task and processed there.
        tokio::spawn(async move {
            handle_connection(socket, db, shareddb).await;
        });
    }
}

async fn handle_connection(socket: TcpStream, db: Db, shareddb: ShardedDb) {
    // 建立一个redis连接, 根据请求向redis-server发出请求并返回
    use mini_redis::{Command, Connection, Frame};

    // Connection, provided by `mini-redis`, handles parsing frames from the socket
    let mut connection = Connection::new(socket);

    // use `read_frame` to receive a command from the connection
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                shareddb[(hash(&cmd.key()) % shareddb.len() as u64) as usize]
                    .lock()
                    .unwrap()
                    .insert(cmd.key().to_string(), cmd.value().clone());
                db.lock()
                    .unwrap()
                    .insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                if let Some(value) = db.lock().unwrap().get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // let response = Frame::Error("unimplemented".to_string());
        // write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
