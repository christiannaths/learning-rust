#[path = "../resource.rs"]
mod resource;

use resource::{IOResource, Resource, ResourceEngine, ResourceTrait, ResourceURI};
use serde::{Deserialize, Serialize};

const URI_TEMPLATE: &str = "users/";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    uri: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserUriParams {
    user_id: String,
}

impl ResourceTrait<User> for User {
    fn from_resource(resource: Resource) -> User {
        let metadata = resource.metadata;
        User {
            id: resource.id,
            uri: resource.uri,
            name: metadata["name"].to_string(),
        }
    }
}

#[tauri::command]
pub fn get_user(app_handle: tauri::AppHandle, user_id: String) -> IOResource<User> {
    ResourceEngine::new::<UserUriParams>(
        app_handle,
        ResourceURI {
            template: URI_TEMPLATE.to_string(),
            params: None,
        },
    )
    .get(&user_id)
}

#[tauri::command]
pub fn create_user(app_handle: tauri::AppHandle, metadata: serde_json::Value) -> IOResource<User> {
    ResourceEngine::new::<UserUriParams>(
        app_handle,
        ResourceURI {
            template: URI_TEMPLATE.to_string(),
            params: None,
        },
    )
    .create(metadata)
}
