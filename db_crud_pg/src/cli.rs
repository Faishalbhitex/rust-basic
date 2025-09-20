use crate::db::User as ModelUser;
use std::io::{self, Write};
use sqlx::PgPool;
use rust_decimal::Decimal;

pub async fn cli_crud_pg(pool: &PgPool) -> Result<(), sqlx::Error> {
    loop {
        println!("\nChoose CRUD mode operations:");
        println!("1. Insert user (name, age, height)");
        println!("2. Select user by id (id)");
        println!("3. Update user age (id)");
        println!("4. Delete user (id)");
        println!("5. Get all users data");
        println!("6. Exit\n");

        println!("choice: ");
        io::stdout().flush()?;
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That option does not exist. Try again with a valid number");
                continue;
            }
        };

        match choice {
            1 => {
                insert_operations(pool).await?;
            }
            2 => {
                select_user_by_id_operations(pool).await?;
            }
            3 => {
                update_user_age_by_id_operations(pool).await?;
            }
            4 => {
                delete_user_by_id_operations(pool).await?;
            }
            5 => {
                get_all_users_operations(pool).await?;
            }
            6 => {
                println!("Exit the program.");
                break Ok(());
            }
            _ => println!("Invalid choice"),
        }
    }
}


async fn insert_operations(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!("\nMode INSERT(name, age, height):");
    let mut name = String::new();
    let mut age_str = String::new();
    let mut height_str = String::new();
    println!(">Enter name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut name).expect("Failed to read name");
    println!(">Enter age: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut age_str).expect("Failed to read age");
    println!(">Enter height: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut height_str).expect("Failed to read height");

    let name = name.trim().to_string();
    let age = age_str.trim().parse::<i32>();
    let height = Decimal::from_str_radix(height_str.trim(), 10);
    match (age, height) {
        (Ok(age), Ok(height)) => {
            let final_height = height.round_dp(2);
            println!("\nValid input height. Inserting...");
            let _ = ModelUser::insert_user(&pool, &name, age, final_height).await?;
            ModelUser::show_users(&pool).await?;
            Ok(())
        }
        (Err(e), _) => {
            eprintln!("Error: Failed to convert age: {}", e);
            Ok(())
        }
        (_,Err(e)) => {
            eprintln!("Error: Failed to convert height. Make sure format (175.50). {}", e);
            Ok(())
        }
    }

}

async fn select_user_by_id_operations(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!("\nMode Select(id):");
    let mut id_str = String::new();
    println!(">Enter user id: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut id_str).expect("Failed read id");

    match id_str.trim().parse::<i32>() {
        Ok(id) => {
            let user = ModelUser::get_user_by_id(pool, id).await?;
            println!("User({:?}): {:?}", &id, &user);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: failed to convert id: {}", e);
            Ok(())
        }
    }
}

async fn update_user_age_by_id_operations(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!("\nMode Update(id):");
    let mut id_str = String::new();
    let mut age_str = String::new();
    println!(">Enter user id: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut id_str).expect("Failed read id");
    io::stdout().flush()?;
    println!(">Enter user new age: ");
    io::stdin().read_line(&mut age_str).expect("Failed read new age");

    match(
        id_str.trim().parse::<i32>(),
        age_str.trim().parse::<i32>(),
    ) {
        (Ok(id),Ok(age)) => {
            // Data before update 
            let user = ModelUser::get_user_by_id(pool, id).await?;
            println!("\n-Before update age: {:?}\n", &user);
            // Data after update 
            let update_user = ModelUser::update_user_age(pool, age, id).await?;
            println!("+Afer update age: {:?}", &update_user);
            Ok(())
        }
        (Err(e), _) => {
            eprintln!("Error: Failed to convert id: {}", e);
            Ok(())
        }
        (_, Err(e)) => {
            eprintln!("Error: Failed to convert age: {}", e);
            Ok(())
        }
    }

}

async fn delete_user_by_id_operations(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!("\nMode Delete(id):");
    let mut id_str = String::new();
    println!(">Enter user id: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut id_str).expect("Failed read id");

    match id_str.trim().parse::<i32>() {
        Ok(id) => {
            let temp_user = ModelUser::get_user_by_id(pool, id).await?;
            println!("Delete User from DB: {:?}", &temp_user);
            ModelUser::delete_user(pool, id).await?;
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: Failed convert id. {}", e);
            Ok(())
        }
    }

}

async fn get_all_users_operations(pool: &PgPool) -> Result<(), sqlx::Error> {
    let separator = "=".repeat(10);
    println!("\n#{separator} All Database Users {separator}#\n");
    ModelUser::show_users(pool).await?;
    Ok(())
}
