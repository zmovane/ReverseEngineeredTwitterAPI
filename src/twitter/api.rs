use reqwest::{self, header, Error};
use serde::Deserialize;
use serde_json::{self, json};
const LOGIN_URL: &str = "https://api.twitter.com/1.1/onboarding/task.json";
const LOGOUR_URL: &str = "https://api.twitter.com/1.1/account/logout.json";
const GUEST_ACTIVE_URL: &str = "https://api.twitter.com/1.1/guest/activate.json";
const OAUTH_URL: &str = "https://api.twitter.com/oauth2/token";
const BEARER_TOKEN: &str = "AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA";
const APP_CONSUMER_KEY: &str = "3nVuSoBZnx6U4vzUxf5w";
const APP_CONSUMER_SECRET: &str = "Bcs59EFbbsdF6Sl9Ng71smgStWEGwXXKSjYvPVt7qys";

#[derive(Deserialize)]
pub struct OpenAccount {
    oauth_token: String,
    oauth_token_secret: String,
}

#[derive(Deserialize)]
pub struct Subtask {
    subtask_id: String,
    open_account: OpenAccount,
}

#[derive(Deserialize)]
pub struct ApiError {
    code: String,
    message: String,
}

#[derive(Deserialize)]
pub struct Flow {
    errors: Vec<ApiError>,
    flow_token: String,
    status: String,
    subtasks: Vec<Subtask>,
}

#[derive(Deserialize)]
pub struct GuestToken {
    guest_token: String,
}

pub struct VerifyCredentials {
    errors: Vec<ApiError>,
}

pub struct Account;

impl Account {
    async fn get_flow(&self, body: serde_json::Value) -> Result<Flow, Error> {
        let guest_token = self.get_guest_token().await?;
        let mut headers = header::HeaderMap::new();
        let token = format!("Bearer {}", BEARER_TOKEN).parse().unwrap();
        headers.insert("Authorization", token);
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("User-Agent", "TwitterAndroid/99".parse().unwrap());
        headers.insert("X-Guest-Token", guest_token.parse().unwrap());
        headers.insert("X-Twitter-Auth-Type", "OAuth2Client".parse().unwrap());
        headers.insert("X-Twitter-Active-User", "yes".parse().unwrap());
        headers.insert("X-Twitter-Client-Language", "en".parse().unwrap());

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client.post(LOGIN_URL).json(&body).send().await?;
        return res.json::<Flow>().await;
    }

    async fn get_flow_token(&self, data: serde_json::Value) -> Result<String, String> {
        let res = self.get_flow(data);
        match res.await {
            Ok(info) => {
                if info.subtasks.len() > 0 {
                    let subtask_id = info.subtasks[0].subtask_id.to_owned();
                    return Err(format!("Auth error: {}", subtask_id));
                }
                return Ok(info.flow_token);
            }
            Err(e) => Err(format!("Request error: {}", e.to_string())),
        }
    }

    async fn get_guest_token(&self) -> Result<String, Error> {
        let mut headers = header::HeaderMap::new();
        let token = format!("Bearer {}", BEARER_TOKEN).parse().unwrap();
        headers.insert("Authorization", token);
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client.post(GUEST_ACTIVE_URL).send().await;
        match res {
            Ok(r) => {
                let op = r.json::<serde_json::Value>().await?;
                let guest_token = op.get("guest_token").unwrap();
                Ok(guest_token.to_string())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn login(
        &self,
        user_name: String,
        password: String,
        confirmation: String,
    ) -> Result<String, String> {
        // flow start
        let data = json!(
            {
                "flow_name": "login",
                "input_flow_data": {
                    "flow_context" : {
                        "debug_overrides": {},
                        "start_location": {
                            "location": "splash_screen"
                        }
                    }
                }
            }
        );
        let flow_token = self.get_flow_token(data).await?;

        // flow instrumentation step
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs" : {
                    "subtask_id": "LoginJsInstrumentationSubtask",
                    "js_instrumentation": {
                        "response": "{}",
                        "link": "next_link"
                    }
                }
            }
        );
        let flow_token = self.get_flow_token(data).await?;

        // flow username step
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs" : {
                    "subtask_id": "LoginEnterUserIdentifierSSO",
                    "settings_list": {
                        "setting_responses" : {
                            "key":           "user_identifier",
                            "response_data": {
                                "text_data" :{
                                    "result": user_name
                                }
                            }
                        },
                        "link": "next_link"
                    }
                }
            }
        );
        let flow_token = self.get_flow_token(data).await?;

        // flow password step
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs": {
                    "subtask_id":     "LoginEnterPassword",
                    "enter_password": {
                        "password": password,
                        "link": "next_link"
                    },
                }
            }
        );
        let flow_token = self.get_flow_token(data).await?;

        // flow duplication check
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs": {
                    "subtask_id":              "AccountDuplicationCheck",
                    "check_logged_in_account": {
                        "link": "AccountDuplicationCheck_false"
                    },
            }
        });
        let flow_token = self.get_flow_token(data).await;
        match flow_token {
            Err(e) => {
                let mut confirmation_subtask = "";
                for item in vec!["LoginAcid", "LoginTwoFactorAuthChallenge"] {
                    if e.contains(item) {
                        confirmation_subtask = item;
                        break;
                    }
                }
                if !confirmation_subtask.is_empty() {
                    if confirmation.is_empty() {
                        let msg = format!(
                            "confirmation data required for {}",
                            confirmation_subtask.to_owned()
                        );
                        return Err(msg);
                    }
                    let data = json!(
                        {
                            "flow_token": "",
                            "subtask_inputs": {
                                    "subtask_id": confirmation_subtask,
                                    "enter_text": {
                                        "text": confirmation,
                                        "link": "next_link",
                                    },
                            },
                        }
                    );
                    return self.get_flow_token(data).await;
                }
                Ok("".to_owned())
            }
            Ok(_) => return Ok("".to_owned()),
        }
    }
}
