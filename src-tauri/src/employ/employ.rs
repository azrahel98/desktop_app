use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tauri::{command, State};

use crate::{keys::DBKEY, ErrorMves, MysqWrapper};

#[derive(Deserialize, Serialize, Debug)]
pub struct ResultadoBuscar {
    nombres: Option<String>,
    dni: Option<String>,
    imagen: Option<String>,
    activo: Option<String>,
    sexo: Option<String>,
}

#[command]
pub async fn buscar_trabajadores(
    state: State<'_, MysqWrapper>,
    nombre: &str,
) -> Result<Vec<ResultadoBuscar>, ErrorMves> {
    let shearch = format!("%{}%", nombre);

    let rows = sqlx::query_as!(
        ResultadoBuscar,
        "select
        concat(dg.nombres, ' ', dg.apaterno, ' ', dg.amaterno) nombres,
        v.dni,
        imagen,
        MAX(v.activo) AS activo,
        dg.sexo
        from
        Datos_generales dg
        inner join Vinculo v on dg.dni = v.dni
        inner join Puesto p on v.puesto = p.id_puesto
        inner join Cargo cr on p.cargo = cr.id
        WHERE
        concat(dg.nombres, ' ', dg.apaterno, ' ', dg.amaterno) like ?
        GROUP by
        v.dni",
        shearch
    )
    .fetch_all(&state.pool)
    .await;

    match rows {
        Ok(r) => Ok(r),
        Err(_e) => Err(ErrorMves {
            code: 4,
            message: "Error durante la consulta".to_string(),
        }),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InforTrabajador {
    nombres: Option<String>,
    dni: String,
    direccion: Option<String>,
    telefono: Option<String>,
    correo: Option<String>,
    nacimiento: Option<NaiveDate>,
    ruc: Option<String>,
    pension: Option<String>,
    cussp: Option<Vec<u8>>,
    imagen: Option<String>,
}

#[command]
pub async fn buscar_x_dni(
    state: State<'_, MysqWrapper>,
    dni: &str,
) -> Result<InforTrabajador, ErrorMves> {
    let rows = sqlx::query_as!(
        InforTrabajador,
        "select
            concat(apaterno, ' ', amaterno, ' ', nombres) nombres,
            cast(aes_decrypt(direccion, ?) as char) direccion,
            dni,
            cast(aes_decrypt(telf, ?) as char) telefono,
            cast(aes_decrypt(email, ?) as char) correo,
            fecha_nacimiento nacimiento,
            ruc,
            pension,
            cussp,
            imagen
            from
            Datos_generales where dni = ?",
        DBKEY.to_string(),
        DBKEY.to_string(),
        DBKEY.to_string(),
        dni
    )
    .fetch_one(&state.pool)
    .await;

    match rows {
        Ok(us) => Ok(us),
        Err(err) => Err(ErrorMves {
            code: 5,
            message: err.as_database_error().unwrap().to_string(),
        }),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VinculoInfo {
    id: i32,
    ingreso: NaiveDate,
    renuncia: Option<NaiveDate>,
    contrato: Option<String>,
    resolucion: Option<String>,
    convocatoria: Option<i32>,
    cas: Option<String>,
    activo: String,
    sueldo: f64,
    cargo: Option<String>,
    area: Option<String>,
    regimen: Option<String>,
}

#[command]
pub async fn vinculos(
    state: State<'_, MysqWrapper>,
    dni: &str,
) -> Result<Vec<VinculoInfo>, ErrorMves> {
    let rows = sqlx::query_as!(
        VinculoInfo,
        "select
        v.id,
        v.fecha_ingreso ingreso,
        v.fecha_salida renuncia,
        cast(v.numero_contrato as char) contrato,
        v.numero_resolucion resolucion,
        v.convocatoria convocatoria,
        v.procesocas cas,
        v.activo,
        p.sueldo,
        cr.nombre cargo,
        ar.nombre area,
        rg.nombre regimen
      from
        Vinculo v
        inner join Puesto p on v.puesto = p.id_puesto
        inner join Cargo cr on p.cargo = cr.id
        inner join Area ar on p.area = ar.id
        inner join Regimen rg on p.regimen = rg.id_regimen
      where
        v.dni = ?
        ORDER BY v.activo",
        dni
    )
    .fetch_all(&state.pool)
    .await;

    match rows {
        Ok(us) => Ok(us),
        Err(error) => {
            return Err(ErrorMves {
                code: 6,
                message: error.as_database_error().unwrap().to_string(),
            });
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActualizarTrabajador {
    direccion: Option<String>,
    telefono: Option<String>,
    correo: Option<String>,
    nacimiento: Option<String>,
    ruc: Option<String>,
    imagen: Option<String>,
    dni: String,
}
#[command]
pub async fn actualizar_trabajador(
    state: State<'_, MysqWrapper>,
    body: ActualizarTrabajador,
) -> Result<ActualizarTrabajador, ErrorMves> {
    let result = sqlx::query!(
        "UPDATE Datos_generales
    SET
        direccion = IFNULL(aes_encrypt(?, ?), direccion),
        telf = IFNULL(aes_encrypt(?, ?), telf),
        email = IFNULL(aes_encrypt(?, ?), email),
        ruc = IFNULL(?, ruc),
        imagen = IFNULL(?, imagen),
        fecha_nacimiento = IFNULL(?, fecha_nacimiento)
    WHERE dni = ?
        ",
        body.direccion,
        DBKEY,
        body.telefono,
        DBKEY,
        body.correo,
        DBKEY,
        body.ruc,
        body.imagen,
        body.nacimiento,
        body.dni
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_e) => Ok(body),
        Err(err) => Err(ErrorMves {
            code: 7,
            message: err.as_database_error().unwrap().to_string(),
        }),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lista {
    dni: Option<String>,
    nombres: Option<String>,
    dia: Option<i32>,
    edad: Option<i32>,
}

#[command]
pub async fn cumpleaños_lista(
    state: State<'_, MysqWrapper>,
    mes: i32,
) -> Result<Vec<Lista>, ErrorMves> {
    let rows = sqlx::query_as!(
        Lista,
        "select
  dg.dni,
  day(dg.fecha_nacimiento) dia,
  concat(dg.apaterno,' ',dg.amaterno,' ',dg.nombres) nombres,
  2024 - year(dg.fecha_nacimiento) edad
from
  Datos_generales dg
  inner join Vinculo v on dg.dni = v.dni
where
  v.activo = 'Y'
  and month(dg.fecha_nacimiento) = ?",
        mes
    )
    .fetch_all(&state.pool)
    .await;

    match rows {
        Ok(us) => Ok(us),
        Err(error) => {
            return Err(ErrorMves {
                code: 6,
                message: error.as_database_error().unwrap().to_string(),
            });
        }
    }
}
