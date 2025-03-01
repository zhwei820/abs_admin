use crate::service::Setting;
use rbatis::Rbatis;
pub mod dto;
pub mod core;
pub mod persistence;
pub mod vo;

pub fn init_rbatis(config: &Setting) -> Rbatis {
    let rbatis = Rbatis::new();
    if rbatis.is_debug_mode() == false && config.debug.eq(&true) {
        panic!(
            r#"已使用release模式运行，但是仍使用debug模式！请修改 application.yml 中debug配置项为  debug: false"#
        );
    }
    return rbatis;
}
