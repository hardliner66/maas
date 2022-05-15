use std::{collections::HashMap, sync::Arc, time::Duration};

use lazy_static::lazy_static;
use rocket::{
    delete, get,
    http::Status,
    put,
    serde::{json::Json, Serialize},
    time::Instant,
    tokio::{spawn, sync::Mutex, time::sleep},
    State,
};
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, settings::UrlObject, swagger_ui::*};
use schemars::JsonSchema;
use uuid::Uuid;

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[doc(hidden)]
struct CustomError {
    message: String,
    code: u16,
}

lazy_static! {
    static ref MAX_MUTEX_DURATION: u64 = std::option_env!("MAAS_MAX_MUTEX_DURATION")
        .unwrap_or("300")
        .parse()
        .unwrap_or(300);
}

type CustomStatus = (Status, Json<CustomError>);

type MutexList = Arc<Mutex<HashMap<String, (Uuid, u64)>>>;

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct MutexData {
    name: String,
    is_locked: bool,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct LockMutexData {
    #[serde(flatten)]
    inner: MutexData,
    uuid: String,
}

#[openapi]
#[get("/lock/<name>")]
/// Returns if a mutex with the given name is locked
///
/// Returns a struct with the `name` and the `is_locked` property of a mutex.
///
/// # Arguments
///
/// * `name` - The name of the mutex.
async fn is_locked(state: &State<MutexList>, name: &str) -> Json<MutexData> {
    let ml = state.lock().await;
    Json(MutexData {
        name: name.to_owned(),
        is_locked: ml.contains_key(name),
    })
}

async fn try_add(ml: &MutexList, name: &str, uuid: &Uuid) -> bool {
    let mut ml = ml.lock().await;
    if ml.contains_key(name) {
        false
    } else {
        ml.insert(name.into(), (*uuid, *MAX_MUTEX_DURATION));
        true
    }
}

#[openapi]
#[put("/lock/<name>?<timeout>")]
/// Tries to get ownership of a mutex
///
/// Tries to get ownership of a mutex with the specified `name`, waiting for a max amount of time specified in `timout`.
/// This method will not wait longer than the maximum amount of time a mutex can be active. This is
/// configured by the server owner.
///
/// # Arguments
///
/// * `name` - The name of the mutex.
///
/// * `timeout` - The maximum amount of seconds to wait for the mutex. Default: 60. Use 0 to return
/// instantly.
///
/// This function returns an uuid, which is proof that you hold the lock. This uuid is needed to
/// unlock the mutex.
async fn lock(
    state: &State<MutexList>,
    name: &str,
    timeout: Option<u64>,
) -> Result<Json<LockMutexData>, CustomStatus> {
    let timeout = timeout.unwrap_or(60);
    let timeout = if timeout >= *MAX_MUTEX_DURATION {
        *MAX_MUTEX_DURATION
    } else {
        timeout
    };
    let end = Instant::now() + Duration::from_secs(timeout);
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes());
    while !try_add(state, name, &uuid).await {
        if Instant::now() > end {
            let status = Status::RequestTimeout;
            return Err((
                status,
                Json(CustomError {
                    code: status.code,
                    message: format!(
                        "Request for mutex '{}' timed out after {} seconds!",
                        name, timeout
                    ),
                }),
            ));
        }
        // todo: change this to use a notification instead of sleep
        sleep(Duration::from_secs(1)).await;
    }
    Ok(Json(LockMutexData {
        uuid: uuid.to_string(),
        inner: MutexData {
            name: name.to_owned(),
            is_locked: true,
        },
    }))
}

#[openapi]
#[delete("/lock/<name>?<uuid>")]
/// Releases ownership of a mutex
///
/// Releases ownership of a mutex with the given `name`, if it is currently owned and the `uuid`
/// matches.
///
/// # Arguments
///
/// * `name` - The name of the mutex.
///
/// * `uuid` - The maximum amount of seconds to wait for the mutex. Default: 60. Use 0 to return
/// instantly.
///
/// returns a struct with the `name` and the `is_locked` property of a mutex.
async fn unlock(
    state: &State<MutexList>,
    name: &str,
    uuid: &str,
) -> Result<Json<MutexData>, CustomStatus> {
    let uuid = Uuid::try_parse(uuid);
    match uuid {
        Ok(uuid) => {
            let mut ml = state.lock().await;
            if let Some((wanted, _)) = ml.get(name) {
                if &uuid == wanted {
                    ml.remove(name);
                    Ok(Json(MutexData {
                        name: name.to_owned(),
                        is_locked: false,
                    }))
                } else {
                    let status = Status::Forbidden;
                    Err((
                        status,
                        Json(CustomError {
                            code: status.code,
                            message: format!("Uuid not valid for mutex: {}", name),
                        }),
                    ))
                }
            } else {
                let status = Status::NotFound;
                Err((
                    status,
                    Json(CustomError {
                        code: status.code,
                        message: format!("Mutex not found: {}", name),
                    }),
                ))
            }
        }
        Err(err) => {
            let status = Status::BadRequest;
            Err((
                status,
                Json(CustomError {
                    code: status.code,
                    message: format!("Could not parse uuid: {}", err),
                }),
            ))
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let ml = MutexList::default();
    let ml2 = ml.clone();
    spawn(async move {
        let dur = Duration::from_secs(60 * 60);
        loop {
            sleep(dur).await;
            let mut ml = ml2.lock().await;
            for (_, val) in ml.values_mut() {
                *val -= 1;
            }
            ml.retain(|_, (_, v)| *v != 0);
        }
    });

    let _rocket = rocket::build()
        .manage(ml)
        .mount("/mutex", openapi_get_routes![lock, unlock, is_locked])
        .mount(
            "/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "mutex/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "/mutex/openapi.json")],
                    ..Default::default()
                },
                ui: UiConfig {
                    theme: Theme::Dark,
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .launch()
        .await?;

    Ok(())
}
