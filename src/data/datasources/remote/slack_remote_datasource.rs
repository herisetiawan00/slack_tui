use std::{any::Any, collections::HashMap, net::SocketAddr, time::Duration};

use axum::{
    Router,
    extract::{Query, State},
    response::Html,
    routing::get,
};
use axum_server::{Handle, tls_rustls::RustlsConfig};
use reqwest::{Client, Response, Url};
use tokio::{runtime::Runtime, sync::mpsc};

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
    pub async fn wait_for_oauth_code(port: u16) -> Option<String> {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let config = RustlsConfig::from_pem(
            cert.cert.pem().into(),
            cert.signing_key.serialize_pem().into(),
        )
        .await
        .ok()?;

        let (tx, mut rx) = mpsc::channel::<String>(1);
        let handle = Handle::new();
        let shutdown_handle = handle.clone();

        let app = Router::new()
            .route(
                "/",
                get(
                    move |Query(params): Query<HashMap<String, String>>,
                          State(tx): State<mpsc::Sender<String>>| {
                        async move {
                            if let Some(code) = params.get("code") {
                                let _ = tx.send(code.clone()).await;
                                return Html("<h1>Success!</h1><p>Return to your app.</p>");
                            }
                            Html("<h1>Error</h1><p>No code found.</p>")
                        }
                    },
                ),
            )
            .with_state(tx);

        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let server_handle = tokio::spawn(async move {
            axum_server::bind_rustls(addr, config)
                .handle(handle)
                .serve(app.into_make_service())
                .await
                .unwrap();
        });

        println!(
            "📡 Waiting for redirect on https://localhost:{}/callback...",
            port
        );

        let captured_code = rx.recv().await;
        shutdown_handle.graceful_shutdown(Some(Duration::from_secs(1)));

        let _ = server_handle.await;

        captured_code
    }

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

        let rt = Runtime::new().unwrap();

        let code = rt.block_on(async { Self::wait_for_oauth_code(7777).await });

        return code;
    }

    pub fn exchange_code(
        &self,
        client_id: &String,
        client_secret: &String,
        code: &String,
    ) -> Option<()> {
        let client = Client::new();
        let mut form_data: HashMap<&str, &str> = HashMap::new();
        form_data.insert("client_id", client_id.as_str());
        form_data.insert("client_secret", client_secret.as_str());
        form_data.insert("code", code.as_str());

        let rt = Runtime::new().unwrap();

        let response = rt
            .block_on(async {
                let response = client
                    .post("https://slack.com/api/oauth.v2.access")
                    .form(&form_data)
                    .send()
                    .await
                    .unwrap();
                response.text().await
            })
            .ok()?;

        println!("{}", response);

        // let cache_code = String::from("oauth.v2.access");
        // store_cache(cache_code.to_string(), text.clone())?;

        // let result: entities::slack::authorization::Authorization =
        //     serde_json::from_str(&text.as_str())?;
        //
        // Ok(result)
        None
    }
}
