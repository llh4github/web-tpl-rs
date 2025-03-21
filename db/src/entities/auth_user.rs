//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa :: ToSchema,
)]
#[sea_orm(table_name = "auth_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", unique)]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    #[serde(skip)]
    pub password: String,
    #[sea_orm(column_type = "Text")]
    pub email: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::link_user_role::Entity")]
    LinkUserRole,
}

impl Related<super::link_user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LinkUserRole.def()
    }
}

impl Related<super::auth_role::Entity> for Entity {
    fn to() -> RelationDef {
        super::link_user_role::Relation::AuthRole.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::link_user_role::Relation::AuthUser.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
