use std::collections::HashMap;
use bincode::{BorrowDecode, Decode, Encode};
use skiplist::OrderedSkipList;

/// SortedSet 结构，使用哈希表 + 跳表实现
///
/// - member_map: 哈希表，提供 O(1) 的 member -> score 查找
/// - score_list: 跳表，按 (score, member) 排序，提供 O(log n) 的范围查询和排名
#[derive(Debug)]
pub struct SortedSet {
    member_map: HashMap<String, f64>,
    score_list: OrderedSkipList<(f64, String)>,
}

impl SortedSet {
    
    pub fn new() -> Self {
        Self {
            member_map: HashMap::new(),
            score_list: OrderedSkipList::new(),
        }
    }

    /// 添加或更新成员
    /// 
    /// # 参数
    /// - `member`: 成员名称
    /// - `score`: 分数
    /// 
    /// # 返回
    /// - `true`: 成员是新增的
    /// - `false`: 成员已存在，只是更新了分数
    pub fn add(&mut self, member: String, score: f64) -> bool {
        let is_new = if let Some(old_score) = self.member_map.get(&member) {
            // 成员已存在，需要先删除旧的 (old_score, member)
            self.score_list.remove(&(*old_score, member.clone()));
            false
        } else {
            true
        };

        // 更新哈希表
        self.member_map.insert(member.clone(), score);

        // 插入新的 (score, member) 到跳表
        self.score_list.insert((score, member));

        is_new
    }

    /// 获取成员的分值
    /// 
    /// # 返回
    /// - `Some(score)`: 成员存在
    /// - `None`: 成员不存在
    pub fn get_score(&self, member: &str) -> Option<f64> {
        self.member_map.get(member).copied()
    }

    /// 删除成员
    /// 
    /// # 返回
    /// - `true`: 成员存在并被删除
    /// - `false`: 成员不存在
    pub fn remove(&mut self, member: &str) -> bool {
        if let Some(score) = self.member_map.remove(member) {
            // 同时从跳表中删除
            self.score_list.remove(&(score, member.to_string()));
            true
        } else {
            false
        }
    }

    /// 获取成员数量
    pub fn len(&self) -> usize {
        self.member_map.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.member_map.is_empty()
    }

    /// 计算成员的排名（从 0 开始，按分数从小到大）
    /// 
    /// # 返回
    /// - `Some(rank)`: 成员存在，返回排名
    /// - `None`: 成员不存在
    pub fn rank(&self, member: &str) -> Option<usize> {
        let score = self.member_map.get(member)?;
        
        // 计算比当前 (score, member) 小的元素数量
        // 跳表已经按 (score, member) 排序，直接遍历计算
        let key = (*score, member.to_string());
        let mut rank = 0;
        for item in self.score_list.iter() {
            if *item < key {
                rank += 1;
            } else {
                break;
            }
        }
        Some(rank)
    }

    /// 获取指定范围的成员（按分数排序）
    /// 
    /// # 参数
    /// - `start`: 起始索引（包含）
    /// - `stop`: 结束索引（包含）
    /// 
    /// # 返回
    /// 成员和分数的元组列表
    pub fn range(&self, start: usize, stop: usize) -> Vec<(String, f64)> {
        self.score_list
            .iter()
            .skip(start)
            .take(stop.saturating_sub(start).saturating_add(1))
            .map(|(score, member)| (member.clone(), *score))
            .collect()
    }

    /// 计算分数范围内的成员数量
    /// 
    /// # 参数
    /// - `min`: 最小分数（包含）
    /// - `max`: 最大分数（包含）
    pub fn count_by_score(&self, min: f64, max: f64) -> usize {
        self.score_list
            .iter()
            .filter(|(score, _)| *score >= min && *score <= max)
            .count()
    }

    /// 获取所有成员（按分数排序）
    pub fn iter(&self) -> impl Iterator<Item = (&String, &f64)> {
        self.score_list
            .iter()
            .map(|(score, member)| (member, score))
    }

    /// 获取所有成员名称（按分数排序）
    pub fn members(&self) -> Vec<String> {
        self.score_list
            .iter()
            .map(|(_, member)| member.clone())
            .collect()
    }

    /// 获取所有成员和分数（按分数排序）
    pub fn members_with_scores(&self) -> Vec<(String, f64)> {
        self.score_list
            .iter()
            .map(|(score, member)| (member.clone(), *score))
            .collect()
    }

    /// 检查成员是否存在
    pub fn contains(&self, member: &str) -> bool {
        self.member_map.contains_key(member)
    }

    /// 获取所有成员名称（用于 ZLEXCOUNT 等命令，按字典序）
    pub fn members_lex(&self) -> Vec<String> {
        let mut members: Vec<String> = self.member_map.keys().cloned().collect();
        members.sort();
        members
    }
}

impl Default for SortedSet {
    fn default() -> Self {
        Self::new()
    }
}


impl Clone for SortedSet {
    fn clone(&self) -> Self {
        let mut new_set = SortedSet::new();
        for (member, score) in &self.member_map {
            new_set.add(member.clone(), *score);
        }
        new_set
    }
}

impl Encode for SortedSet {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let items: Vec<(f64, String)> = self.score_list.iter().cloned().collect();
        items.encode(encoder)
    }
}

impl<Context> Decode<Context> for SortedSet {
    fn decode<D: bincode::de::Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let items: Vec<(f64, String)> = Vec::decode(decoder)?;
        let mut set = SortedSet::new();
        for (score, member) in items {
            set.add(member, score);
        }
        Ok(set)
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for SortedSet {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de, Context = Context>>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let items: Vec<(f64, String)> = Vec::borrow_decode(decoder)?;
        let mut set = SortedSet::new();
        for (score, member) in items {
            set.add(member, score);
        }
        Ok(set)
    }
}