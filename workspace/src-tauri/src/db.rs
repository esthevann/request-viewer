use crate::{prisma::{
    self,
    request::{address, id, name},
}, request::MethodType};
use prisma_client_rust::chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use ts_rs::TS;


#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RequestRecord {
    id: String,
    name: String,
    address: Option<String>,
    created_at: DateTime<FixedOffset>,
    method: MethodType
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateRequestArgs {
    name: String,
    address: Option<String>,
}

#[tauri::command]
pub async fn create_request_record(args: CreateRequestArgs) -> Result<RequestRecord, String> {
    if args.name.is_empty() {
        return Err("Name can't be empty".to_owned());
    }
    let client = prisma::new_client().await.unwrap();
    let req = client
        .request()
        .create(
            name::set(args.name.to_owned()),
            vec![address::set(args.address)],
        )
        .exec()
        .await;

    match req {
        Ok(d) => Ok(RequestRecord {
            name: d.name,
            id: d.id,
            address: d.address,
            created_at: d.created_at,
            method: d.method.into()
        }),
        Err(e) => Err(format!("Error: {}", e.to_string())),
    }
}

#[tauri::command]
pub async fn update_request_record(id: &str, address: Option<String>, name: &str) -> Result<RequestRecord, String> {
    let client = prisma::new_client().await.unwrap();
    let req = client
        .request()
        .find_unique(id::equals(id.to_owned()))
        .update(vec![address::set(address), name::set(name.to_owned())])
        .exec()
        .await;

    match req {
        Ok(data) => {
            if let Some(data) = data {
                Ok(RequestRecord {
                    id: data.id,
                    address: data.address,
                    name: data.name,
                    created_at: data.created_at,
                    method: data.method.into()
                })
            } else {
                Err("No op".to_owned())
            }
        }
        Err(e) => Err(format!("Error: {}", e.to_string())),
    }
}

#[tauri::command]
pub async fn list_all_requests() -> Result<Vec<RequestRecord>, String> {
    let client = prisma::new_client().await.unwrap();
    let reqs = client.request().find_many(vec![]).exec().await;

    match reqs {
        Ok(d) => Ok(d
            .into_iter()
            .map(|x| RequestRecord {
                address: x.address,
                id: x.id,
                name: x.name,
                created_at: x.created_at,
                method: x.method.into()
            })
            .collect()),
        Err(e) => Err(format!("Error: {}", e.to_string())),
    }
}

#[tauri::command]
pub async fn get_record_by_id(id: String) -> Result<RequestRecord, String> {
    let client = prisma::new_client().await.unwrap();
    let rec = client.request().find_unique(id::equals(id)).exec().await;

    match rec {
        Ok(d) => {
            if let Some(data) = d {
                Ok(RequestRecord {
                    id: data.id,
                    address: data.address,
                    name: data.name,
                    created_at: data.created_at,
                    method: data.method.into()
                })
            } else {
                Err("Request not found".to_string())
            }
        }
        Err(e) => Err(format!("Error: {}", e.to_string())),
    }
}
