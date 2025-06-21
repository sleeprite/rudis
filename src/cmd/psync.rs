use std::sync::Arc;

use crate::{args::Args, db::DatabaseMessage, db_manager::DatabaseManager, frame::Frame, persistence::rdb_file::RdbFile};
use anyhow::Error;
use tokio::sync::oneshot;

pub struct Psync {}

impl Psync {
    
    pub fn parse_from_frame(_frame: Frame) -> Result<Self, Error> {
        Ok(Psync {})
    }

    pub async fn apply(self, db_manager: Arc<DatabaseManager>, _args: Arc<Args>) -> Result<Frame, Error> {

        // 获取 RDB 快照
        let mut snapshots = Vec::new();
        for sender in db_manager.get_senders() {
            let (tx, rx) = oneshot::channel();
            sender.send(DatabaseMessage::Snapshot(tx)).await?;
            snapshots.push(rx.await?);
        }

        // 构建 RDB 文件
        let rdb = RdbFile::from_snapshots(snapshots);
        let rdb_data = rdb.serialize()?;
        Ok(Frame::RDBFile(rdb_data))
    }
}
