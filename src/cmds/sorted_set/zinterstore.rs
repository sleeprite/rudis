use std::collections::HashMap;

use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};
use crate::store::sorted_set::SortedSet;
use super::aggregate_util::{parse_store_options, extract_members_with_scores, Aggregate};

pub struct Zinterstore {
    destination: String,
    keys: Vec<String>,
    weights: Vec<f64>,
    aggregate: Aggregate,
}

impl Zinterstore {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        // ZINTERSTORE dest numkeys key [key ...] [WEIGHTS ...] [AGGREGATE ...]
        if args.len() < 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zinterstore' command"));
        }

        let destination = args[1].to_string();
        let num_keys = args[2].parse::<usize>()
            .map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;
        if num_keys == 0 {
            return Err(Error::msg("ERR at least 1 input key is needed for 'zinterstore' command"));
        }
        if args.len() < 3 + num_keys {
            return Err(Error::msg("ERR syntax error"));
        }

        let keys: Vec<String> = args[3..3 + num_keys].iter().map(|s| s.to_string()).collect();
        let tail: Vec<&str> = args[3 + num_keys..].iter().map(|s| s.as_str()).collect();
        let (weights, aggregate) = parse_store_options(&tail, num_keys)?;

        Ok(Zinterstore { destination, keys, weights, aggregate })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 任一源键不存在 → 交集为空（提早返回前要删旧 dest）
        for key in &self.keys {
            if !db.records.contains_key(key) {
                db.records.remove(&self.destination);
                return Ok(Frame::Integer(0));
            }
        }

        // 用第一个源初始化候选集
        let first_pairs = match extract_members_with_scores(db.records.get(&self.keys[0]).unwrap()) {
            Ok(p) => p,
            Err(e) => return Ok(Frame::Error(e.to_string())),
        };
        let w0 = self.weights[0];
        let mut acc: HashMap<String, f64> = first_pairs.into_iter()
            .map(|(m, s)| (m, s * w0))
            .collect();

        // 与后续源做交集 + 聚合
        for i in 1..self.keys.len() {
            let pairs = match extract_members_with_scores(db.records.get(&self.keys[i]).unwrap()) {
                Ok(p) => p,
                Err(e) => return Ok(Frame::Error(e.to_string())),
            };
            let weight = self.weights[i];
            let lookup: HashMap<String, f64> = pairs.into_iter()
                .map(|(m, s)| (m, s * weight))
                .collect();

            // 只保留双方都有的成员
            acc.retain(|m, _| lookup.contains_key(m));
            for (m, cur) in acc.iter_mut() {
                let incoming = lookup[m];
                *cur = self.aggregate.combine(*cur, incoming);
            }
        }

        if acc.is_empty() {
            db.records.remove(&self.destination);
            return Ok(Frame::Integer(0));
        }

        let mut new_set = SortedSet::new();
        for (member, score) in &acc {
            new_set.add(member.clone(), *score);
        }
        let count = new_set.len() as i64;
        db.records.insert(self.destination, Structure::SortedSet(new_set));

        Ok(Frame::Integer(count))
    }
}
