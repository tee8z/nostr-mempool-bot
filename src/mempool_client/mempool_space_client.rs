/*
 * TODO:
 * 
 * need methods to listen or poll for
 * 1) utxo movement
 * 2) transaction confirmation height
 * 3) mempool fee hitting a given threshold
 * 4) a given block height has been reached
 */

 pub struct MempoolClient {
    mempool_space: String,
    db_pool: PgPool
}

impl MempoolClient {
    pub async fn build(self, configuration: Settings, db: PgPool) -> MempoolClient {
        Ok((self))
    }

    pub async fn utxo_movements(){

    }

    pub async fn transaction_height(){

    }

    pub async fn mempool_fee(){

    }

    pub async fn block_height(){

    }
}