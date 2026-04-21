use std::collections::HashMap;

use anyhow::Error;

use crate::{store::db::{Db, Structure}, frame::Frame};
use crate::store::sorted_set::SortedSet;
use super::aggregate_util::{parse_store_options, extract_members_with_scores, Aggregate};

pub struct Zunionstore {
    destination: String,
    keys: Vec<String>,
    weights: Vec<f64>,
    aggregate: Aggregate,
}

impl Zunionstore {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        // ZUNIONSTORE dest numkeys key [key ...] [WEIGHTS ...] [AGGREGATE ...]
        if args.len() < 4 {
            return Err(Error::msg("ERR wrong number of arguments for 'zunionstore' command"));
        }

        let destination = args[1].to_string();
        let num_keys = args[2].parse::<usize>()
            .map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;
        if num_keys == 0 {
            return Err(Error::msg("ERR at least 1 input key is needed for 'zunionstore' command"));
        }
        if args.len() < 3 + num_keys {
            return Err(Error::msg("ERR syntax error"));
        }

        let keys: Vec<String> = args[3..3 + num_keys].iter().map(|s| s.to_string()).collect();
        let tail: Vec<&str> = args[3 + num_keys..].iter().map(|s| s.as_str()).collect();
        let (weights, aggregate) = parse_store_options(&tail, num_keys)?;

        Ok(Zunionstore { destination, keys, weights, aggregate })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let mut acc: HashMap<String, f64> = HashMap::new();

        for (idx, key) in self.keys.iter().enumerate() {
            let weight = self.weights[idx];
            if let Some(structure) = db.records.get(key) {
                let pairs = match extract_members_with_scores(structure) {
                    Ok(p) => p,
                    Err(e) => return Ok(Frame::Error(e.to_string())),
                };
                for (member, score) in pairs {
                    let weighted = score * weight;
                    acc.entry(member)
                        .and_modify(|cur| *cur = self.aggregate.combine(*cur, weighted))
                        .or_insert(weighted);
                }
            }
            // 不存在的源键视为空集，跳过
        }

        if acc.is_empty() {
            // 结果为空：删掉已存在的目的键，不创建新键
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
