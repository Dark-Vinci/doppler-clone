use sea_orm::{
    ActiveModelTrait,
    ActiveValue,
    DbErr,
    EntityTrait,
};
use uuid::Uuid;

use crate::models::business::{
    self,
    ActiveModel as BusinessActiveModel,
    ActiveModel,
    Model,
};

pub struct BusinessRepository(String);

impl BusinessRepository {
    pub async fn create_business(&self, el: Model) -> Result<ActiveModel, &str> {
        let mut active_model = BusinessActiveModel {
            name: ActiveValue::Set(el.name),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
            deleted_at: ActiveValue::NotSet,
        };

        let insertion = active_model.save(&self.0).await;

        if insertion.is_err() {
            let Err(t) = insertion;

            if t.is(DbErr::AttrNotSet) {
                return Err("not inserted");
            }
        }

        return Ok(insertion.unwrap());
    }

    pub fn update_business(&self, _: Model) -> () {
        // let
    }

    pub async fn get_business_by_id(&self, id: Uuid) -> Result<Model, &'static str> {
        let business = business::Entity::find_by_id(id).
            one(&self.0).
            await;

        if business.is_err() {
            return Err("something something error");
        }

        Ok(business.unwrap().unwrap())
    }

    pub async fn get_businesses(&self, _: Model) -> () {}
}