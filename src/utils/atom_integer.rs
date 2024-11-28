/**
 * 一个简单的整数原子类型的结构体，用于封装一个整数值。
 */
pub struct AtomInteger {
    value: i64,
}

impl AtomInteger {

    /**
     * 创建一个新的 `AtomInteger` 实例。
     * 
     * 初始化时，`value` 被设置为 0。
     * 
     * # 返回值
     * 
     * 返回一个 `AtomInteger` 实例，其 `value` 为 0。
     */
    pub fn new() -> Self {
        AtomInteger {
            value: 0,
        }
    }

    /**
     * 将 `AtomInteger` 的 `value` 增加 1。
     * 
     * 这个方法会改变 `AtomInteger` 实例的状态，递增其内部的 `value`。
     */
    pub fn increment(&mut self) {
        self.value += 1;
    }

    /**
     * 获取 `AtomInteger` 的当前 `value`。
     * 
     * # 返回值
     * 
     * 返回 `AtomInteger` 实例的当前 `value`。
     */
    pub fn get(&self) -> i64 {
        self.value
    }
}