// use twilio::{twiml::{self, Twiml}, OutboundMessage};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OTPData {
    pub phone_number: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOTPData {
    pub user: OTPData,
    pub code: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OTPResponse {
    pub sid: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OTPVerifyResponse {
    pub status: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct APIResponse {
    pub status: u16,
    pub message: String,
    pub data: String,
}

use std::collections::HashMap;

use reqwest::{header, Client};

pub struct TwilioService {}

impl TwilioService {

    pub async fn send_otp(phone_number: &String) -> Result<OTPResponse, &'static str> {
        let account_sid = std::env::var("TWILIO_ACCOUNT_SID").expect("TWILIO_ACCOUNT_SID must be set");
        let auth_token = std::env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN must be set");
        let service_id = std::env::var("TWILIO_SERVICE_ID").expect("TWILIO_SERVICE_ID must be set");

        let url = format!(
            "https://verify.twilio.com/v2/Services/{serv_id}/Verifications",
            serv_id = service_id
        );

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut form_body: HashMap<&str, String> = HashMap::new();
        form_body.insert("To", phone_number.to_string());
        form_body.insert("Channel", "sms".to_string());

        let client = Client::new();
        let res = client
            .post(url)
            .basic_auth(account_sid, Some(auth_token))
            .headers(headers)
            .form(&form_body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let result = response.json::<OTPResponse>().await;
                match result {
                    Ok(data) => Ok(data),
                    Err(_) => Err("Error sending OTP"),
                }
            }
            Err(_) => Err("Error sending OTP"),
        }
    }


    pub async fn verify_otp(phone_number: &String, code: &String) -> Result<(), &'static str> {
        let account_sid = std::env::var("TWILIO_ACCOUNT_SID").expect("TWILIO_ACCOUNT_SID must be set");
        let auth_token = std::env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN must be set");
        let service_id = std::env::var("TWILIO_SERVICE_ID").expect("TWILIO_SERVICE_ID must be set");

        let url = format!(
            "https://verify.twilio.com/v2/Services/{serv_id}/VerificationCheck",
            serv_id = service_id,
        );

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut form_body: HashMap<&str, &String> = HashMap::new();
        form_body.insert("To", phone_number);
        form_body.insert("Code", code);

        let client = Client::new();
        let res = client
            .post(url)
            .basic_auth(account_sid, Some(auth_token))
            .headers(headers)
            .form(&form_body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let data = response.json::<OTPVerifyResponse>().await;
                match data {
                    Ok(result) => {
                        if result.status == "approved" {
                            Ok(())
                        } else {
                            Err("Error verifying OTP")
                        }
                    }
                    Err(_) => Err("Error verifying OTP"),
                }
            }
            Err(_) => Err("Error verifying OTP"),
        }
    }
}
