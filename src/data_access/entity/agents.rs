//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "agents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: DateTimeUtc,
    pub modified_at: DateTimeUtc,
    pub name: String,
    pub snmp_data_url: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::managed_devices::Entity")]
    ManagedDevices,
}

impl Related<super::managed_devices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ManagedDevices.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
