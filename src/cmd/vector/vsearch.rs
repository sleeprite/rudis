use anyhow::Error;
use crate::{db::{Db, Structure}, frame::Frame};

#[derive(Debug)]
pub struct Vsearch {
    key: String,
    query: Vec<f32>,
    k: usize,
}

impl Vsearch {

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 2 {
            return Err(Error::msg("ERR wrong number of arguments for 'VSEARCH' command"));
        }

        let key = args[1].to_string();

        // 解析查询向量
        let mut query = Vec::new();
        let mut idx = 2;
        while idx < args.len() && !args[idx].eq_ignore_ascii_case("K") {
            query.push(args[idx].parse::<f32>()?);
            idx += 1;
        }

        if query.is_empty() {
            return Err(Error::msg("ERR query vector cannot be empty"));
        }

        // 解析K参数
        let k = if idx < args.len() && args[idx].eq_ignore_ascii_case("K") {
            idx += 1;
            args.get(idx)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(10)
        } else {
            10
        };

        idx += if idx < args.len() && args[idx-1].eq_ignore_ascii_case("K") { 1 } else { 0 };

        // 检查多余参数
        if idx != args.len() {
            return Err(Error::msg("ERR syntax error"));
        }

        Ok(Self { key, query, k })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let structure = db.records.get(&self.key);

        let results = match structure {
            Some(Structure::VectorCollection(v)) => {
                
                // 维度校验
                if self.query.len() != v.dimension {
                    return Err(Error::msg("ERR query dimension mismatch"));
                }

                // 计算查询向量范数
                let query_norm = self.query.iter()
                    .map(|x| x.powi(2))
                    .sum::<f32>()
                    .sqrt();

                // 计算相似度
                let mut scores = v.vectors.iter()
                    .map(|(id, vec)| {
                        let dot_product = self.query.iter().zip(vec.iter()).map(|(a, b)| a * b).sum::<f32>();
                        let norm = v.norms[id];
                        let similarity = if query_norm == 0.0 || norm == 0.0 {
                            0.0
                        } else {
                            dot_product / (query_norm * norm)
                        };
                        
                        (id.clone(), similarity)
                    })
                    .collect::<Vec<_>>();

                scores.sort_unstable_by(|a, b| 
                    b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                );

                scores.into_iter().take(self.k)
                    .map(|(id, score)| 
                        Frame::Array(vec![
                            Frame::BulkString(score.to_string()),
                            Frame::BulkString(id.into())
                        ])
                    )
                    .collect()
            }
            Some(_) => return Err(Error::msg("WRONGTYPE Operation against a key holding the wrong kind of value")),
            None => Vec::new(),
        };

        Ok(Frame::Array(results))
    }
}