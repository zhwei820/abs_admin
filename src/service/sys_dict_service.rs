use rbatis::sql::{Page, PageRequest};

use crate::domain::dto::{DictEditDTO, DictPageDTO};
use crate::domain::core::SysDict;
use crate::domain::vo::SysDictVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;

const DICT_KEY: &'static str = "sys_dict:all";

/// dictionary service
pub struct SysDictService {}

impl SysDictService {
    pub async fn page(&self, arg: &DictPageDTO) -> Result<Page<SysDictVO>> {
        let _page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = SysDict::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let page = Page::<SysDictVO>::from(data);
        Ok(page)
    }

    pub async fn add(&self, arg: &SysDict) -> Result<u64> {
        let old =
            SysDict::select_by_column(pool!(), "id", arg.id.as_deref().unwrap_or_default()).await?;
        if old.len() > 0 {
            return Err(Error::from(format!(
                "字典已存在! code={}",
                arg.code.as_deref().unwrap_or_default()
            )));
        }
        let result = Ok(SysDict::insert(pool!(), &arg).await?.rows_affected);
        self.update_cache().await?;
        return result;
    }

    pub async fn edit(&self, arg: &DictEditDTO) -> Result<u64> {
        let data = SysDict::from(arg);
        let result = SysDict::update_by_column(pool!(), &data, "id").await;
        if result.is_ok() {
            self.update_cache().await?;
        }
        return Ok(result?.rows_affected);
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        let targets = SysDict::select_by_column(pool!(), "id", id).await?;
        let r = SysDict::delete_by_column(pool!(), "id", id).await?;
        if r.rows_affected > 0 {
            self.update_cache().await?;
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_dict", &targets).await?;
        }
        Ok(r.rows_affected)
    }

    /// update for all cache
    pub async fn update_cache(&self) -> Result<()> {
        let all = SysDict::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(DICT_KEY, &all).await?;
        return Ok(());
    }
}
