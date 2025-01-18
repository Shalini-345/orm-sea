use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{Database, DbErr};


mod entities{
    pub mod userentity;
    pub mod rideentity;
    pub mod  driverentity;
    pub mod favorite;
    pub mod helpsupport;
    pub mod payment;
    pub mod recentlocation;
    pub mod settings;
    pub mod vehicleentity;

}


use entities:: driverentity:: Model as DeriveEntityModel;
use entities::favorite::Model as FavoriteModel;
use entities::helpsupport::Model as HelpSupportModel;
use entities::payment::Model as PaymentModel;
use entities::recentlocation::Model as RecentLocationModel;
use entities::userentity::Entity as UserEntity;
use entities::rideentity::Entity as RideEntity;
use entities::settings::Model as SettingsModel;
use entities::vehicleentity::Model as VehicleEntityModel;
use crate::entities::rideentity::Relation::Payment;
pub struct Model {
    pub created_at: DateTime<Utc>, // Specify Utc as the time zone
    pub updated_at: DateTime<Utc>,
    pub password_reset_expires: Option<DateTime<Utc>>,
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    // Connect to the database
    let db = Database::connect("postgres://username:password@localhost/my_database").await?;

    // Query all users
    let users = UserEntity::find().all(&db).await?;
    println!("Users: {:?}", users);

    // Create instances of models (if needed for testing purposes)
    let user = entities::userentity::Model {id:1,email:"shalini@example.com".to_string(),
    password_hash:"hashed_password".to_string(),first_name:"shalini".to_string(),
    last_name:"motupalli".to_string(), phone: todo!(), city: todo!(), 
    state: todo!(), profile_photo: todo!(), remember_token: todo!(), 
    password_reset_token: todo!(), password_reset_expires: todo!(), 
    about_me: todo!(), languages: todo!(), created_at: todo!(), updated_at: todo!() };


    let ride = entities::rideentity::Model {id:1,
        user_id:1,driver_id:1,
        vehicle_id:1, ride_type: todo!(), 
        vehicle_type: todo!(), pickup_location: todo!(), 
        pickup_lat: todo!(), pickup_lng: todo!(), dropoff_location: todo!(), 
        dropoff_lat: todo!(), dropoff_lng: todo!(), scheduled_time: todo!(), 
        start_time: todo!(), end_time: todo!(), status: todo!(), base_fare: todo!(), 
        distance_fare: todo!(), time_fare: todo!(), tip_amount: todo!(), total_amount: todo!(), 
        rating: todo!(), review: todo!(), cancel_reason: todo!(), payment_status: todo!(), 
        payment_id: todo!(), created_at: todo!(), updated_at: todo!() };
        
    

    println!("User: {:?}", user);
    println!("Ride: {:?}", ride);

    Ok(())
}
    


