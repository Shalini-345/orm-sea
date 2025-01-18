// Ride Entity
use crate::entities::payment;
use crate::entities::userentity;
use sea_orm::DeriveRelation; 
use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, )]
#[sea_orm(table_name = "ride")]


pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub driver_id: i32,
    pub vehicle_id: i32,
    pub ride_type: String, // "immediate", "scheduled"
    pub vehicle_type: String, // "car", "airplane"
    pub pickup_location: String,
    pub pickup_lat: f64,
    pub pickup_lng: f64,
    pub dropoff_location: String,
    pub dropoff_lat: f64,
    pub dropoff_lng: f64,
    pub scheduled_time: Option<DateTime>,
    pub start_time: Option<DateTime>,
    pub end_time: Option<DateTime>,
    pub status: String, // "pending", "accepted", "arrived", "in_progress", "completed", "cancelled"
    pub base_fare: Decimal,
    pub distance_fare: Decimal,
    pub time_fare: Decimal,
    pub tip_amount: Option<Decimal>,
    pub total_amount: Decimal,
    pub rating: Option<i16>,
    pub review: Option<String>,
    pub cancel_reason: Option<String>,
    pub payment_status: String,
    pub payment_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::payment::Entity", from = "Column::PaymentId", to = "super::payment::Column::Id")]
    Payment,
    #[sea_orm(belongs_to = "super::userentity::Entity", from = "Column::UserId", to = "super::userentity::Column::Id")]
    User,
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
    fn via() -> Option<RelationDef> {
        None
    }
}

impl ActiveModelBehavior for ActiveModel {}