use std::{sync::Arc, time::{Duration, SystemTime}};

use tokio::sync::{mpsc::Sender, oneshot};

use crate::{args::Args, store::db::{DatabaseMessage, Db}, persistence::rdb_file::RdbFile};

/**
 * DB 管理器
 */
pub struct DatabaseManager {
    senders: Vec<Sender<DatabaseMessage>>
}

impl DatabaseManager {

    /**
     * 创建 DB 管理器
     *
     * @param config 参数
     */
    pub fn new(args: Arc<Args>) -> Self {

        let mut dbs = Vec::new();
        let mut senders = Vec::new();
        let mut rdb_file = RdbFile::new(args.dbfilename.clone());
        let _ = rdb_file.load();

        for id in 0..args.databases {
            let db = Db::new(rdb_file.get_database(id));
            senders.push(db.sender.clone());
            dbs.push(db);
        }

        for mut db in dbs {
            tokio::spawn(async move {
                db.run().await;
            });
        }

        let args_clone = args.clone();
        let senders_clone = senders.clone();

        tokio::spawn(async move {
            let period = Duration::from_secs_f64(1.0 / args_clone.hz);
            let mut interval = tokio::time::interval(period);
            loop {

                interval.tick().await;
                for sender in &senders_clone {
                    let _ = sender.send(DatabaseMessage::CleanExpired).await;
                }

                let mut changes = 0;
                for sender in &senders_clone {
                    let (tx, rx) = oneshot::channel();
                    if sender.send(DatabaseMessage::Changes(tx)).await.is_ok() {
                        if let Ok(count) = rx.await {
                            changes += count;
                        }
                    }
                }

                let should_save = {
                    let now = SystemTime::now();
                    let elapsed = now.duration_since(rdb_file.last_save_time).unwrap().as_secs();
                    args_clone.save.iter().any(|rule| {
                        elapsed >= rule.seconds && changes.saturating_sub(rdb_file.last_save_changes) >= rule.changes
                    })
                };

                if should_save {
                    let mut snapshots = Vec::new();
                    for sender in &senders_clone {
                        let (tx, rx) = oneshot::channel();
                        if sender.send(DatabaseMessage::Snapshot(tx)).await.is_ok() {
                            if let Ok(snapshot) = rx.await {
                                snapshots.push(snapshot);
                            }
                        }
                    }

                    for (index, snapshot) in snapshots.into_iter().enumerate() {
                        rdb_file.set_database(index, snapshot); // 更新快照
                    }

                    rdb_file.last_save_time = SystemTime::now();
                    rdb_file.last_save_changes = changes;
                    match rdb_file.save() {
                        Ok(()) => {
                            log::debug!("Successfully persisted dump.RDB");
                            for sender in &senders_clone {
                                let _ = sender.send(DatabaseMessage::ResetChanges).await;
                            }
                        },
                        Err(_) => log::error!("Failed to dump.RDB")
                    };
                }
            }
        });
        DatabaseManager { 
            senders 
        }
    }

    /**
     * 获取发送者
     *
     * @param idx 数据库索引
     */
    pub fn get_sender(&self, idx: usize) -> Sender<DatabaseMessage> {
        if let Some(sender) = self.senders.get(idx) {
            sender.clone()
        } else {
            panic!("Index out of bounds");
        }
    }

    /**
     * 获取发送者【所有】
     */
    pub fn get_senders(&self) -> Vec<Sender<DatabaseMessage>> {
        self.senders.clone()
    }
}