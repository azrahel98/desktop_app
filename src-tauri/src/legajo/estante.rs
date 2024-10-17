use serde::{Deserialize, Serialize};
use tauri::{command, State};

use crate::{ErrorMves, MysqWrapper};

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    id: i32,
    nombre: String,
    filas: i32,
    columnas: i32,
}

#[command]
pub async fn ingresar_estante(
    state: State<'_, MysqWrapper>,
    table: Table,
) -> Result<u64, ErrorMves> {
    let result = sqlx::query!(
        r#"
        INSERT INTO Estantes (nombre, filas, columnas)
        VALUES (?, ?, ?)
        "#,
        table.nombre,
        table.filas,
        table.columnas
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(e) => Ok(e.last_insert_id()),
        Err(err) => Err(ErrorMves {
            code: 10,
            message: err.to_string(),
        }),
    }
}

#[command]
pub async fn listar_estantes(state: State<'_, MysqWrapper>) -> Result<Vec<Table>, ErrorMves> {
    let table = sqlx::query_as!(Table, "select * from Estantes")
        .fetch_all(&state.pool)
        .await;

    match table {
        Ok(e) => Ok(e),
        Err(err) => Err(ErrorMves {
            code: 11,
            message: err.to_string(),
        }),
    }
}

#[command]
pub async fn agregar_ubicacion(
    state: State<'_, MysqWrapper>,
    dni: String,
    estante: i32,
    fila: i32,
    columna: i32,
) -> Result<u64, ErrorMves> {
    let result = sqlx::query!(
        r#"
        INSERT INTO ubicaciones_legajos (trabajador_id, estante_id, fila,columna)
        VALUES (?, ?, ?,?)
        "#,
        dni,
        estante,
        fila,
        columna
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(e) => Ok(e.last_insert_id()),
        Err(err) => Err(ErrorMves {
            code: 10,
            message: err.to_string(),
        }),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ubicacion {
    dni: Option<String>,
    nombre: Option<String>,
    filas: Option<i32>,
    columnas: Option<i32>,
    ufila: Option<i32>,
    ucolumna: Option<i32>,
}

#[command]
pub async fn get_ubicacion(
    state: State<'_, MysqWrapper>,
    dni: &str,
) -> Result<Ubicacion, ErrorMves> {
    let result = sqlx::query_as!(
        Ubicacion,
        "select
  dg.dni,
  es.nombre,
  es.filas,
  es.columnas,
  u.fila ufila,
  u.columna ucolumna
from
  ubicaciones_legajos u
  inner join Datos_generales dg on u.trabajador_id = dg.dni
  inner join Estantes es on u.estante_id = es.id where dg.dni = ? ",
        dni
    )
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok(e) => Ok(e),
        Err(err) => Err(ErrorMves {
            code: 10,
            message: err.as_database_error().unwrap().to_string(),
        }),
    }
}
