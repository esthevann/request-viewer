use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::prisma::{
    self,
    request::{address, id, name},
};

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RequestRecord {
    id: String,
    name: String,
    address: Option<String>,
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
        }),
        Err(e) => Err(format!("Error: {}", e.to_string())),
    }
}

#[tauri::command]
pub async fn update_request_record(id: &str, address: &str) -> Result<RequestRecord, String> {
    let client = prisma::new_client().await.unwrap();
    let req = client
        .request()
        .find_unique(id::equals(id.to_owned()))
        .update(vec![address::set(Some(address.to_owned()))])
        .exec()
        .await;

    match req {
        Ok(data) => {
            if let Some(data) = data {
                Ok(RequestRecord {
                    id: data.id,
                    address: data.address,
                    name: data.name,
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
            })
            .collect()),
        Err(e) => Err(format!("Error: {}", e.to_string())),
    }
}
