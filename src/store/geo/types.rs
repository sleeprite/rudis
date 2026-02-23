use serde::{Deserialize, Serialize};
use bincode::{Encode, Decode, BorrowDecode};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPoint {
    pub longitude: f64,
    pub latitude: f64,
    pub name: String,

    // 核心扩展：LBS 属性
    // 使用 Option 且 skip_serializing_if，保证在普通 Geo 命令下零开销
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Map<String, serde_json::Value>>,
}

impl GeoPoint {
    pub fn new(name: String, longitude: f64, latitude: f64) -> Self {
        Self {
            longitude,
            latitude,
            name,
            properties: None,
        }
    }
}

impl Encode for GeoPoint {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        self.longitude.encode(encoder)?;
        self.latitude.encode(encoder)?;
        self.name.encode(encoder)?;
        // properties 序列化为 JSON 字符串存储（serde_json::Value 无 bincode::Encode）
        // 注意：增加此字段后，与仅含 lon/lat/name 的旧 RDB 不兼容，需重新持久化或迁移
        let props_str: Option<String> = self
            .properties
            .as_ref()
            .and_then(|m| serde_json::to_string(m).ok());
        props_str.encode(encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for GeoPoint {
    fn decode<D: bincode::de::Decoder<Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let longitude = f64::decode(decoder)?;
        let latitude = f64::decode(decoder)?;
        let name = String::decode(decoder)?;
        let props_str: Option<String> = Option::decode(decoder)?;
        let properties = props_str
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok());
        Ok(Self {
            longitude,
            latitude,
            name,
            properties,
        })
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for GeoPoint {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let longitude = f64::borrow_decode(decoder)?;
        let latitude = f64::borrow_decode(decoder)?;
        let name = String::borrow_decode(decoder)?;
        let props_str: Option<String> = Option::borrow_decode(decoder)?;
        let properties = props_str
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok());
        Ok(Self {
            longitude,
            latitude,
            name,
            properties,
        })
    }
}
// 地理围栏实体 (Fence Entity) - 为 Phase 4 预留
// 暂时定义在这里，确保架构设计的一致性
// #[derive(Debug, Clone)]
// pub struct GeoFence {
//     pub id: String,
//     pub name: String,
//     
//     //TODO:
//     // 实际存储 geometry 会用到 geo crate 的类型，暂时用占位符或具体实现
//     // pub geometry: geo::Polygon<f64>, 
//     pub properties: Option<serde_json::Value>,
// }