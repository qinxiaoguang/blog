use crate::REDIS;

pub fn get_string(key: &str) -> Option<String> {
    let redis = &REDIS;
    let mut con = redis.get_connection().unwrap();
    let res: Option<String> = redis::cmd("GET").arg(key).query(&mut con).ok();
    res
}

// 将对应的session_id放入到redis中，并设置,名字起的有点搓
pub fn store_to_redis<T: redis::ToRedisArgs>(
    key: &str,
    value: T,
    time_s: Option<u32>,
) -> Result<bool, String> {
    let redis = &REDIS;
    let mut con = redis.get_connection().unwrap();
    match time_s {
        Some(time) => {
            let _: () = redis::cmd("SET")
                .arg(key)
                .arg(value)
                .arg("EX")
                .arg(time)
                .query(&mut con)
                .unwrap();
        }
        None => {
            let _: () = redis::cmd("SET")
                .arg(key)
                .arg(value)
                .query(&mut con)
                .unwrap();
        }
    };
    Ok(true)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_redis() {
        match super::get_string("---") {
            Some(res) => println!("{}", res),
            None => println!("none"),
        }
    }
}
