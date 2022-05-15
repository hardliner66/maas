use std::{collections::HashMap, sync::Arc, time::Duration};

use rocket::{
    delete, get,
    http::Status,
    put,
    serde::{json::Json, Serialize},
    time::Instant,
    tokio::{spawn, sync::Mutex, time::sleep},
    State,
};
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, settings::UrlObject};
use schemars::JsonSchema;
use uuid::Uuid;

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct CustomError {
    message: String,
    code: u16,
}

const MAX_MUTEX_DURATION: u64 = 120;

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
        ml.insert(name.into(), (uuid.clone(), MAX_MUTEX_DURATION));
        true
    }
}

#[openapi]
#[put("/lock/<name>?<timeout>")]
async fn lock(
    state: &State<MutexList>,
    name: &str,
    timeout: Option<u64>,
) -> Result<Json<LockMutexData>, CustomStatus> {
    let timeout = timeout.unwrap_or(60);
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
                *val = *val - 1;
            }
            ml.retain(|_, (_, v)| *v != 0);
        }
    });

    let _rocket = rocket::build()
        .manage(ml)
        .mount("/", openapi_get_routes![lock, unlock, is_locked])
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
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
