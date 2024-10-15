use crate::{ErrorMves, MysqWrapper};
use chrono::{Duration, Local, TimeZone, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use tauri::{command, State};

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    id: i64,
    nombre: String,
    pass: Option<String>,
    nivel: i64,
    nickname: String,
}

#[command]
pub async fn login(
    state: State<'_, MysqWrapper>,
    username: &str,
    password: String,
) -> Result<String, ErrorMves> {
    let rows = sqlx::query_as!(
        Record,
        "select id,nombre,cast(aes_decrypt(pass,'olafmves') as char) pass,nivel,nickname from usuario where nickname = ?",username
    )
    .fetch_optional(&state.pool)
    .await;

    match rows {
        Ok(Some(record)) => {
            if record.pass == Some(password) {
                Ok(create_token(record.id, record.nivel, record.nombre)
                    .unwrap()
                    .to_string())
            } else {
                Err(ErrorMves {
                    code: 2,
                    message: "La contraseña es incorrecta".to_string(),
                })
            }
        }
        Ok(None) => Err(ErrorMves {
            code: 1,
            message: "El usuario no existe".to_string(),
        }),
        Err(e) => Err(ErrorMves {
            code: 3,
            message: format!("Error al consultar el usuario: {}", e),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: i64,
    exp: i64,
    lvl: i64,
    nombre: String,
}
pub fn create_token(id: i64, lvl: i64, nombre: String) -> Option<String> {
    let mut ahora = Utc::now().naive_utc();

    ahora = ahora + Duration::days(1);

    let claim = Claims {
        id,
        exp: Local.from_local_datetime(&ahora).unwrap().timestamp(),
        lvl,
        nombre,
    };

    let head = Header {
        alg: jsonwebtoken::Algorithm::HS512,
        kid: Some("munives".to_owned()),
        ..Default::default()
    };

    let token = encode(
        &head,
        &claim,
        &EncodingKey::from_secret("olafmves".as_ref()),
    );
    match token {
        Ok(e) => Some(e),
        Err(e) => {
            print!("{:?}", e);
            None
        }
    }
}
