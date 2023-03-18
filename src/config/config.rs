/// Config
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Setting {
    pub debug: bool,
    pub server_url: String,
    pub redis_url: String,
    pub database_url: String,
    pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64,
    pub log_dir: String,
    pub log_temp_size: String,
    pub log_pack_compress: String,
    pub log_rolling_type: String,
    pub log_level: String,
    pub log_chan_len: Option<usize>,
    pub sms_cache_send_key_prefix: String,
    pub jwt_secret: String,
    pub white_list_api: Vec<String>,
    pub cache_type: String,
    pub login_fail_retry: u64,
    pub login_fail_retry_wait_sec: u64,
}

impl Default for Setting {
    fn default() -> Self {
        let yml_data = include_str!("./config.yml");
        //load config
        let result: Setting =
            serde_yaml::from_str(yml_data).expect("load config file fail");
        if result.debug {
            println!("[abs_admin] load config:{:?}", result);
            println!("[abs_admin] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[abs_admin] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}


