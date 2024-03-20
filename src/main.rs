// run as ./target/debug/surrealtest 13 2 1998
#![allow(unused)]
use std::fmt::format;

use clap::Parser;
use colored::Colorize;
use serde::Deserialize;
use surrealdb::engine::local::Db;
use surrealdb::engine::local::RocksDb;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
//use surrealkv::Store;

#[derive(Parser, Debug)]
struct Value {
    /// day number to add
    number_day: i32,
    /// month number to add
    number_month: i32,
    /// year number to add
    number_year: i32,
}

#[derive(Debug, Deserialize)]
struct Magazine {
    name: String,
    price: f32,
    day: u8,
    month: u8,
    year: u32,
}

#[derive(Debug, Deserialize)]
struct Record {
    id: Thing,
    name: String,
    price: f32,
    day: u8,
    month: u8,
    year: u32,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let value = Value::parse();

    let day = value.number_day;
    let month = value.number_month;
    let year = value.number_year;

    let next = format!("Day:{} Month:{} Year:{}",day,month,year);

    println!("{}", next.green());

    // Create database connection
    let mut db_path = std::env::current_dir().unwrap();
    db_path.push("db");
    //println!("{db_path:?}");

    // use embedded RocksDB for storage -try SurrealKV as soon as it compiles ok
    let db = Surreal::new::<RocksDb>(db_path).await?;
    //let db = Surreal::new::<Store>(db_path).await?;

    // Select specific namespace & database
    db.use_ns("test").use_db("test").await?;

    let _cleanup = db.query("REMOVE TABLE product").await?;

    let _response = db
        //-- Create an index on the name, month and year fields of the product table
        // DEFINE INDEX test ON user FIELDS account, email;
        .query("DEFINE INDEX magid ON TABLE product COLUMNS name,month,year UNIQUE")
        .await?;

    let data = vec![
        Magazine {
            name: "Autosport".to_string(),
            price: 1.80,
            day: 12,
            month: 12,
            year: 1987,
        },
        Magazine {
            name: "Autosport".to_string(),
            price: 2.10,
            day: 1,
            month: 9,
            year: 1985,
        },
        Magazine {
            name: "Autosport".to_string(),
            price: 0.90,
            day: 22,
            month: 1,
            year: 1984,
        },
        Magazine {
            name: "Autosport".to_string(),
            price: 1.20,
            day: 23,
            month: 9,
            year: 1984,
        },
    ];

    println!(
        r"
         __                       ____  
   _____/ /_____  ____  ___  ____/ / /_ 
  / ___/ __/ __ \/ __ \/ _ \/ __  / __ \
 (__  ) /_/ /_/ / / / /  __/ /_/ / /_/ /
/____/\__/\____/_/ /_/\___/\__,_/_.___/ "
    );

    add_to(&db, data).await?;
    list_all(&db).await?;
    println!("----------------------------------------------------------------");

    list_year(&db, 1987).await?;
    println!("----------------------------------------------------------------");

    //add_relate(&db,"Senna".to_string());
    list_related(&db);

    Ok(())
}

async fn add_to(db: &Surreal<Db>, data: Vec<Magazine>) -> surrealdb::Result<()> {
    for magazine in data {
        let response = db
            .query(
                "CREATE product SET  name=$name, 
                   price=$price, day=$day, month=$month, year=$year",
            )
            .bind(("name", magazine.name))
            .bind(("price", magazine.price))
            .bind(("day", magazine.day))
            .bind(("month", magazine.month))
            .bind(("year", magazine.year))
            .await?;

        match response.check() {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Could not add entry: '{}'", err);
                return Err(err);
            }
        };
    }
    Ok(())
}

async fn list_all(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let mut entries = db
        .query(
            "SELECT name, price, day, month, year, id 
               FROM type::table($table) ORDER BY name ASC",
        )
        .bind(("table", "product"))
        .await?;
    let entries: Vec<Record> = entries.take(0)?;
    println!("----------------------------------------------------------------");
    println!(
        "{:<12} {:5} {:<2} {:<2} {:<2} {:}",
        "Magazine", "price", "day", "month", "year", "table+id"
    );
    println!("----------------------------------------------------------------");
    for entry in entries {
        println!(
            "{:<12} {:<5.2} {:<3} {:<5} {:<4} {:}",
            entry.name.yellow(),
            entry.price,
            entry.day,
            entry.month,
            entry.year,
            entry.id.to_raw().blue()
        );
    }

    Ok(())
}

async fn list_year(db: &Surreal<Db>, year: u32) -> surrealdb::Result<()> {
    let mut entries = db
        .query("SELECT * FROM type::table($table) WHERE year=$year")
        .bind(("table", "product"))
        .bind(("year", year))
        .await?;
    let entries: Vec<Magazine> = entries.take(0)?;
    for entry in entries {
        println!("{:?} ", entry);
    }
    Ok(())
}

async fn add_relate(db: &Surreal<Db>, topic: String) -> surrealdb::Result<()> {
    let _relate = db
        .query("RELATE product->featured->($topic)")
        .bind(("topic", topic))
        .await?;
    Ok(())
}

async fn list_related(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let mut entries = db
        .query("SELECT * FROM type::table($table)")
        .bind(("table", "featured"))
        .await?;
    let entries: Vec<Record> = entries.take(0)?;
    for entry in entries {
        println!("{:?} ", entry);
    }
    Ok(())
}
