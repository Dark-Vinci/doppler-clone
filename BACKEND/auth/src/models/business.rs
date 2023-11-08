use sea_orm::{ActiveModelBehavior, DeriveEntityModel, Related, RelationDef};
use sea_orm::entity::prelude::*;
use sea_orm::prelude::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "businesses", schema_name = "public")]
pub struct Model {
    #[sea_orm(
    primary_key,
    auto_increment = false,
    column_name = "id",
    )]
    pub id: Uuid,

    #[sea_orm(
    indexed,
    unique,
    column_type = "varchar",
    column_name = "name"
    )]
    pub name: String,

    #[sea_orm(
    default_value = "CURRENT_TIMESTAMP",
    column_name = "created_at",
    )]
    pub created_at: DateTime,

    #[sea_orm(
    default_value = "CURRENT_TIMESTAMP",
    nullable,
    column_name = "updated_at",
    )]
    pub updated_at: Option<DateTime>,

    #[sea_orm(
    column_name = "deleted_at",
    nullable,
    )]
    pub deleted_at: Option<DateTime>,
}

pub enum Relations {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        todo!()
    }
}

impl ActiveModelBehavior for ActiveModel {}
