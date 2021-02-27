use std::error::Error;

#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;

mod schema;

use schema::users;

#[derive(Queryable, Debug, Insertable, Identifiable)]
struct User {
    id: i32,
    name: String,
}

use diesel::AsChangeset;

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "users"]
struct NewUser {
    name: String
}

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    dotenv::dotenv().ok().expect("env missing");
    let db_url = std::env::var("DATABASE_URL")?;
    let con = SqliteConnection::establish(&db_url)?;

    let new_users = vec!["John Mnemic", "Jane Doe", "Victor 123"].into_iter().map(|name| NewUser { name: name.to_owned() }).collect::<Vec<NewUser>>();
    let new_users_inserted = diesel::insert_into(schema::users::table).values(&new_users).execute(&con)?;
    info!("Inserted users {:?} : {:?}", &new_users, new_users_inserted);

    let users = schema::users::table.get_results::<User>(&con)?;
    info!("users : {:?}", users);

    let janes: Vec<User> = schema::users::table.filter(schema::users::name.like("Ja%")).load::<User>(&con)?;
    info!("janes : {:?}", janes);

    janes.into_iter().for_each(|jane|{
        diesel::update(&jane).set(NewUser { name: "Lara".to_owned()}).execute(&con).ok();
    });

    let users = schema::users::table.get_results::<User>(&con)?;
    info!("users : {:?}", users);

    diesel::delete(schema::users::table).execute(&con)?;
    Ok(())
}
