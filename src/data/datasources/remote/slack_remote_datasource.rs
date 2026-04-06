use std::{any::Any, collections::HashMap};

use reqwest::Url;
use tiny_http::{Response, Server};

use crate::{common::Injectable, data::datasources::remote::RemoteDatasource};

pub struct SlackRemoteDatasource {}

impl Injectable for SlackRemoteDatasource {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RemoteDatasource for SlackRemoteDatasource {
    fn base_url() -> String {
        "https://slack.com".to_string()
    }
}

impl SlackRemoteDatasource {
    pub fn oauth_authorize(&self, client_id: &String, redirect_uri: &String) -> Option<String> {
        let path = "/oauth/v2/authorize";
        let scope: Vec<&str> = vec![];
        let user_scope: Vec<&str> = vec![
            "users:read",
            "usergroups:read",
            "channels:read",
            "channels:history",
            "groups:read",
            "groups:history",
            "mpim:read",
            "mpim:history",
            "im:read",
            "im:history",
            "chat:write",
        ];

        let mut auth_url = Url::parse(Self::base_url().as_str()).ok()?;
        auth_url = auth_url.join(path).ok()?;

        let mut params: HashMap<String, String> = HashMap::new();

        params.insert("scope".to_string(), scope.join(","));
        params.insert("user_scope".to_string(), user_scope.join(","));
        params.insert("redirect_uri".to_string(), redirect_uri.to_string());
        params.insert("client_id".to_string(), client_id.clone());

        for (key, value) in params {
            auth_url
                .query_pairs_mut()
                .append_pair(key.as_str(), value.as_str());
        }

        opener::open(auth_url.to_string()).ok()?;

        let server = Server::http(redirect_uri.split("://").last().unwrap()).unwrap();

        for request in server.incoming_requests() {
            let base_url = Url::parse(redirect_uri.as_str()).unwrap();
            let full_url = base_url.join(request.url()).unwrap();

            for (key, value) in full_url.query_pairs() {
                if key == "code" {
                    request.respond(Response::from_string("Success").with_status_code(200)).ok()?;
                    return Some(value.to_string());
                }
            }

            request
                .respond(Response::from_string("No code on query params").with_status_code(400))
                .ok()?;
        }

        None
    }
}

