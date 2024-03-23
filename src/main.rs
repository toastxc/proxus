use futures::future::try_join_all;
use proxus::{
    data::Conf,
    threads::{task_spawn, task_spawn_ha},
};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        println!("you need to specify a path for the configuration");
        return;
    };

    let conf = Conf::from_file(path).unwrap();

    println!("generating connections...");

    let mut tasks = Vec::new();

    match &conf.reconnect {
        None => {
            task_spawn(conf, &mut tasks).await;
        }
        Some(reconnect) => {
            task_spawn_ha(conf.clone(), &mut tasks, reconnect.clone()).await;
        }
    }

    // Await the completion of all tasks
    if let Err(err) = try_join_all(tasks).await {
        println!("THREAD ERROR: {:?}", err);
    }
}
