use sea_orm::{
    ActiveModelBehavior,
    DeriveEntityModel,
    EntityTrait,
    PrimaryKeyTrait,
    Related,
    RelationDef,
    RelationTrait,
};
use sea_orm::entity::prelude::{
    DateTime,
    DeriveRelation,
    EnumIter,
    Uuid,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users", schema_name = "public")]
pub struct Model {
    #[sea_orm(
    primary_key,
    auto_increment = false,
    nullable,
    )]
    pub id: Uuid,

    #[sea_orm(
    column_name = "first_name",
    column_type = "varchar"
    )]
    pub first_name: String,

    #[sea_orm(
    column_name = "last_name",
    column_type = "varchar"
    )]
    pub last_name: String,

    #[sea_orm(
    column_name = "date_of_birth",
    )]
    pub date_of_birth: DateTime,

    #[sea_orm(
    column_name = "password",
    )]
    pub password: String,

    #[sea_orm(
    column_name = "created_at",
    default_value = "CURRENT_TIMESTAMP",
    )]
    pub created_at: DateTime,

    #[sea_orm(
    column_name = "updated_at",
    default_value = "CURRENT_TIMESTAMP",
    nullable
    )]
    pub updated_at: Option<DateTime>,

    #[sea_orm(
    column_name = "deleted_at",
    nullable
    )]
    pub deleted_at: Option<DateTime>,
}


impl Related<super::business::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Business.def()
    }

    // fn via() -> Option<RelationDef> {
    //     todo!()
    // }
    // 
    // fn find_related() -> Select<crate::models::business::Entity> {
    //     todo!()
    // }
}

#[derive(Clone, Debug, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    has_many = "super::business::Entity",
    from = "Column::BusinessId",
    to = "super::business::Column::Id",
    )]
    Business
}

impl ActiveModelBehavior for ActiveModel {}
