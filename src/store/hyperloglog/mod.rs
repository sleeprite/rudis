use bincode::{Decode, Encode};

/**
 * HyperLogLog 数据结构
 * 
 * 使用 HyperLogLog 算法进行基数估计，标准误差约 0.81%
 * 使用 16384 (2^14) 个寄存器，每个寄存器存储 6 位（0-63）
 * 
 * 参考 Redis 实现：
 * - 使用密集表示（12KB 内存）
 * - 支持基数估计和合并操作
 */
#[derive(Clone, Encode, Decode)]
pub struct HyperLogLog {
    /// 16384 个寄存器，每个寄存器存储前导零的数量（0-63）
    registers: Vec<u8>,
    
    /// HyperLogLog 修正因子 alpha
    alpha: f64,
    
    /// 缓存的基数估计值（用于优化 PFCOUNT 性能）
    cached_cardinality: Option<u64>,
    
    /// 标记缓存是否有效
    cache_valid: bool,
}

impl HyperLogLog {
    /// 创建新的 HyperLogLog 实例
    pub fn new() -> Self {
        let m = 16384; // 2^14
        Self {
            registers: vec![0; m],
            alpha: 0.7213 / (1.0 + 1.079 / m as f64),
            cached_cardinality: None,
            cache_valid: false,
        }
    }

    /**
     * 重置缓存
     * 
     * 在从 RDB 加载后调用，确保缓存状态正确
     */
    pub fn reset_cache(&mut self) {
        self.cached_cardinality = None;
        self.cache_valid = false;
    }

    /**
     * 添加元素到 HyperLogLog
     * 
     * @param value 要添加的元素
     * @return 如果至少有一个寄存器被更新，返回 true，否则返回 false
     */
    pub fn add(&mut self, value: &str) -> bool {
        let hash = self.hash(value);
        
        // 使用低 14 位作为寄存器索引 (0-16383)
        let j = (hash & 0x3FFF) as usize;
        
        // 使用高 50 位计算前导零数量
        // 右移 14 位后，剩余 50 位用于计算前导零
        let remaining_bits = hash >> 14;
        
        // 计算前导零数量（在 50 位范围内）
        // remaining_bits 是 50 位数据，在 u64 中占据低 50 位（高 14 位是 0）
        // 我们需要计算这 50 位中从最高位（第 49 位）开始的前导零
        // 根据 HyperLogLog 算法，寄存器存储的是 ρ = 1 + leading_zeros
        let r = if remaining_bits == 0 {
            51u8 // 如果高 50 位全为 0，则前导零为 50，ρ = 51
        } else {
            // 将 remaining_bits 左移 14 位，使其占据高 50 位
            // 然后计算前导零，这样得到的就是 50 位范围内的前导零数
            let shifted = remaining_bits << 14;
            let leading_zeros = shifted.leading_zeros();
            // leading_zeros 现在就是 50 位范围内的前导零数（0-50）
            // 然后加 1 得到 ρ 值（1-51）
            (leading_zeros.min(50) + 1) as u8
        };
        
        // 如果新的前导零数大于当前寄存器值，更新寄存器
        if r > self.registers[j] {
            self.registers[j] = r;
            self.cache_valid = false; // 标记缓存失效
            true
        } else {
            false
        }
    }

    /**
     * 估计基数
     * 
     * 使用 HyperLogLog 算法估计集合的基数
     * 
     * @return 估计的基数
     */
    pub fn count(&mut self) -> u64 {
        // 如果缓存有效，直接返回缓存值
        if self.cache_valid {
            return self.cached_cardinality.unwrap_or(0);
        }
        
        let m = self.registers.len() as f64;
        let mut sum = 0.0;
        
        // 计算调和平均数
        for &r in &self.registers {
            // 避免除零错误：如果 r 为 0，则 1 << 0 = 1
            sum += 1.0 / (1u64 << r) as f64;
        }
        
        // 基础估计
        let estimate = self.alpha * m * m / sum;
        
        // 小基数修正（Linear Counting）
        let corrected_estimate = if estimate < 2.5 * m {
            let zeros = self.registers.iter().filter(|&&r| r == 0).count();
            if zeros > 0 {
                m * (m / zeros as f64).ln()
            } else {
                estimate
            }
        } else {
            estimate
        };
        
        let result = corrected_estimate.max(0.0) as u64;
        
        // 缓存结果
        self.cached_cardinality = Some(result);
        self.cache_valid = true;
        
        result
    }

    /**
     * 合并另一个 HyperLogLog
     * 
     * 对每个寄存器，取两个 HyperLogLog 中的最大值
     * 
     * @param other 要合并的另一个 HyperLogLog
     */
    pub fn merge(&mut self, other: &HyperLogLog) {
        for (i, &r) in other.registers.iter().enumerate() {
            if r > self.registers[i] {
                self.registers[i] = r;
            }
        }
        self.cache_valid = false; // 合并后缓存失效
    }

    /**
     * 哈希函数
     * 
     * 使用 FNV-1a 哈希算法，这是一个稳定且分布均匀的哈希函数
     * 适合生产环境使用
     * 
     * @param value 要哈希的值
     * @return 64 位哈希值
     */
    fn hash(&self, value: &str) -> u64 {
        // FNV-1a 哈希算法的常量
        const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
        const FNV_PRIME: u64 = 0x100000001b3;
        
        let mut hash = FNV_OFFSET_BASIS;
        for byte in value.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        hash
    }
}

impl Default for HyperLogLog {
    fn default() -> Self {
        Self::new()
    }
}
