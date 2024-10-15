use serde::{Deserialize, Serialize};
use tauri::{command, State};

use crate::{ErrorMves, MysqWrapper};

#[derive(Serialize, Deserialize, Debug)]
pub struct Prestamo {
    id: Option<i32>, // Solo para editar, ya que en insert es autoincremental
    ubicacion_origen: Option<String>,
    prestamo: String,
    usuario: i32,
    create_at: Option<String>, // Se puede manejar con el valor por defecto en la base de datos
    devuelto: Option<String>,
    trabajador: Option<String>,
    fechaprestamo: String,
}

#[command]
pub async fn añadir_prestamo(
    state: State<'_, MysqWrapper>,
    body: Prestamo,
) -> Result<(), ErrorMves> {
    let result = sqlx::query!(
        "INSERT INTO Legajo (ubicacion_origen, prestamo, usuario, create_at, devuelto,trabajador,fechaprestamo)
        VALUES (?, ?, ?, current_timestamp(), ?,?,?)",
        body.ubicacion_origen,
        body.prestamo,
        body.usuario,
        body.devuelto,
        body.trabajador,
        body.fechaprestamo
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_e) => Ok(()),
        Err(err) => Err(ErrorMves {
            code: 7,
            message: err.as_database_error().unwrap().to_string(),
        }),
    }
}

#[command]
pub async fn editar_prestamo(
    state: State<'_, MysqWrapper>,
    body: Prestamo,
) -> Result<Prestamo, ErrorMves> {
    let result = sqlx::query!(
        "UPDATE Legajo
        SET ubicacion_origen = IFNULL(?, ubicacion_origen),
            prestamo = IFNULL(?, prestamo),
            usuario = IFNULL(?, usuario),
            devuelto = IFNULL(?, devuelto)
        WHERE id = ?",
        body.ubicacion_origen,
        body.prestamo,
        body.usuario,
        body.devuelto,
        body.id
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_e) => Ok(body),
        Err(err) => Err(ErrorMves {
            code: 9,
            message: err.as_database_error().unwrap().to_string(),
        }),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resumen {
    id: Option<i32>,
    ubicacion_origen: Option<String>,
    prestamo: String,
    usuario: String,
    fechaprestamo: Option<String>,
    create_at: Option<String>,
    devuelto: Option<String>,
}
#[command]
pub async fn buscar_prestamos(
    state: State<'_, MysqWrapper>,
    dni: &str,
) -> Result<Vec<Resumen>, ErrorMves> {
    let result = sqlx::query_as!(
        Resumen,
        "select
           l.id,
            l.ubicacion_origen,
            l.prestamo,
            u.nickname usuario,
            cast(l.fechaprestamo as char) fechaprestamo,
            cast(l.create_at as char) create_at,
            cast(l.devuelto as char) devuelto
            from
            Legajo l
            inner join usuario u on l.usuario = u.id
            where l.trabajador = ? ",
        dni
    )
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(e) => Ok(e),
        Err(err) => Err(ErrorMves {
            code: 10,
            message: err.as_database_error().unwrap().to_string(),
        }),
    }
}
