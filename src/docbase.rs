extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;
extern crate serde_json;
extern crate jsonway;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use self::hyper::Client;
use self::hyper::{Method, Request};
use self::hyper::header::{ContentType};
use self::hyper_tls::HttpsConnector;
use self::tokio_core::reactor::Core;
use self::futures::{Future, Stream};
use self::serde_json::Value;
use super::Args;

pub struct Docbase {
}

impl Docbase {
    pub fn new() -> Docbase {
        Docbase {}
    }
    pub fn run(&mut self, args: Args) {
        if args.cmd_post {
            self.execute_post(args.arg_post_file_path, args.arg_post_title);
        } else {
            println!("{:?}", args);
        }
    }

    // TODO: リファクタリング、APIリクエスト部分の共通化
    fn execute_post(&self, post_file_path: Vec<String>, post_title: Vec<String>) {
        let docbase_domain = get_domain();

        let docbase_base_uri = "https://api.docbase.io/teams/";
        let docbase_uri = format!("{}{}{}", docbase_base_uri, docbase_domain, "/posts");
        let docbase_token = env::var("DOCBASE_TOKEN").unwrap();

        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(1, &handle).unwrap())
            .build(&handle);

        let title = &post_title[0];
        let post_file = &post_file_path[0];
        let path = Path::new(post_file);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };

        let mut s = String::new();
        let body = match file.read_to_string(&mut s) {
            Err(why) => panic!("Couldn't read {}: {}", display, Error::description(&why)),
            Ok(_) => s
        };

        let json = jsonway::object(|json| {
            json.set("title", title.to_string());
            json.set("body", body.to_string());
            json.set("draft", "false".to_string()); //TODO: true/falseをargumemtで指定可能にする
            //json.set("tags", ["tag1", "tag2"]); //TODO: tag付けのI/F検討
            //json.set("scope", "everyone".to-string()); //TODO: scope定義の検討
        }).unwrap().to_string();

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
    execute_https_request();
    env::var("DOCBASE_DOMAIN").unwrap().replace("\"", "")
}

fn execute_https_request() {
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
    env::var("DOCBASE_DOMAIN").unwrap().replace("\"", "");
}

