//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub product_id: i32,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub product_number: String,
    pub color: Option<String>,
    pub standard_cost: f64,
    pub list_price: f64,
    pub size: Option<String>,
    pub weight: Option<Decimal>,
    pub product_category_id: Option<i32>,
    pub product_model_id: Option<i32>,
    #[serde(deserialize_with = "super::utils::date_time_from_str")]
    pub sell_start_date: DateTime,
    #[serde(deserialize_with = "super::utils::opt_date_time_from_str")]
    pub sell_end_date: Option<DateTime>,
    #[serde(deserialize_with = "super::utils::opt_date_time_from_str")]
    pub discontinued_date: Option<DateTime>,
    #[serde(deserialize_with = "super::utils::opt_bytes_from_str")]
    pub thumb_nail_photo: Option<String>,
    pub thumbnail_photo_file_name: Option<String>,
    #[sea_orm(unique)]
    pub rowguid: Uuid,
    #[serde(deserialize_with = "super::utils::date_time_from_str")]
    pub created_date: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::product_category::Entity",
        from = "Column::ProductCategoryId",
        to = "super::product_category::Column::ProductCategoryId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ProductCategory,
    #[sea_orm(
        belongs_to = "super::product_model::Entity",
        from = "Column::ProductModelId",
        to = "super::product_model::Column::ProductModelId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ProductModel,
    #[sea_orm(has_many = "super::sales_order_detail::Entity")]
    SalesOrderDetail,
}

impl Related<super::product_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductCategory.def()
    }
}

impl Related<super::product_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductModel.def()
    }
}

impl Related<super::sales_order_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SalesOrderDetail.def()
    }
}

impl Related<super::sales_order_header::Entity> for Entity {
    fn to() -> RelationDef {
        super::sales_order_detail::Relation::SalesOrderHeader.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::sales_order_detail::Relation::Product.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::product_category::Entity")]
    ProductCategory,
    #[sea_orm(entity = "super::product_model::Entity")]
    ProductModel,
    #[sea_orm(entity = "super::sales_order_detail::Entity")]
    SalesOrderDetail,
    #[sea_orm(entity = "super::sales_order_header::Entity")]
    SalesOrderHeader,
}
