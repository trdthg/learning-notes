
use mini_redis::{ client, Result };


#[tokio::main]
pub async fn main() -> Result<()> {

    // async only allowed inside `async` functions and blocks
    // calling an async fn returns a value representing the operation. This is conceptually analogous to a zero-argument closure. To actually run the operation, you should use the .await operator on the return value.
    // async_test(); 

    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}

async fn async_test() {

    async fn say_hello() {
        println!("world")
    }

    println!("hello");
    say_hello().await;

}
