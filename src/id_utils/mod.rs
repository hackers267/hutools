use nanoid::nanoid;
use ulid::Ulid;
use uuid::Uuid;

pub struct IdUtils;

impl IdUtils {
    /// 使用nanoid生成随机id
    pub fn nano_id() -> String {
        nanoid!()
    }

    /// 使用uuid生成随机id
    pub fn uuid() -> String {
        Uuid::new_v4().to_string()
    }

    /// 使用ulid生成随机id
    pub fn ulid() -> String {
        Ulid::new().to_string()
    }
}
