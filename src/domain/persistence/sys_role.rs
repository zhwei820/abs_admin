use crate::domain::core::*;
crud!(SysRole {});
impl_select_page!(SysRole{select_page_by_name(name:&str)=>
    "`where del = 0`
    if name != '':
      ` and name like #{'%'+name+'%'}`
    ` and parent_id IS NULL `
    if !sql.contains('count'):
     `order by create_date desc`"});
