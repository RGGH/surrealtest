// run as ./target/debug/surrealtest add 13 2 1998
#![allow(unused)]
use clap::Parser;
use colored::Colorize;
use controller::*;
use serde::Deserialize;
use std::fmt::format;
use surrealdb::engine::local::Db;
use surrealdb::engine::local::RocksDb;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
//use surrealkv::Store;

mod controller;

#[derive(Parser, Debug)]
pub struct Value {
    /// add product
    add: String,
    /// day number to add
    number_day: u8,
    /// month number to add
    number_month: u8,
    /// year number to add
    number_year: u32,
}

#[derive(Debug, Deserialize)]
pub struct Magazine {
    name: String,
    price: f32,
    day: u8,
    month: u8,
    year: u32,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    id: Thing,
    name: String,
    price: f32,
    day: u8,
    month: u8,
    year: u32,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // handle user input
    let value = Value::parse();
    let day: u8 = value.number_day;
    let month: u8 = value.number_month;
    let year: u32 = value.number_year;
    let next = format!("Day:{} Month:{} Year:{}", day, month, year);
    println!("{}", next.green());
    let new_mag = Magazine {
        name: "Autosport".to_string(),
        price: 2.30,
        day,
        month,
        year,
    };

    // Create database connection
    let mut db_path = std::env::current_dir().unwrap();
    db_path.push("db");

    // use embedded RocksDB for storage -try SurrealKV as soon as it compiles ok
    let db = Surreal::new::<RocksDb>(db_path).await?;
    //let db = Surreal::new::<Store>(db_path).await?;

    // Select specific namespace & database
    db.use_ns("test").use_db("test").await?;

    // clear old, test data
    //let _cleanup = db.query("REMOVE TABLE product").await?;

    let _response = db
        //-- Create an index on the name, month and year fields of the product table
        .query("DEFINE INDEX magid ON TABLE product COLUMNS name,day,month,year UNIQUE")
        .await?;

    let mut data = vec![
        // Magazine {
        //     name: "Autosport".to_string(),
        //     price: 1.80,
        //     day: 12,
        //     month: 12,
        //     year: 1987,
        // },
        // Magazine {
        //     name: "Autosport".to_string(),
        //     price: 2.10,
        //     day: 1,
        //     month: 9,
        //     year: 1985,
        // },
        // Magazine {
        //     name: "Autosport".to_string(),
        //     price: 0.90,
        //     day: 22,
        //     month: 1,
        //     year: 1984,
        // },
        // Magazine {
        //     name: "Autosport".to_string(),
        //     price: 1.20,
        //     day: 23,
        //     month: 9,
        //     year: 1984,
        // },
    ];
    data.push(new_mag);

    // Banner - read from seperate file for customizing
    println!(
        r"
   _   _   _   _   _   _  
  / \ / \ / \ / \ / \ / \ 
 ( m | a | g | s | D | B )
  \_/ \_/ \_/ \_/ \_/ \_/
    ");

    add_to(&db, data).await?;
    list_all(&db).await?;
    println!("----------------------------------------------------------------");

    list_year(&db, 1987).await?;
    println!("----------------------------------------------------------------");

    //add_relate(&db,"Senna".to_string());
    list_related(&db);

    Ok(())
}
