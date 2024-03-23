use crate::{
    data::{Conf, Reconnect},
    tcp::connection
};

pub async fn task_spawn_ha(
    conf: Conf,
    tasks: &mut Vec<tokio::task::JoinHandle<()>>,
    reconnect: Reconnect,
) {
    let mut counter = 0;
    conf.data.into_iter().for_each(|a| {
        tasks.push(tokio::spawn(async move {
            loop {
                let Err(error) = connection(&a.a1, a.a2.clone()).await else {
                    continue;
                };
                println!("Error processing item\nerror: {}\nconfig: {:?}", error, a);
                println!(
                    "retrying in {}s...",
                    std::time::Duration::from_secs(reconnect.retry_time.unwrap_or(5)).as_secs()
                );

                tokio::time::sleep(std::time::Duration::from_secs(
                    reconnect.retry_time.unwrap_or(5),
                ))
                .await;
                let Some(count) = reconnect.watchdog_timer else {
                    continue;
                };
                counter += 1;
                if counter >= count {
                    println!("failed reattempts, exiting thread");
                    break;
                }
            }
        }));
    });
}
pub async fn task_spawn(conf: Conf, tasks: &mut Vec<tokio::task::JoinHandle<()>>) {
    conf.data.into_iter().for_each(|a| {
        tasks.push(tokio::spawn(async move {
            if let Err(err) = connection(a.a1.clone(), a.a2.clone()).await {
                println!("Error processing item\nerror: {}\n config: {:?}", err, a);
            };
        }));
    });
}
