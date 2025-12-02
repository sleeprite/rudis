use anyhow::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    frame::Frame,
    store::db::Db,
};

pub struct Info {
    section: Option<String>,
}

impl Info {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        
        let section = if args.len() > 1 {
            Some(args[1].to_lowercase())
        } else {
            None
        };

        Ok(Info { section })
    }

    pub fn apply(self, db: &Db) -> Result<Frame, Error> {
        let info = self.generate_info(db);
        Ok(Frame::BulkString(info))
    }

    fn generate_info(&self, db: &Db) -> String {
        let mut info = String::new();
        
        // Default sections to show
        let show_all = self.section.is_none() || self.section.as_ref().map_or(false, |s| s == "all");
        let show_default = self.section.is_none() || self.section.as_ref().map_or(true, |s| s == "default");
        let show_server = show_all || show_default || self.section.as_ref().map_or(false, |s| s == "server");
        let show_clients = show_all || show_default || self.section.as_ref().map_or(false, |s| s == "clients");
        let show_memory = show_all || show_default || self.section.as_ref().map_or(false, |s| s == "memory");
        let show_persistence = show_all || show_default || self.section.as_ref().map_or(false, |s| s == "persistence");
        let show_stats = show_all || show_default || self.section.as_ref().map_or(false, |s| s == "stats");
        let show_replication = show_all || show_default || self.section.as_ref().map_or(false, |s| s == "replication");
        let show_cpu = show_all || show_default || self.section.as_ref().map_or(false, |s| s == "cpu");
        let show_commandstats = show_all || show_default || self.section.as_ref().map_or(false, |s| s == "commandstats");
        let show_keyspace = show_all || show_default || self.section.as_ref().map_or(false, |s| s == "keyspace");

        // Server section
        if show_server {
            info.push_str("# Server\r\n");
            info.push_str("redis_version:0.1.0\r\n");
            info.push_str("redis_git_sha1:00000000\r\n");
            info.push_str("redis_git_dirty:0\r\n");
            info.push_str("redis_build_id:unknown\r\n");
            info.push_str("redis_mode:standalone\r\n");
            info.push_str("os:Rust\r\n");
            info.push_str("arch_bits:64\r\n");
            info.push_str("multiplexing_api:unknown\r\n");
            info.push_str("gcc_version:0.0.0\r\n");
            info.push_str("process_id:0\r\n");
            
            // Calculate uptime
            if let Ok(startup_time) = SystemTime::now().duration_since(UNIX_EPOCH) {
                let uptime = startup_time.as_secs();
                info.push_str(&format!("uptime_in_seconds:{}\r\n", uptime));
                info.push_str(&format!("uptime_in_days:{}\r\n", uptime / 86400));
            }
            
            info.push_str("hz:10\r\n");
            info.push_str("configured_hz:10\r\n");
            info.push_str("lru_clock:0\r\n");
            info.push_str("executable:/rudis-server\r\n");
            info.push_str("config_file:rudis.conf\r\n\r\n");
        }

        // Clients section
        if show_clients {
            info.push_str("# Clients\r\n");
            info.push_str("connected_clients:1\r\n");
            info.push_str("client_recent_max_input_buffer:0\r\n");
            info.push_str("client_recent_max_output_buffer:0\r\n");
            info.push_str("blocked_clients:0\r\n");
            info.push_str("tracking_clients:0\r\n");
            info.push_str("clients_in_timeout_table:0\r\n\r\n");
        }

        // Memory section
        if show_memory {
            info.push_str("# Memory\r\n");
            // Estimate memory usage based on the number of records
            let memory_used = db.records.len() * 100; // Rough estimate
            info.push_str(&format!("used_memory:{}\r\n", memory_used));
            info.push_str(&format!("used_memory_human:{}B\r\n", memory_used));
            info.push_str("used_memory_rss:0\r\n");
            info.push_str("used_memory_peak:0\r\n");
            info.push_str("used_memory_peak_human:0B\r\n");
            info.push_str("used_memory_lua:0\r\n");
            info.push_str("used_memory_lua_human:0B\r\n");
            info.push_str("maxmemory:0\r\n");
            info.push_str("maxmemory_human:0B\r\n");
            info.push_str("maxmemory_policy:noeviction\r\n");
            info.push_str("mem_fragmentation_ratio:0.00\r\n");
            info.push_str("mem_allocator:jemalloc-0.0.0\r\n\r\n");
        }

        // Persistence section
        if show_persistence {
            info.push_str("# Persistence\r\n");
            info.push_str("loading:0\r\n");
            info.push_str("rdb_changes_since_last_save:0\r\n");
            info.push_str("rdb_bgsave_in_progress:0\r\n");
            info.push_str("rdb_last_save_time:0\r\n");
            info.push_str("rdb_last_bgsave_status:ok\r\n");
            info.push_str("rdb_last_bgsave_time_sec:-1\r\n");
            info.push_str("rdb_current_bgsave_time_sec:-1\r\n");
            info.push_str("rdb_last_cow_size:0\r\n");
            info.push_str("aof_enabled:0\r\n");
            info.push_str("aof_rewrite_in_progress:0\r\n");
            info.push_str("aof_rewrite_scheduled:0\r\n");
            info.push_str("aof_last_rewrite_time_sec:-1\r\n");
            info.push_str("aof_current_rewrite_time_sec:-1\r\n");
            info.push_str("aof_last_bgrewrite_status:ok\r\n");
            info.push_str("aof_last_write_status:ok\r\n");
            info.push_str("aof_last_cow_size:0\r\n");
            info.push_str("module_fork_in_progress:0\r\n");
            info.push_str("module_fork_last_cow_size:0\r\n\r\n");
        }

        // Stats section
        if show_stats {
            info.push_str("# Stats\r\n");
            info.push_str(&format!("total_connections_received:{}\r\n", 1));
            info.push_str(&format!("total_commands_processed:{}\r\n", 1));
            info.push_str("instantaneous_ops_per_sec:0\r\n");
            info.push_str("total_net_input_bytes:0\r\n");
            info.push_str("total_net_output_bytes:0\r\n");
            info.push_str("instantaneous_input_kbps:0.00\r\n");
            info.push_str("instantaneous_output_kbps:0.00\r\n");
            info.push_str("rejected_connections:0\r\n");
            info.push_str("sync_full:0\r\n");
            info.push_str("sync_partial_ok:0\r\n");
            info.push_str("sync_partial_err:0\r\n");
            info.push_str("expired_keys:0\r\n");
            info.push_str("expired_stale_perc:0.00\r\n");
            info.push_str("expired_time_cap_reached_count:0\r\n");
            info.push_str("expire_cycle_cpu_milliseconds:0\r\n");
            info.push_str("evicted_keys:0\r\n");
            info.push_str("keyspace_hits:0\r\n");
            info.push_str("keyspace_misses:0\r\n");
            info.push_str("pubsub_channels:0\r\n");
            info.push_str("pubsub_patterns:0\r\n");
            info.push_str("latest_fork_usec:0\r\n");
            info.push_str("total_forks:0\r\n");
            info.push_str("migrate_cached_sockets:0\r\n");
            info.push_str("slave_expires_tracked_keys:0\r\n");
            info.push_str("active_defrag_hits:0\r\n");
            info.push_str("active_defrag_misses:0\r\n");
            info.push_str("active_defrag_key_hits:0\r\n");
            info.push_str("active_defrag_key_misses:0\r\n");
            info.push_str("tracking_total_keys:0\r\n");
            info.push_str("tracking_total_items:0\r\n");
            info.push_str("tracking_total_prefixes:0\r\n");
            info.push_str("unexpected_error_replies:0\r\n");
            info.push_str("total_reads_processed:0\r\n");
            info.push_str("total_writes_processed:0\r\n");
            info.push_str("io_threaded_reads_processed:0\r\n");
            info.push_str("io_threaded_writes_processed:0\r\n\r\n");
        }

        // Replication section
        if show_replication {
            info.push_str("# Replication\r\n");
            info.push_str("role:master\r\n");
            info.push_str("connected_slaves:0\r\n");
            info.push_str("master_replid:0000000000000000000000000000000000000000\r\n");
            info.push_str("master_replid2:0000000000000000000000000000000000000000\r\n");
            info.push_str("master_repl_offset:0\r\n");
            info.push_str("second_repl_offset:-1\r\n");
            info.push_str("repl_backlog_active:0\r\n");
            info.push_str("repl_backlog_size:1048576\r\n");
            info.push_str("repl_backlog_first_byte_offset:0\r\n");
            info.push_str("repl_backlog_histlen:0\r\n\r\n");
        }

        // CPU section
        if show_cpu {
            info.push_str("# CPU\r\n");
            info.push_str("used_cpu_sys:0.000000\r\n");
            info.push_str("used_cpu_user:0.000000\r\n");
            info.push_str("used_cpu_sys_children:0.000000\r\n");
            info.push_str("used_cpu_user_children:0.000000\r\n");
            info.push_str("used_cpu_sys_main_thread:0.000000\r\n");
            info.push_str("used_cpu_user_main_thread:0.000000\r\n\r\n");
        }

        // Commandstats section
        if show_commandstats {
            info.push_str("# Commandstats\r\n");
            // In a real implementation, we would track command statistics
            info.push_str("cmdstat_info:calls=1,usec=10,usec_per_call=10.00\r\n\r\n");
        }

        // Keyspace section
        if show_keyspace {
            info.push_str("# Keyspace\r\n");
            let db_size = db.records.len();
            info.push_str(&format!("db0:keys={},expires=0,avg_ttl=0\r\n", db_size));
        }

        info
    }
}