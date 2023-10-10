use log::{debug, error, info, trace, warn};

pub fn add(left: usize, right: usize) -> usize {
    env_logger::init();
    info!("BlockFilter received exit signal, exit now");
    let block_hash = 255;
    debug!("Latest built block hash {:#x}", block_hash);
    trace!("light-client: new chain root MMR with size = {}", 100);
    let warn_description = "Waring!";
    warn!("Warning! {}!", warn_description);
    let e = "pending";
    error!("notify update_tx_pool_for_reorg error {}", e);
    debug!(
        "Latest built block is main chain, start from {}",
        1 + 1 + 2 + 5 + 6
    );
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
