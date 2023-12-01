use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

type Data = f32;

async fn task_a(
    mut receiver: UnboundedReceiver<Data>,
    send_to_b: UnboundedSender<Data>,
    _send_to_c: UnboundedSender<Data>,
    send_to_d: UnboundedSender<Data>,
) {
    let initial_data = receiver.recv().await.unwrap();
    println!("[A] Got initial data: {initial_data}");

    println!("[A] Forwarding initial data ({initial_data}) to B and D");
    send_to_b.send(initial_data).unwrap();
    send_to_d.send(initial_data).unwrap();

    let intermediate_data = receiver.recv().await.unwrap();
    println!("[A] Got intermediate data: {intermediate_data}");

    let result = (intermediate_data + initial_data) * 0.7;

    println!("[A] Sending result to B: {result}");
    send_to_b.send(result).unwrap();

    println!("[A] Done! ✅");
}

async fn task_b(
    mut receiver: UnboundedReceiver<Data>,
    send_to_a: UnboundedSender<Data>,
    send_to_c: UnboundedSender<Data>,
    _send_to_d: UnboundedSender<Data>,
) {
    let initial_data = receiver.recv().await.unwrap();
    println!("[B] Got initial data: {initial_data}");

    let intermediate_data = initial_data.powi(3);

    println!("[B] Forwarding intermediate data ({intermediate_data}) to A and C");
    send_to_a.send(intermediate_data).unwrap();
    send_to_c.send(intermediate_data).unwrap();

    let first_summand = receiver.recv().await.unwrap();
    println!("[B] Got first summand: {first_summand}");
    let second_summand = receiver.recv().await.unwrap();
    println!("[B] Got second summand: {second_summand}");
    let third_summand = receiver.recv().await.unwrap();
    println!("[B] Got third summand: {third_summand}");

    let result = first_summand + second_summand + third_summand;
    println!("[B] BINGO: {result}");

    println!("[B] Done! ✅");
}

async fn task_c(
    mut receiver: UnboundedReceiver<Data>,
    mut file: UnboundedReceiver<Data>,
    _send_to_a: UnboundedSender<Data>,
    send_to_b: UnboundedSender<Data>,
    send_to_d: UnboundedSender<Data>,
) {
    let initial_data = receiver.recv().await.unwrap();
    println!("[C] Got initial data: {initial_data}");

    let intermediate_data = initial_data.powi(3);

    println!("[C] Forwarding intermediate data ({intermediate_data}) to D");
    send_to_d.send(intermediate_data).unwrap();

    let file_content = file.recv().await.unwrap();
    println!("[C] Got file content: {file_content}");

    println!("[C] Forwarding file content ({file_content}) to B");
    send_to_b.send(file_content).unwrap();

    println!("[C] Done! ✅");
}

async fn task_d(
    mut receiver: UnboundedReceiver<Data>,
    _send_to_a: UnboundedSender<Data>,
    send_to_b: UnboundedSender<Data>,
    _send_to_c: UnboundedSender<Data>,
) {
    let first_input = receiver.recv().await.unwrap();
    println!("[D] Got first input: {first_input}");

    let second_input = receiver.recv().await.unwrap();
    println!("[D] Got second input: {second_input}");

    let result = (first_input * second_input) / (first_input + second_input);
    println!("[D] Sending result to B: {result}");
    send_to_b.send(result).unwrap();

    println!("[D] Done! ✅");
}

#[tokio::main]
async fn main() {
    // ----------- BUILD CHANNELS ------------------------------------------------------------------
    let (send_to_a, a_receive) = tokio::sync::mpsc::unbounded_channel();
    let (send_to_b, b_receive) = tokio::sync::mpsc::unbounded_channel();
    let (send_to_c, c_receive) = tokio::sync::mpsc::unbounded_channel();
    let (send_to_d, d_receive) = tokio::sync::mpsc::unbounded_channel();
    let (file_writer, file_reader) = tokio::sync::mpsc::unbounded_channel();
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
    tasks.push(tokio::spawn(async move {
        task_c(c_receive, file_reader, sa, sb, sd).await
    }));

    // ----------- SPAWN TASK D --------------------------------------------------------------------
    let [sa, sb, sc, _] = senders.clone();
    tasks.push(tokio::spawn(
        async move { task_d(d_receive, sa, sb, sc).await },
    ));

    // ----------- SEND INITIAL DATA ---------------------------------------------------------------
    senders[0].send(1.5).unwrap();

    // ----------- FILL FILE CONTENT ---------------------------------------------------------------
    file_writer.send(2.7).unwrap();

    // ----------- JOIN TASKS ----------------------------------------------------------------------
    for task in tasks {
        task.await.unwrap();
    }
}
