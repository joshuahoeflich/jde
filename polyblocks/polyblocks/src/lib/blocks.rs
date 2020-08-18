use super::cli::Block;
use futures::future::join_all;

pub async fn run_blocks(blocks: Vec<Block>) {
    let mut handles = vec![];
    for block in blocks {
        handles.push(get_join_handle(block));
    }
    join_all(handles).await;
}

fn get_join_handle(mut block: Block) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        if block.interval.as_secs() == 0 {
            run_print_err(&mut block).await;
        } else {
            loop {
                run_print_err(&mut block).await;
                tokio::time::delay_for(block.interval).await;
            }
        }
    })
}

async fn run_print_err(block: &mut Block) {
    if let Err(err) = block.script.output().await {
        eprintln!("{:?}", err);
    }
}
