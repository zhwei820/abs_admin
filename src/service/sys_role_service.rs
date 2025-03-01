use crate::domain::dto::{RoleAddDTO, RoleEditDTO, RolePageDTO};
use crate::domain::core::{SysRole, SysRoleRes, SysUserRole};
use crate::domain::vo::{SysResVO, SysRoleVO};
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;
use rbatis::sql::{Page, PageRequest};
use std::collections::{BTreeMap, HashMap};

const RES_KEY: &'static str = "sys_role:all";

///Role of service
pub struct SysRoleService {}

impl SysRoleService {
    pub async fn page(&self, arg: &RolePageDTO) -> Result<Page<SysRoleVO>> {
        let data = SysRole::select_page_by_name(
            pool!(),
            &PageRequest::from(arg),
            arg.name.as_deref().unwrap_or_default(),
        )
        .await?;
        let all_role = self.finds_all_map().await?;
        let mut page = Page::<SysRoleVO>::from(data);
        for mut vo in &mut page.records {
            self.loop_find_childs(&mut vo, &all_role);
        }
        Ok(page)
    }

    /// Role-level data
    pub async fn finds_layer(&self) -> Result<Vec<SysRoleVO>> {
        let all = self.finds_all_map().await?;
        let mut data = vec![];
        for (_k, v) in &all {
            if v.parent_id.is_none() {
                let mut top = SysRoleVO::from(v.clone());
                self.loop_find_childs(&mut top, &all);
                data.push(top);
            }
        }
        return Ok(data);
    }

    pub async fn finds_all(&self) -> Result<Vec<SysRole>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysRole>>>(RES_KEY)
            .await;
        if js.is_err()
            || js.as_ref().unwrap().is_none()
            || js.as_ref().unwrap().as_ref().unwrap().is_empty()
        {
            let all = self.update_cache().await?;
            return Ok(all);
        }
        if CONTEXT.config.debug {
            log::info!("[abs_admin] get from cache:{}", RES_KEY);
        }
        return Ok(js?.unwrap_or_default());
    }

    pub async fn update_cache(&self) -> Result<Vec<SysRole>> {
        let all = SysRole::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        return Ok(all);
    }

    /// All user ids - User Map data
    pub async fn finds_all_map(&self) -> Result<HashMap<String, SysRole>> {
        let all = self.finds_all().await?;
        let mut result = HashMap::with_capacity(all.capacity());
        for x in all {
            result.insert(x.id.as_deref().unwrap_or_default().to_string(), x);
        }
        return Ok(result);
    }

    pub async fn add(&self, arg: RoleAddDTO) -> Result<(u64, String)> {
        let role = SysRole::from(arg);
        let result = (
            SysRole::insert(pool!(), &role).await?.rows_affected,
            role.id.clone().unwrap(),
        );
        self.update_cache().await?;
        Ok(result)
    }

    pub async fn edit(&self, arg: RoleEditDTO) -> Result<u64> {
        let role = SysRole::from(arg);
        let result = SysRole::update_by_column(pool!(), &role, "id").await;
        self.update_cache().await?;
        Ok(result?.rows_affected)
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        let trash = SysRole::select_by_column(pool!(), "id", id).await?;
        let result = SysRole::delete_by_column(pool!(), "id", id).await?;
        CONTEXT.sys_trash_service.add("sys_role", &trash).await?;
        self.update_cache().await?;
        Ok(result.rows_affected)
    }

    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRole>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(SysRole::select_in_column(pool!(), "id", ids).await?)
    }

    pub async fn find_role_res(&self, role_ids: &Vec<String>) -> Result<Vec<SysRoleRes>> {
        if role_ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(SysRoleRes::select_in_column(pool!(), "role_id", role_ids).await?)
    }

    pub async fn find_user_permission(
        &self,
        user_id: &str,
        all_res: &BTreeMap<String, SysResVO>,
    ) -> Result<Vec<String>> {
        let user_roles = SysUserRole::select_by_column(pool!(), "user_id", user_id).await?;
        let role_res = self
            .find_role_res(&rbatis::make_table_field_vec!(&user_roles, role_id))
            .await?;
        let res = CONTEXT
            .sys_res_service
            .finds_layer(&rbatis::make_table_field_vec!(&role_res, res_id), &all_res)
            .await?;
        let permissions = rbatis::make_table_field_vec!(&res, inner.permission);
        return Ok(permissions);
    }

    ///Loop to find the parent-child associative relation array
    pub fn loop_find_childs(&self, arg: &mut SysRoleVO, all: &HashMap<String, SysRole>) {
        let mut childs = vec![];
        for (_key, x) in all {
            if x.parent_id.is_some() && x.parent_id.eq(&arg.inner.id) {
                let mut item = SysRoleVO::from(x.clone());
                self.loop_find_childs(&mut item, all);
                childs.push(item);
            }
        }
        if !childs.is_empty() {
            arg.childs = Some(childs);
        }
    }
}
