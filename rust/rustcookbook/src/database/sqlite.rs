use rusqlite::{Connection, Result};
use std::collections::HashMap;

pub fn create_db() -> Result<()> {
    let conn = Connection::open("cats.db")?;
    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        [],
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        [],
    )?;
    Ok(())
}

pub fn insert_db() -> Result<()> {
    #[derive(Debug)]
    struct Cat {
        name: String,
        color: String,
    }
    let conn = Connection::open("cats.db")?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Summy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);
    for (color, cat_names) in cat_colors {
        conn.execute("
            insert into cat_colors (name) values (?1)
        ", [&color])?;
        let last_id = conn.last_insert_rowid().to_string();
        for cat_name in cat_names {
            conn.execute("
                insert into cats (name, color_id) values (?1, ?2)
            ", [&cat_name, &last_id.as_str()])?;
        }
    }

    let mut stmt = conn.prepare("
        SELECT c.name, cc.name
        FROM cats c INNER JOIN cat_colors cc
        WHERE cc.id = c.color_id
    ")?;
    let cats = stmt.query_map([], |row| {
        Ok(Cat {  
            name: row.get(0)?,
            color: row.get(1)?
        })
    })?;

    cats.filter_map(|cat| cat.ok()).for_each(|cat| {
        println!("{:?}", cat);
    });

    Ok(())
}

fn rolled_back() -> Result<()> {
    let mut conn = Connection::open("cats.db")?;
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", [])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    // tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;

    // 尝试插入相同的数据, 回滚
    // tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;

    tx.commit()
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        create_db();
        // insert_db().expect("ssssssssssssssssss");
        // rolled_back().expect("ssssssssssssssssss");;
    }
}