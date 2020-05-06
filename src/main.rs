#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use bigdecimal::BigDecimal;

use diesel::insert_into;
use diesel::prelude::*;

mod schema {
    table! {
        users {
            id -> Integer,
            n -> Numeric,
        }
    }
}

#[derive(Queryable, PartialEq, Debug)]
struct User {
    id: i32,
    n: BigDecimal,
}

embed_migrations!("./migrations");

fn main() {
    let conn = establish_connection();
    embedded_migrations::run(&conn).unwrap();
    insert_single_column(&conn).unwrap();
}

fn establish_connection() -> PgConnection {
    let url = ::std::env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&url).unwrap()
}

pub fn insert_single_column(conn: &PgConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    let decimal = BigDecimal::new(50.into(), -2);
    assert_eq!(decimal.to_string(), "5000");
    insert_into(users).values(n.eq(decimal)).execute(conn)
}
