use surrealdb::{engine::local::Db, Surreal};
use crate::Colorize;
use crate::{Magazine, Record};

pub async fn add_to(db: &Surreal<Db>, data: Vec<Magazine>) -> surrealdb::Result<()> {
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

pub async fn list_all(db: &Surreal<Db>) -> surrealdb::Result<()> {
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

pub async fn list_year(db: &Surreal<Db>, year: u32) -> surrealdb::Result<()> {
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

pub async fn add_relate(db: &Surreal<Db>, topic: String) -> surrealdb::Result<()> {
    let _relate = db
        .query("RELATE product->featured->($topic)")
        .bind(("topic", topic))
        .await?;
    Ok(())
}

pub async fn list_related(db: &Surreal<Db>) -> surrealdb::Result<()> {
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
