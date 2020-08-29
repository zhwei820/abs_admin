use std::fs::File;
use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};

///服务启动配置
pub struct ApplicationConfig {
    ///当前服务地址
    pub server_url: String,
    ///日志路径
    pub log_path: String,
    ///redis地址
    pub redis_url: String,
    ///mysql地址
    pub mysql_url: String,
}

///默认配置
impl Default for ApplicationConfig {
    fn default() -> Self {

        let mut yml_data = String::new();
        File::open("src/application.yml")
            .expect("application.yml not exist!")
            .read_to_string(&mut yml_data);

        let docs = YamlLoader::load_from_str(&yml_data).unwrap();
        //读取配置
        let server_url= get_cfg(&docs, "server_url");
        let log_path= get_cfg(&docs, "log_path");
        let redis_url= get_cfg(&docs, "redis_url");
        let mysql_url= get_cfg(&docs, "mysql_url");

        Self {
            server_url: server_url.as_str().unwrap_or("").to_string(),
            log_path: log_path.as_str().unwrap_or("").to_string(),
            redis_url: redis_url.as_str().unwrap_or("").to_string(),
            mysql_url: mysql_url.as_str().unwrap_or("").to_string(),
        }
    }
}


/// 获取配置
/// key: 需要获取配置的key
fn get_cfg<'a>(docs: &'a Vec<Yaml>, key: &str) -> &'a Yaml {
    for x in docs {
        match x {
            Yaml::Hash(hash) => {
                let v = hash.get(&Yaml::String(key.to_string()));
                if v.is_some() {
                    return v.unwrap();
                }
            }
            _ => {}
        }
    }
    panic!(format!("application.yml/ {} not exist!",key))
}