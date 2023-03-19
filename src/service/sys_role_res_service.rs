use std::collections::{BTreeMap, HashMap, HashSet};

use crate::domain::dto::{
    RoleAddDTO, RoleEditDTO, RolePageDTO, SysRoleResAddDTO, SysRoleResPageDTO, SysRoleResUpdateDTO,
};
use crate::domain::core::SysRoleRes;
use crate::domain::vo::{SysResVO, SysRoleVO};
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;
use rbatis::plugin::object_id::ObjectId;
use rbatis::rbdc::types::datetime::DateTime;
use rbatis::sql::Page;

use crate::util::options::OptionStringRefUnwrapOrDefault;

/// Role Resource Service
pub struct SysRoleResService {}

impl SysRoleResService {
    pub async fn page(&self, arg: &SysRoleResPageDTO) -> Result<Page<SysRoleVO>> {
        let mut role_page = CONTEXT
            .sys_role_service
            .page(&RolePageDTO {
                page_no: arg.page_no.clone(),
                page_size: arg.page_size.clone(),
                name: arg.name.clone(),
            })
            .await?;
        let all = CONTEXT.sys_res_service.finds_all_map().await?;
        let role_res_map = self.find_role_res_map(&role_page.records).await?;
        role_page.records = self.loop_set_res_vec(role_page.records, &role_res_map, &all)?;
        return Result::Ok(role_page);
    }

    fn loop_find_role_ids(&self, arg: &Vec<SysRoleVO>) -> Vec<String> {
        let mut results = vec![];
        for x in arg {
            results.push(x.inner.id.as_deref().unwrap_or_default().to_string());
            match &x.childs {
                Some(childs) => {
                    let ids = self.loop_find_role_ids(childs);
                    for id in ids {
                        results.push(id);
                    }
                }
                _ => {}
            }
        }
        return results;
    }

    async fn find_role_res_map(
        &self,
        arg: &Vec<SysRoleVO>,
    ) -> Result<HashMap<String, HashSet<SysRoleRes>>> {
        let role_ids = self.loop_find_role_ids(arg);
        let role_res_vec = {
            if role_ids.is_empty() {
                vec![]
            } else {
                SysRoleRes::select_in_column(pool!(), "role_id", &role_ids).await?
            }
        };
        let mut role_res_map: HashMap<String, HashSet<SysRoleRes>> =
            HashMap::with_capacity(role_res_vec.capacity());
        for role_res in role_res_vec {
            let role_id = role_res.role_id.as_deref().unwrap_or_default();
            //remove repeat
            match role_res_map.get_mut(role_id) {
                None => {
                    let role_id = role_id.to_string();
                    let mut sets = HashSet::new();
                    sets.insert(role_res);
                    role_res_map.insert(role_id, sets);
                }
                Some(sets) => {
                    sets.insert(role_res);
                }
            }
        }
        return Ok(role_res_map);
    }

    /// Add the resource
    fn loop_set_res_vec(
        &self,
        arg: Vec<SysRoleVO>,
        role_res_map: &HashMap<String, HashSet<SysRoleRes>>,
        all: &BTreeMap<String, SysResVO>,
    ) -> Result<Vec<SysRoleVO>> {
        let mut data = vec![];
        for mut role in arg {
            let res_ids = role_res_map.get(role.inner.id.as_ref().unwrap_or_def());
            let mut res_vos = vec![];
            if let Some(res_ids) = res_ids {
                for x in res_ids {
                    match all.get(x.res_id.as_ref().unwrap_or_def()) {
                        Some(res) => {
                            res_vos.push(res.clone());
                        }
                        _ => {}
                    }
                }
            }
            role.resources = res_vos;
            if role.childs.is_some() {
                role.childs = Some(self.loop_set_res_vec(
                    role.childs.unwrap_or(vec![]),
                    role_res_map,
                    all,
                )?);
            }
            role.resource_ids = rbatis::make_table_field_vec!(&role.resources, inner.id);
            data.push(role);
        }
        return Ok(data);
    }

    pub async fn add(&self, arg: &SysRoleResAddDTO) -> Result<u64> {
        let (_, role_id) = CONTEXT
            .sys_role_service
            .add(RoleAddDTO::from(arg.clone()))
            .await?;
        return self
            .save_resources(&role_id, arg.resource_ids.clone())
            .await;
    }

    pub async fn edit(&self, arg: &SysRoleResUpdateDTO) -> Result<u64> {
        let role_id = arg
            .id
            .as_ref()
            .ok_or_else(|| Error::from("角色id不能为空！"))?;
        CONTEXT
            .sys_role_service
            .edit(RoleEditDTO::from(arg.clone()))
            .await?;
        return self.save_resources(role_id, arg.resource_ids.clone()).await;
    }

    async fn save_resources(&self, role_id: &str, resource_ids: Vec<String>) -> Result<u64> {
        self.remove_by_role_id(role_id).await?;
        let mut sys_role_res = vec![];
        for resource_id in resource_ids {
            sys_role_res.push(SysRoleRes {
                id: ObjectId::new().to_string().into(),
                role_id: role_id.to_string().into(),
                res_id: resource_id.clone().into(),
                create_date: DateTime::now().set_micro(0).into(),
            });
        }
        Ok(SysRoleRes::insert_batch(pool!(), &sys_role_res, 20)
            .await?
            .rows_affected)
    }

    ///Roles, user relationships, and rights are deleted
    pub async fn remove_role(&self, role_id: &str) -> Result<u64> {
        let remove_roles = CONTEXT.sys_role_service.remove(role_id).await?;
        let remove_user_roles = CONTEXT
            .sys_user_role_service
            .remove_by_role_id(role_id)
            .await?;
        let remove_role_res = CONTEXT
            .sys_role_res_service
            .remove_by_role_id(role_id)
            .await?;
        return Ok(remove_roles + remove_user_roles + remove_role_res);
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        Ok(SysRoleRes::delete_by_column(pool!(), "id", id)
            .await?
            .rows_affected)
    }

    pub async fn remove_by_res_id(&self, res_id: &str) -> Result<u64> {
        Ok(SysRoleRes::delete_by_column(pool!(), "res_id", res_id)
            .await?
            .rows_affected)
    }

    pub async fn remove_by_role_id(&self, role_id: &str) -> Result<u64> {
        Ok(SysRoleRes::delete_by_column(pool!(), "role_id", role_id)
            .await?
            .rows_affected)
    }
}
