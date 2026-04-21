use anyhow::Error;

/// 分数边界（ZRANGEBYSCORE / ZREVRANGEBYSCORE 使用）
pub enum ScoreBound {
    Inclusive(f64),
    Exclusive(f64),
}

/// 解析分数边界：支持 `-inf`、`+inf`、`inf`、`(3` 开区间、`3` 闭区间
pub fn parse_score_bound(s: &str) -> Result<ScoreBound, Error> {
    let (exclusive, rest) = if let Some(r) = s.strip_prefix('(') {
        (true, r)
    } else {
        (false, s)
    };
    let val = match rest.to_ascii_lowercase().as_str() {
        "-inf" => f64::NEG_INFINITY,
        "+inf" | "inf" => f64::INFINITY,
        _ => rest.parse::<f64>()
            .map_err(|_| Error::msg("ERR min or max is not a float"))?,
    };
    Ok(if exclusive { ScoreBound::Exclusive(val) } else { ScoreBound::Inclusive(val) })
}

/// 判断分数是否落入 [min, max] 区间（遵循开闭性）
pub fn score_in_range(score: f64, min: &ScoreBound, max: &ScoreBound) -> bool {
    let ok_min = match min {
        ScoreBound::Inclusive(v) => score >= *v,
        ScoreBound::Exclusive(v) => score > *v,
    };
    let ok_max = match max {
        ScoreBound::Inclusive(v) => score <= *v,
        ScoreBound::Exclusive(v) => score < *v,
    };
    ok_min && ok_max
}

/// 字典序边界（ZRANGEBYLEX / ZREVRANGEBYLEX / ZREMRANGEBYLEX 使用）
pub enum LexBound {
    NegInf,            // "-"
    PosInf,            // "+"
    Inclusive(String), // "[value"
    Exclusive(String), // "(value"
}

/// 解析字典序边界
pub fn parse_lex_bound(s: &str) -> Result<LexBound, Error> {
    if s == "-" {
        return Ok(LexBound::NegInf);
    }
    if s == "+" {
        return Ok(LexBound::PosInf);
    }
    if let Some(v) = s.strip_prefix('[') {
        return Ok(LexBound::Inclusive(v.to_string()));
    }
    if let Some(v) = s.strip_prefix('(') {
        return Ok(LexBound::Exclusive(v.to_string()));
    }
    Err(Error::msg("ERR min or max not valid string range item"))
}

/// 判断成员是否落入字典序 [min, max] 区间
pub fn member_in_lex_range(member: &str, min: &LexBound, max: &LexBound) -> bool {
    let ok_min = match min {
        LexBound::NegInf => true,
        LexBound::PosInf => false,
        LexBound::Inclusive(v) => member >= v.as_str(),
        LexBound::Exclusive(v) => member > v.as_str(),
    };
    let ok_max = match max {
        LexBound::NegInf => false,
        LexBound::PosInf => true,
        LexBound::Inclusive(v) => member <= v.as_str(),
        LexBound::Exclusive(v) => member < v.as_str(),
    };
    ok_min && ok_max
}

/// LIMIT 参数
pub struct Limit {
    pub offset: i64,
    pub count: i64,
}

/// 解析命令尾部的可选 `LIMIT offset count` 和 `WITHSCORES`。
///
/// # 参数
/// - `tail`: 从命令参数里已剥离 key/min/max 之后剩余的那段 slice
/// - `allow_with_scores`: ZRANGEBYLEX 不支持 WITHSCORES，传 false
///
/// # 返回
/// (WITHSCORES 是否出现, LIMIT 参数)
pub fn parse_range_options(
    tail: &[&str],
    allow_with_scores: bool,
) -> Result<(bool, Option<Limit>), Error> {
    let mut with_scores = false;
    let mut limit: Option<Limit> = None;
    let mut i = 0;
    while i < tail.len() {
        let tok = tail[i].to_ascii_uppercase();
        match tok.as_str() {
            "WITHSCORES" => {
                if !allow_with_scores {
                    return Err(Error::msg("ERR syntax error"));
                }
                with_scores = true;
                i += 1;
            }
            "LIMIT" => {
                if i + 2 >= tail.len() {
                    return Err(Error::msg("ERR syntax error"));
                }
                let offset = tail[i + 1].parse::<i64>()
                    .map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;
                let count = tail[i + 2].parse::<i64>()
                    .map_err(|_| Error::msg("ERR value is not an integer or out of range"))?;
                limit = Some(Limit { offset, count });
                i += 3;
            }
            _ => return Err(Error::msg("ERR syntax error")),
        }
    }
    Ok((with_scores, limit))
}

/// 对已筛选结果应用 LIMIT offset count。
/// offset < 0 视为 0；count < 0 视为全部。
pub fn apply_limit<T>(items: Vec<T>, limit: Option<Limit>) -> Vec<T> {
    match limit {
        None => items,
        Some(Limit { offset, count }) => {
            let offset = offset.max(0) as usize;
            if offset >= items.len() {
                return Vec::new();
            }
            let take = if count < 0 {
                items.len() - offset
            } else {
                count as usize
            };
            items.into_iter().skip(offset).take(take).collect()
        }
    }
}
