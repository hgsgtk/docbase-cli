extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;
extern crate serde_json;
extern crate dotenv;

use std::env;
use self::hyper::Client;
use self::hyper::{Method, Request};
use self::hyper::header::{ContentType};
use self::hyper_tls::HttpsConnector;
use self::tokio_core::reactor::Core;
use self::serde_json::Value;
use self::dotenv::dotenv;
use self::futures::{Future, Stream};
use super::Args;

pub struct Docbase {
}

impl Docbase {
    pub fn new() -> Docbase {
        Docbase {}
    }
    pub fn run(&mut self, args: Args) {
        if args.cmd_post {
            self.execute_post();
        } else {
            println!("{:?}", args);
        }
    }

    // TODO: リファクタリング、APIリクエスト部分の共通化
    fn execute_post(&self) {
        let docbase_domain = get_domain();

        dotenv().ok();
        let docbase_base_uri = "https://api.docbase.io/teams/";
        let docbase_uri = format!("{}{}{}", docbase_base_uri, docbase_domain, "/posts");
        let docbase_token = env::var("DOCBASE_TOKEN").unwrap();

        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(1, &handle).unwrap())
            .build(&handle);

        let json = r#"{
            "title": "test post from cli",
	        "body": "This is the test post from cli written in Rustlang.",
	        "draft": true,
	        "tags": ["tag1", "tag2"],
	        "scope": "everyone",
	        "notice": false
        }"#;

        let uri = docbase_uri.parse().unwrap();
        let mut req = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set_raw("X-Api-Version", "1");
        req.headers_mut().set_raw("X-DocBaseToken", docbase_token);
        req.set_body(json);

        let post = client.request(req).and_then(|res| {
            println!("POST: {}", res.status());
            res.body().concat2().and_then(move |body| {
                let v: Value = serde_json::from_slice(&body).unwrap();
                println!("Success! The url posted is {}.", v["url"].to_string());
                Ok(())
            })
        });
        core.run(post).unwrap();
    }
}

fn get_domain() -> String {
    // TODO: .envファイルが存在しない場合のエラーハンドリング
    dotenv().ok();
    let docbase_uri = "https://api.docbase.io/teams";
    let docbase_token = env::var("DOCBASE_TOKEN").unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(1, &handle).unwrap())
        .build(&handle);

    let uri = docbase_uri.parse().unwrap();
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set_raw("X-Api-Version", "1");
    req.headers_mut().set_raw("X-DocBaseToken", docbase_token);

    let get = client.request(req).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            let v: Value = serde_json::from_slice(&body).unwrap();
            env::set_var("DOCBASE_DOMAIN", v[0]["domain"].to_string()); //TODO:複数ドメイン対応
            Ok(())
        })
    });
    core.run(get).unwrap();
    env::var("DOCBASE_DOMAIN").unwrap().replace("\"", "")
}
