mod employ;
pub mod keys;
mod notion;

use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;
use std::time::Duration;
use tokio::runtime::Runtime;

use employ::employ::{actualizar_trabajador, buscar_trabajadores, buscar_x_dni, vinculos};
use employ::legajo::{añadir_prestamo, buscar_prestamos, editar_prestamo};
use employ::login::login;
use notion::notion::fetch_notion;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMves {
    pub code: i32,
    pub message: String,
}

pub struct MysqWrapper {
    pub pool: MySqlPool,
}

async fn establecer_conexion() -> Result<MySqlPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("No se encontró la variable de entorno");
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&db_url)
        .await?;

    println!("Conexión establecida: {:?}", pool);
    Ok(pool)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv()?;

    let runtime = Runtime::new()?;
    let pool = runtime.block_on(establecer_conexion())?;

    tauri::Builder::default()
        .manage(MysqWrapper { pool })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            login,
            buscar_trabajadores,
            fetch_notion,
            buscar_x_dni,
            vinculos,
            actualizar_trabajador,
            añadir_prestamo,
            editar_prestamo,
            buscar_prestamos
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
