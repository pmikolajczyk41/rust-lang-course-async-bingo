use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

type Data = f32;

async fn task_a(
    receiver: UnboundedReceiver<Data>,
    send_to_b: UnboundedSender<Data>,
    send_to_c: UnboundedSender<Data>,
    send_to_d: UnboundedSender<Data>,
) {
    println!("Hello from task A!");
}

async fn task_b(
    receiver: UnboundedReceiver<Data>,
    send_to_a: UnboundedSender<Data>,
    send_to_c: UnboundedSender<Data>,
    send_to_d: UnboundedSender<Data>,
) {
    println!("Hello from task B!");
}

async fn task_c(
    receiver: UnboundedReceiver<Data>,
    send_to_a: UnboundedSender<Data>,
    send_to_b: UnboundedSender<Data>,
    send_to_d: UnboundedSender<Data>,
) {
    println!("Hello from task C!");
}

async fn task_d(
    receiver: UnboundedReceiver<Data>,
    send_to_a: UnboundedSender<Data>,
    send_to_b: UnboundedSender<Data>,
    send_to_c: UnboundedSender<Data>,
) {
    println!("Hello from task D!");
}

#[tokio::main]
async fn main() {
    // ----------- BUILD CHANNELS ------------------------------------------------------------------
    let (send_to_a, a_receive) = tokio::sync::mpsc::unbounded_channel();
    let (send_to_b, b_receive) = tokio::sync::mpsc::unbounded_channel();
    let (send_to_c, c_receive) = tokio::sync::mpsc::unbounded_channel();
    let (send_to_d, d_receive) = tokio::sync::mpsc::unbounded_channel();
    let senders = [send_to_a, send_to_b, send_to_c, send_to_d];

    let mut tasks = Vec::new();

    // ----------- SPAWN TASK A --------------------------------------------------------------------
    let [_, sb, sc, sd] = senders.clone();
    tasks.push(tokio::spawn(
        async move { task_a(a_receive, sb, sc, sd).await },
    ));

    // ----------- SPAWN TASK B --------------------------------------------------------------------
    let [sa, _, sc, sd] = senders.clone();
    tasks.push(tokio::spawn(
        async move { task_b(b_receive, sa, sc, sd).await },
    ));

    // ----------- SPAWN TASK C --------------------------------------------------------------------
    let [sa, sb, _, sd] = senders.clone();
    tasks.push(tokio::spawn(
        async move { task_c(c_receive, sa, sb, sd).await },
    ));

    // ----------- SPAWN TASK D --------------------------------------------------------------------
    let [sa, sb, sc, _] = senders.clone();
    tasks.push(tokio::spawn(
        async move { task_d(d_receive, sa, sb, sc).await },
    ));

    // ----------- JOIN TASKS ----------------------------------------------------------------------
    for task in tasks {
        task.await.unwrap();
    }
}
