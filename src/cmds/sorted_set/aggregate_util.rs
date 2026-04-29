use anyhow::Error;

use crate::store::db::Structure;

/// 聚合方式（ZUNIONSTORE / ZINTERSTORE 的 AGGREGATE 选项）
#[derive(Clone, Copy)]
pub enum Aggregate {
    Sum,
    Min,
    Max,
}

impl Aggregate {
    /// 在新分数和已有分数之间做聚合
    pub fn combine(&self, existing: f64, incoming: f64) -> f64 {
        match self {
            Aggregate::Sum => existing + incoming,
            Aggregate::Min => existing.min(incoming),
            Aggregate::Max => existing.max(incoming),
        }
    }
}

/// 解析 ZUNIONSTORE / ZINTERSTORE 命令尾部的 `[WEIGHTS w1 w2 ...] [AGGREGATE SUM|MIN|MAX]`。
///
/// # 参数
/// - `tail`: numkeys 个 key 之后剩余的参数
/// - `num_keys`: 用于校验 WEIGHTS 长度
///
/// # 返回
/// (weights, aggregate)，weights 默认全 1.0，aggregate 默认 SUM
pub fn parse_store_options(tail: &[&str], num_keys: usize) -> Result<(Vec<f64>, Aggregate), Error> {
    let mut weights: Vec<f64> = vec![1.0; num_keys];
    let mut aggregate = Aggregate::Sum;
    let mut weights_seen = false;
    let mut aggregate_seen = false;

    let mut i = 0;
    while i < tail.len() {
        let tok = tail[i].to_ascii_uppercase();
        match tok.as_str() {
            "WEIGHTS" => {
                if weights_seen {
                    return Err(Error::msg("ERR syntax error"));
                }
                weights_seen = true;
                if i + num_keys >= tail.len() {
                    return Err(Error::msg("ERR syntax error"));
                }
                let mut ws = Vec::with_capacity(num_keys);
                for j in 0..num_keys {
                    let w = tail[i + 1 + j].parse::<f64>()
                        .map_err(|_| Error::msg("ERR weight value is not a float"))?;
                    ws.push(w);
                }
                weights = ws;
                i += 1 + num_keys;
            }
            "AGGREGATE" => {
                if aggregate_seen {
                    return Err(Error::msg("ERR syntax error"));
                }
                aggregate_seen = true;
                if i + 1 >= tail.len() {
                    return Err(Error::msg("ERR syntax error"));
                }
                aggregate = match tail[i + 1].to_ascii_uppercase().as_str() {
                    "SUM" => Aggregate::Sum,
                    "MIN" => Aggregate::Min,
                    "MAX" => Aggregate::Max,
                    _ => return Err(Error::msg("ERR syntax error")),
                };
                i += 2;
            }
            _ => return Err(Error::msg("ERR syntax error")),
        }
    }

    Ok((weights, aggregate))
}

/// 从一个 Structure 中提取 (member, score) 列表。
/// SortedSet 直接取，Set 视每个成员分数为 1.0；其他类型返回错误。
pub fn extract_members_with_scores(structure: &Structure) -> Result<Vec<(String, f64)>, Error> {
    match structure {
        Structure::SortedSet(set) => Ok(set.members_with_scores()),
        Structure::Set(set) => Ok(set.iter().map(|m| (m.clone(), 1.0)).collect()),
        _ => Err(Error::msg("ERR Operation against a key holding the wrong kind of value")),
    }
}
