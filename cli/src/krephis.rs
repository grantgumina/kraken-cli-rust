// Auth/http manager for kraken server
// https://mtgcardsmith.com/view/krephis-kraken-overlord

use hyper::rt::{self, Future, Stream};
use hyper::{Client, Body, Request, StatusCode};
use hyper::header::{HeaderValue, HeaderMap};
use hyper_tls::HttpsConnector;
use kraken_utils;
use prettytable::{Table};

// new job command WILL NOT work if this is HTTPS?!? The thread seems to panic or something. Zero visibility.
static BASE_URL: &str = "http://kraken-grantgumina.herokuapp.com";
// static BASE_URL: &str = "http://localhost:5000";

pub fn login(email: String, password: String) {

    let fut = fetch_token(email, password).map(|response| {

        kraken_utils::store_token(&response.token);

    }).map_err(|e| {
        match e {
            kraken_utils::FetchError::Http(e) => {
                eprintln!("http error: {}", e);
            },
            kraken_utils::FetchError::Json(e) => {
                eprintln!("json parsing error: {}", e);
            },
            kraken_utils::FetchError::KrakenServerError(e) => {
                eprintln!("Server error: {}", e.message);
            },
            kraken_utils::FetchError::Other(e) => {
                eprintln!("Error: {}", e);
            }
        }
    });

    rt::run(fut);
}

pub fn logout() {

    kraken_utils::store_token("empty");

}

pub fn show_jobs() {
    
    match kraken_utils::retrieve_token() {
       
        Ok(token) => {
            
            let fut = fetch_jobs(&token).map(|jobs_response| {

                let mut table = Table::new();
                table.add_row(row![b->"Job Name", b->"Description", b->"Status"]);

                for job in &jobs_response {

                    let mut d = Some(String::new());
                    let mut s = Some(String::new());

                    if job.description.is_some() && job.description != Some("".to_string()) {
                        d = job.description.clone();
                    }

                    if job.status.is_some() && job.status != Some("".to_string()) {
                        s = job.status.clone();
                    }
                    
                    table.add_row(row![job.name, d.unwrap(), s.unwrap()]);
                }

                table.printstd();

            }).map_err(|e| {

                match e {
                    kraken_utils::FetchError::Http(e) => {
                        eprintln!("http error: {}", e);
                    },
                    kraken_utils::FetchError::Json(e) => {
                        eprintln!("json parsing error: {}", e);
                    },
                    kraken_utils::FetchError::KrakenServerError(e) => {
                        eprintln!("Server error: {}", e.message);
                    },
                    kraken_utils::FetchError::Other(e) => {
                        eprintln!("Error: {}", e);
                    }
                }

            });

            rt::run(fut);

        },

        Err(error) => {
            eprintln!("Run `kraken login` to authenticate this machine. Token will be stored in ~/.krakenrc.\n{}", error)
        }

    }

}

pub fn show_job(job_id: &str, line_limit: &str) {

    match kraken_utils::retrieve_token() {
        
        Ok(token) => {

            let fut = fetch_logs(&token, job_id, line_limit).map(|logs_response| {

                for log in &logs_response {
                    println!("{}", log.line);
                }

            }).map_err(|e| {

                match e {
                    kraken_utils::FetchError::Http(e) => {
                        eprintln!("http error: {}", e);
                    },
                    kraken_utils::FetchError::Json(e) => {
                        eprintln!("json parsing error: {}", e);
                    },
                    kraken_utils::FetchError::KrakenServerError(e) => {
                        eprintln!("Server error: {}", e.message);
                    },                    
                    kraken_utils::FetchError::Other(e) => {
                        eprintln!("Error: {}", e);
                    }
                }

            });

            rt::run(fut);

        },

        Err(error) => {
            eprintln!("Run `kraken login` to authenticate this machine. Token will be stored in ~/.krakenrc.\n{}", error)
        }

    }

}

pub fn new_job(machine_name: &str, job_name: &str, description: &str) {

    match kraken_utils::retrieve_token() {
       
        Ok(token) => {

            let fut = create_job_async(&token, machine_name, job_name, description).map(|_response| {
            }).map_err(|e| {

                match e {
                    kraken_utils::FetchError::Http(e) => {
                        eprintln!("http error: {}", e);
                    },
                    kraken_utils::FetchError::Json(e) => {
                        eprintln!("json parsing error: {}", e);
                    },
                    kraken_utils::FetchError::KrakenServerError(e) => {
                        eprintln!("Server error: {}", e.message);
                    },                    
                    kraken_utils::FetchError::Other(e) => {
                        eprintln!("Error: {}", e);
                    }
                }

            });

            rt::run(fut);
        },

        Err(error) => {
            eprintln!("Run `kraken login` to authenticate this machine. Token will be stored in ~/.krakenrc.\n{}", error)
        }

    }

}

pub fn new_log(job_id: &str, line: &str) {
    
    match kraken_utils::retrieve_token() {
       
        Ok(token) => {

            let fut = create_log_async(&token, job_id, line).map(|_response| {
                
            }).map_err(|e| {

                match e {
                    kraken_utils::FetchError::Http(e) => {
                        eprintln!("http error: {}", e);
                    },
                    kraken_utils::FetchError::Json(e) => {
                        eprintln!("json parsing error: {}", e);
                    },
                    kraken_utils::FetchError::KrakenServerError(e) => {
                        eprintln!("Server error: {}", e.message);
                    },                    
                    kraken_utils::FetchError::Other(e) => {
                        eprintln!("Error: {}", e);
                    }
                }

            });

            rt::run(fut);
        },

        Err(error) => {
            eprintln!("Run `kraken login` to authenticate this machine. Token will be stored in ~/.krakenrc.\n{}", error)
        }

    }
}

pub fn remove_job(job_name: &str) {

    match kraken_utils::retrieve_token() {
       
        Ok(token) => {

            let fut = remove_job_async(&token, job_name).map(|_response| {
                
            }).map_err(|e| {

                match e {
                    kraken_utils::FetchError::Http(e) => {
                        eprintln!("http error: {}", e);
                    },
                    kraken_utils::FetchError::Json(e) => {
                        eprintln!("json parsing error: {}", e);
                    },
                    kraken_utils::FetchError::KrakenServerError(e) => {
                        eprintln!("Server error: {}", e.message);
                    },                    
                    kraken_utils::FetchError::Other(e) => {
                        eprintln!("Error: {}", e);
                    }
                }

            });

            rt::run(fut);
        },

        Err(error) => {
            eprintln!("Run `kraken login` to authenticate this machine. Token will be stored in ~/.krakenrc.\n{}", error)
        }

    }

}

pub fn remove_all_jobs() {
     match kraken_utils::retrieve_token() {
       
        Ok(token) => {

            let fut = remove_all_jobs_async(&token).map(|_response| {
                
            }).map_err(|e| {

                match e {
                    kraken_utils::FetchError::Http(e) => {
                        eprintln!("http error: {}", e);
                    },
                    kraken_utils::FetchError::Json(e) => {
                        eprintln!("json parsing error: {}", e);
                    },
                    kraken_utils::FetchError::KrakenServerError(e) => {
                        eprintln!("Server error: {}", e.message);
                    },                    
                    kraken_utils::FetchError::Other(e) => {
                        eprintln!("Error: {}", e);
                    }
                }

            });

            rt::run(fut);
        },

        Err(error) => {
            eprintln!("Run `kraken login` to authenticate this machine. Token will be stored in ~/.krakenrc.\n{}", error)
        }

    }
}

// Futures functions
fn fetch_jobs(token: &str) -> impl Future<Item = Vec<kraken_utils::JobJSON>, Error = kraken_utils::FetchError> {
    
    let https_connector = HttpsConnector::new(4).unwrap();
    let client = Client::builder().build(https_connector);
    let mut req = Request::new(Body::empty());

    let url: hyper::Uri = format!("{}/jobs", BASE_URL).parse().unwrap();    
    let mut headers = HeaderMap::new();
    let method = hyper::Method::GET;
    
    headers.insert("x-access-token", HeaderValue::from_str(&token).unwrap());

    *req.method_mut() = method;
    *req.uri_mut() = url;
    *req.headers_mut() = headers;

    client.request(req).and_then(|res| {
        
        let status = res.status();
        res.into_body().concat2().and_then(move |body| Ok((status, body)))

    }).from_err::<kraken_utils::FetchError>()
    .and_then(|(status, body)| {

        if status == 500 {
            let error_json: kraken_utils::ErrorJSON = serde_json::from_slice(&body)?;
            Err(kraken_utils::FetchError::KrakenServerError(error_json))
        } else {
            let json_response: Vec<kraken_utils::JobJSON> = serde_json::from_slice(&body)?;
            Ok(json_response)
        }

        
    }).from_err()
    
}

fn fetch_logs(token: &str, job_id: &str, line_limit: &str) -> impl Future<Item = Vec<kraken_utils::LogJSON>, Error = kraken_utils::FetchError> {
    
    let https_connector = HttpsConnector::new(4).unwrap();
    let client = Client::builder().build(https_connector);
    let mut req = Request::new(Body::empty());
    let url: hyper::Uri = format!("{}/jobs/{}", BASE_URL, job_id).parse().unwrap();    
    let mut headers = HeaderMap::new();
    let method = hyper::Method::GET;

    headers.insert("x-access-token", HeaderValue::from_str(token).unwrap());
    headers.insert("x-line-limit", HeaderValue::from_str(line_limit).unwrap());

    *req.method_mut() = method;
    *req.uri_mut() = url;
    *req.headers_mut() = headers;

    client.request(req).and_then(|res| {
        
        let status = res.status();
        res.into_body().concat2().and_then(move |body| Ok((status, body)))

    }).from_err::<kraken_utils::FetchError>()
    .and_then(|(status, body)| {
        
        if status == 500 {
            let error_json: kraken_utils::ErrorJSON = serde_json::from_slice(&body)?;
            Err(kraken_utils::FetchError::KrakenServerError(error_json))
        } else {
            let json_response: Vec<kraken_utils::LogJSON> = serde_json::from_slice(&body)?;
            Ok(json_response)
        }

        
    }).from_err()

}

fn fetch_token(email: String, password: String) -> impl Future<Item = kraken_utils::TokenJSON, Error = kraken_utils::FetchError> {
    
    let url: hyper::Uri = format!("{}/auth/login", BASE_URL).parse().unwrap();
    let https_connector = HttpsConnector::new(4).unwrap();
    let client = Client::builder().build(https_connector);
    let method = hyper::Method::POST;
    let mut headers = HeaderMap::new();

    let json_payload = json!({
        "email": email,
        "password": password
    });

    let mut req = Request::new(Body::from(json_payload.to_string()));

    headers.insert(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json")
    );

    *req.method_mut() = method;
    *req.uri_mut() = url;
    *req.headers_mut() = headers;

    client.request(req).and_then(|res| {
       let status = res.status();
        res.into_body().concat2().and_then(move |body| Ok((status, body)))

    }).from_err::<kraken_utils::FetchError>()
    .and_then(|(status, body)| {

        if status == 500 {
            let error_json: kraken_utils::ErrorJSON = serde_json::from_slice(&body)?;
            Err(kraken_utils::FetchError::KrakenServerError(error_json))
        } else {
            let json_response: kraken_utils::TokenJSON = serde_json::from_slice(&body)?;
            Ok(json_response)
        }
    }).from_err()

}

fn create_job_async(token: &str, machine_name: &str, job_name: &str, description: &str) -> impl Future<Item = StatusCode, Error = kraken_utils::FetchError> {
    
    let url: hyper::Uri = format!("{}/jobs/new", BASE_URL).parse().unwrap();
    let https_connector = HttpsConnector::new(4).unwrap();
    let client = Client::builder().build(https_connector);
    let method = hyper::Method::POST;
    let mut headers = HeaderMap::new();

    let json_payload = json!({
        "machine": machine_name,
        "name": job_name,
        "description": description
    });

    let mut req = Request::new(Body::from(json_payload.to_string()));

    headers.insert("x-access-token", HeaderValue::from_str(&token).unwrap());
    headers.insert(hyper::header::CONTENT_TYPE, HeaderValue::from_static("application/json"));

    *req.method_mut() = method;
    *req.uri_mut() = url;
    *req.headers_mut() = headers;

    client.request(req).and_then(|res| {
        Ok(res.status())
    }).from_err::<kraken_utils::FetchError>().from_err()

}

fn create_log_async(token: &str, job_name: &str, line: &str) -> impl Future<Item = StatusCode, Error = kraken_utils::FetchError> {

    let url: hyper::Uri = format!("{}/logs/new", BASE_URL).parse().unwrap();
   
    let https_connector = HttpsConnector::new(4).unwrap();
    let client = Client::builder().build(https_connector);
    let method = hyper::Method::POST;
    let mut headers = HeaderMap::new();

    let json_payload = json!({
        "jobName": job_name,
        "line": line,
    });

    let mut req = Request::new(Body::from(json_payload.to_string()));

    headers.insert("x-access-token", HeaderValue::from_str(&token).unwrap());
    headers.insert(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json")
    );

    *req.method_mut() = method;
    *req.uri_mut() = url;
    *req.headers_mut() = headers;

    client.request(req).and_then(|res| {
        Ok(res.status())
    }).from_err::<kraken_utils::FetchError>().from_err()

}

fn remove_job_async(token: &str, job_name: &str) -> impl Future<Item = StatusCode, Error = kraken_utils::FetchError> {
    let url: hyper::Uri = format!("{}/jobs/{}", BASE_URL, job_name).parse().unwrap();
   
    let https_connector = HttpsConnector::new(4).unwrap();
    let client = Client::builder().build(https_connector);
    let method = hyper::Method::DELETE;
    let mut headers = HeaderMap::new();

    let mut req = Request::new(Body::empty());

    headers.insert("x-access-token", HeaderValue::from_str(&token).unwrap());
    headers.insert(hyper::header::CONTENT_TYPE, HeaderValue::from_static("application/json"));

    *req.method_mut() = method;
    *req.uri_mut() = url;
    *req.headers_mut() = headers;

    client.request(req).and_then(|res| {
        Ok(res.status())
    }).from_err::<kraken_utils::FetchError>().from_err()

}

fn remove_all_jobs_async(token: &str) -> impl Future<Item = StatusCode, Error = kraken_utils::FetchError> {

   let url: hyper::Uri = format!("{}/jobs/remove-all", BASE_URL).parse().unwrap();
   
    let https_connector = HttpsConnector::new(4).unwrap();
    let client = Client::builder().build(https_connector);
    let method = hyper::Method::POST;
    let mut headers = HeaderMap::new();

    let mut req = Request::new(Body::empty());

    headers.insert("x-access-token", HeaderValue::from_str(&token).unwrap());
    headers.insert(hyper::header::CONTENT_TYPE, HeaderValue::from_static("application/json"));

    *req.method_mut() = method;
    *req.uri_mut() = url;
    *req.headers_mut() = headers;

    client.request(req).and_then(|res| {
        Ok(res.status())
    }).from_err::<kraken_utils::FetchError>().from_err()

}