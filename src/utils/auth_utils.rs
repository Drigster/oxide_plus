use fcm_receiver_rs::client::{AndroidApp, FcmClient};
use serde::{Deserialize, Serialize};

pub fn create_fcm_client() -> Result<FcmClient, Box<dyn std::error::Error>> {
    const API_KEY: &'static str = "AIzaSyB5y2y-Tzqb4-I4Qnlsh_9naYv_TD8pCvY";
    const PROJECT_ID: &'static str = "rust-companion-app";
    const GCM_SENDER_ID: &'static str = "976529667804";
    const GMS_APP_ID: &'static str = "1:976529667804:android:d6f1ddeb4403b338fea619";
    const ANDROID_PACKAGE_NAME: &'static str = "com.facepunch.rust.companion";
    const ANDROID_PACKAGE_CERT: &'static str = "E28D05345FB78A7A1A63D70F4A302DBF426CA5AD";
    let mut client = FcmClient::new(
        API_KEY.to_string(),
        GMS_APP_ID.to_string(),
        PROJECT_ID.to_string(),
    )?;

    client.android_app = Some(AndroidApp {
        gcm_sender_id: GCM_SENDER_ID.to_string(),
        android_package: ANDROID_PACKAGE_NAME.to_string(),
        android_package_cert: ANDROID_PACKAGE_CERT.to_string(),
    });

    return Ok(client);
}

#[derive(Serialize)]
struct ExpoPushTokenRequestBody {
    r#type: String,
    #[serde(rename = "deviceId")]
    device_id: String,
    development: bool,
    #[serde(rename = "appId")]
    app_id: String,
    #[serde(rename = "deviceToken")]
    device_token: String,
    #[serde(rename = "projectId")]
    project_id: String,
}

#[derive(Deserialize)]
struct ExpoPushTokenResponse {
    data: ExpoPushTokenData,
}

#[derive(Deserialize)]
struct ExpoPushTokenData {
    #[serde(rename = "expoPushToken")]
    expo_push_token: String,
}

pub async fn get_expo_push_token(fcm_token: String) -> Result<String, Box<dyn std::error::Error>> {
    let response = blocking::unblock(move || {
        ureq::post("https://exp.host/--/api/v2/push/getExpoPushToken")
            .send_json(ExpoPushTokenRequestBody {
                r#type: "fcm".to_string(),
                device_id: uuid::Uuid::new_v4().to_string(),
                development: false,
                app_id: "com.facepunch.rust.companion".to_string(),
                device_token: fcm_token,
                project_id: "49451aca-a822-41e6-ad59-955718d0ff9c".to_string(),
            })?
            .body_mut()
            .read_json::<ExpoPushTokenResponse>()
    })
    .await?;

    Ok(response.data.expo_push_token)
}

#[derive(Serialize)]
struct RegisterWithRustPlusRequestBody {
    #[serde(rename = "AuthToken")]
    auth_token: String,
    #[serde(rename = "DeviceId")]
    device_id: String,
    #[serde(rename = "PushKind")]
    push_kind: i32,
    #[serde(rename = "PushToken")]
    push_token: String,
}

#[derive(Deserialize)]
struct RegisterWithRustPlusResponse {
    token: String,
}

pub async fn register_with_rust_plus(
    steam_auth_token: String,
    expo_push_token: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = blocking::unblock(move || {
        ureq::post("https://companion-rust.facepunch.com:443/api/push/register")
            .send_json(RegisterWithRustPlusRequestBody {
                auth_token: steam_auth_token,
                device_id: "OxidePlus".to_string(),
                push_kind: 3,
                push_token: expo_push_token,
            })?
            .body_mut()
            .read_json::<RegisterWithRustPlusResponse>()
    })
    .await?;

    Ok(response.token)
}
