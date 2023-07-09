use log::{debug, info};
use reqwest::{self, Client, Error};
use serde::Deserialize;
use serde_json::{self, json};
const LOGIN_URL: &str = "https://api.twitter.com/1.1/onboarding/task.json";
const LOGOUR_URL: &str = "https://api.twitter.com/1.1/account/logout.json";
const GUEST_ACTIVE_URL: &str = "https://api.twitter.com/1.1/guest/activate.json";
const VERIFY_CREDENTIALS_URL: &str = "https://api.twitter.com/1.1/account/verify_credentials.json";
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
    open_account: Option<OpenAccount>,
}

#[derive(Deserialize)]
pub struct ApiError {
    code: i64,
    message: String,
}

#[derive(Deserialize)]
pub struct Flow {
    errors: Option<Vec<ApiError>>,
    flow_token: String,
    status: String,
    subtasks: Vec<Subtask>,
    js_instrumentation: Option<Insrumentation>,
}

#[derive(Deserialize)]
pub struct Insrumentation {
    url: String,
    timeout_ms: i64,
    next_link: Link,
}

#[derive(Deserialize)]
pub struct Link {
    link_type: String,
    link_id: String,
}

#[derive(Deserialize)]
pub struct GuestToken {
    guest_token: String,
}

#[derive(Deserialize)]
pub struct VerifyCredentials {
    errors: Vec<ApiError>,
}

pub struct API {
    client: Client,
    guest_token: String,
}

impl API {
    pub fn new() -> API {
        return API {
            client: reqwest::ClientBuilder::new().build().unwrap(),
            guest_token: "".to_string(),
        };
    }
    async fn get_flow(&mut self, body: serde_json::Value) -> Result<Flow, Error> {
        if self.guest_token.is_empty() {
            self.get_guest_token().await?
        }
        let res = self
            .client
            .post(LOGIN_URL)
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("Content-Type", "application/json")
            .header("User-Agent", "TwitterAndroid/99")
            .header("X-Guest-Token", self.guest_token.replace("\"", ""))
            .header("X-Twitter-Auth-Type", "OAuth2Client")
            .header("X-Twitter-Active-User", "yes")
            .header("X-Twitter-Client-Language", "en")
            .json(&body)
            .send()
            .await?;

        let text = res.text().await?;
        let result: Flow = serde_json::from_str(&text).unwrap();
        return Ok(result);
    }

    async fn get_flow_token(&mut self, data: serde_json::Value) -> Result<String, String> {
        let res = self.get_flow(data);
        match res.await {
            Ok(info) => {
                println!("flow token: {}", info.flow_token);
                if info.subtasks.len() > 0 {
                    let subtask_id = info.subtasks[0].subtask_id.to_owned();
                    if subtask_id.ne("LoginJsInstrumentationSubtask") {
                        return Err(format!("Auth error: {}", subtask_id));
                    }
                }
                return Ok(info.flow_token);
            }
            Err(e) => Err(format!("Request error: {}", e.to_string())),
        }
    }

    async fn get_guest_token(&mut self) -> Result<(), Error> {
        let token = format!("Bearer {}", BEARER_TOKEN);
        let res = self
            .client
            .post(GUEST_ACTIVE_URL)
            .header("Authorization", token)
            .send()
            .await;
        match res {
            Ok(r) => {
                let op = r.json::<serde_json::Value>().await?;
                let guest_token = op.get("guest_token").unwrap();
                self.guest_token = guest_token.to_string();
                return Ok(());
            }
            Err(e) => Err(e),
        }
    }

    pub async fn login(
        &mut self,
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
        println!("flow start: {}", flow_token.to_owned());

        // flow instrumentation step
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs" : [{
                    "subtask_id": "LoginJsInstrumentationSubtask",
                    "js_instrumentation":{
                        "response": "{}",
                        "link": "next_link"
                    }
                }],
            }
        );
        let flow_token = self.get_flow_token(data).await?;
        println!("flow instrumentation step: {}", flow_token.to_owned());

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
        println!("flow username step: {}", flow_token.to_owned());

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
        println!("flow password step: {}", flow_token.to_owned());

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

    pub async fn is_logged_in(&self) -> bool {
        let res = self
            .client
            .get(VERIFY_CREDENTIALS_URL)
            .send()
            .await
            .unwrap()
            .json::<VerifyCredentials>()
            .await
            .unwrap();
        res.errors.len() == 0
    }
}
