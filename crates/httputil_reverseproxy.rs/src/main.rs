extern crate hyper;

use better_panic;
use log::{debug, error, info};

use http;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;
use tokio;
use tower::limit::rate::{RateLimit, RateLimitLayer};
// use tower_limit::rate::{RateLimit, RateLimitLayer};
use tower::builder::ServiceBuilder;
use tower::retry;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type HttpsClient =
    hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

#[derive(Debug, Clone)]
pub struct ReverseProxy {
    pub target_uri: http::uri::Uri,
    pub target_host: String,
    pub target_host_header: hyper::header::HeaderValue,
    pub target_headers: hyper::HeaderMap,
    pub client_https:
        hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
    pub client_http: hyper::Client<hyper::client::HttpConnector, hyper::Body>,
}

impl ReverseProxy {
    pub fn new(target_uri: http::uri::Uri) -> Result<Self, GenericError> {
        let https = hyper_tls::HttpsConnector::new();
        let client_https: HttpsClient =
            hyper::Client::builder().build::<_, hyper::Body>(https);
        let client_http = hyper::Client::new();

        let target_host = target_uri.host().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ReverseProxy.new - cannot get host from target URI",
        ))?;
        let target_host_header =
            hyper::header::HeaderValue::from_str(target_host)?;
        let user_agent: String = format!(
            "banaio-{}/{} ({})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_REPOSITORY"),
        );
        let authorization_header = ReverseProxy::authorization_header()?;

        let mut headers = hyper::HeaderMap::new();
        headers.insert(hyper::header::CACHE_CONTROL, "no-cache".parse()?);
        headers.insert(hyper::header::HOST, target_host_header.clone());
        headers.insert(hyper::header::FORWARDED, target_host_header.clone());
        headers.insert(
            hyper::header::HeaderName::from_lowercase(b"x-forwarded-host")?,
            target_host_header.clone(),
        );
        headers.insert(hyper::header::USER_AGENT, user_agent.parse()?);
        headers.insert(hyper::header::AUTHORIZATION, authorization_header);
        // headers.insert(hyper::header::PROXY_AUTHORIZATION, authorization_header);

        info!("ReverseProxy.new - target_headers={:?}", headers);
        debug!("ReverseProxy.new - target_headers={:?}", headers);

        Ok(Self {
            target_uri: target_uri.clone(),
            target_host: target_host.to_owned(),
            target_host_header,
            target_headers: headers,
            client_https,
            client_http,
        })
    }

    pub fn authorization_header(
    ) -> Result<hyper::header::HeaderValue, hyper::header::InvalidHeaderValue>
    {
        // 	expected := "Basic " + base64.StdEncoding.EncodeToString([]byte(projectID+":"+projectSecret))
        // "Basic ZjJiYjVmNmE5MDIzNDA3MmI2NzdmY2ZhZDdjNGRjOWM6ZTM5ZjhjNjNjNTJkNDdiZmJmNzk4MDNjMDc4OTUwYTk=".parse()
        "Basic OmZjOTkxMmViYWMwMzRjYjdiNWM0YTFlMWZlOTMxMmQ1".parse()
    }

    pub async fn handle<'a, 'b>(
        &self,
        mut request: Request<Body>,
    ) -> Result<Response<Body>, hyper::Error> {
        info!("ReverseProxy.handle - incoming request={:?}", request);

        let headers = self.target_headers.clone();
        request.headers_mut().extend(headers);
        *request.uri_mut() = self.target_uri.clone();
        // info!("ReverseProxy.handle - outgoing request={:?}", request);

        let response = self.client_https.request(request).await?;
        if response.status() != hyper::StatusCode::OK {
            error!("ReverseProxy.handle - response={:?}", response);
        } /*else {
              info!("ReverseProxy.handle - response={:?}", response);
          }*/

        Ok(response)
    }

    pub async fn shutdown_signal(&self) {
        // Wait for the CTRL+C signal
        tokio::signal::ctrl_c()
            .await
            .expect("ReverseProxy.shutdown_signal - failed to install CTRL+C signal handler");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // async fn main() {
    // https://github.com/hyperium/hyper/blob/master/examples/http_proxy.xrs

    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "trace");

    better_panic::Settings::debug()
        .most_recent_first(true)
        .backtrace_first(true)
        .lineno_suffix(true)
        .verbosity(better_panic::Verbosity::Full)
        .install();

    let cli = Cli::default();
    let log_level = cli.log_level.as_str();
    let rust_log = [
        log_level.into(),
        format!("hyper={}", log_level),
        format!("tower_limit={}", log_level),
        format!("tower={}", log_level),
    ]
    .join(",");
    std::env::set_var("RUST_LOG", rust_log);
    env_logger::init();
    info!("cli={:?}", cli);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let reverse_proxy = std::sync::Arc::new(ReverseProxy::new(cli.target_uri)?);

    use std::sync::Arc;

    // https://infura.io/docs/ethereum/json-rpc/ratelimits
    let allowed_rps = 1;
    let layer =
        RateLimitLayer::new(allowed_rps, std::time::Duration::from_secs(1));

    let make_svc_fn = |remote_addr: Arc<std::net::SocketAddr>,
                       reverse_proxy: Arc<ReverseProxy>| {
        service_fn(move |req: Request<Body>| {
            // info!("service_fn - remote_addr={:?}", remote_addr);
            let reverse_proxy = reverse_proxy.clone();
            let remote_addr = remote_addr.clone();
            async move {
                // info!("service_fn - async move - remote_addr={:?}", remote_addr);
                let res = reverse_proxy.handle(req).await;
                res
            }
        })
    };
    let make_svc = |reverse_proxy: Arc<ReverseProxy>| {
        let reverse_proxy = reverse_proxy.clone();
        make_service_fn(move |addr_stream: &hyper::server::conn::AddrStream| {
            let remote_addr = Arc::new(addr_stream.remote_addr());
            // info!("make_service_fn - remote_addr={:?}", remote_addr);
            let remote_addr = remote_addr.clone();
            let reverse_proxy = reverse_proxy.clone();
            async move {
                // use hyper::Error;
                use std::convert::Infallible;
                Ok::<_, Infallible>(make_svc_fn(remote_addr, reverse_proxy))
            }
        })
    };
    let new_service = make_svc(reverse_proxy.clone());
    let mut svc = tower::ServiceBuilder::new()
        .rate_limit(allowed_rps, std::time::Duration::from_secs(1))
        .service(new_service);
    let server = Server::bind(&addr).serve(svc);
    // And now add a graceful shutdown signal...
    let graceful_shutdown =
        server.with_graceful_shutdown(reverse_proxy.shutdown_signal());
    info!("reverse_proxy - listening on http://{}", addr);

    // Run this server for... forever!
    if let Err(error) = graceful_shutdown.await {
        error!("graceful_shutdown - error={}", error);
    } else {
        info!("graceful_shutdown - received ctrl-c event");
        info!("graceful_shutdown - CTRL+C handled");
    }

    Ok(())
}

use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug, Hash, PartialEq, Eq)]
#[structopt(
    name = "httputil_reverseproxy",
    global_settings = &[
        // structopt::clap::AppSettings::ColoredHelp,
        structopt::clap::AppSettings::ColorAuto,
        structopt::clap::AppSettings::VersionlessSubcommands,
        structopt::clap::AppSettings::ArgRequiredElseHelp,
        structopt::clap::AppSettings::DisableVersion,
        structopt::clap::AppSettings::DeriveDisplayOrder,
        structopt::clap::AppSettings::NextLineHelp,
    ],
    rename_all = "kebab-case",
)]
pub struct Cli {
    #[structopt(
        short = "l",
        long,
        help = "The level to configure the logger.",
        possible_values = &LogLevel::variants(),
        default_value = "info",
        case_insensitive = true
    )]
    pub log_level: LogLevel,
    #[structopt(
        short = "c",
        long,
        help = "The config file to use.",
        parse(from_os_str),
        default_value = "config.json"
    )]
    pub config: std::path::PathBuf,
    #[structopt(
        short = "t",
        long,
        help = "The target URI.",
        parse(try_from_str = http::uri::Uri::from_str),
        default_value = "https://mainnet.infura.io/v3/7cc6248aacc14235bc563ef289a5168b"
    )]
    pub target_uri: http::uri::Uri,
}

// PROJECT ID
// 7cc6248aacc14235bc563ef289a5168b
// PROJECT SECRET
// fc9912ebac034cb7b5c4a1e1fe9312d5
// ENDPOINTS

// https://mainnet.infura.io/v3/7cc6248aacc14235bc563ef289a5168b
// wss://mainnet.infura.io/ws/v3/7cc6248aacc14235bc563ef289a5168b

use clap::arg_enum;
arg_enum! {
    #[derive(StructOpt, Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
    pub enum LogLevel {
        Error,
        Warn,
        Info,
        Debug,
        Trace,
    }
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        use LogLevel::*;
        match self {
            Error => "error",
            Warn => "warn",
            Info => "info",
            Debug => "debug",
            Trace => "trace",
        }
    }
}

// impl std::str::FromStr for LogLevel {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "error" => Ok(LogLevel::Error),
//             "warn" => Ok(LogLevel::Warn),
//             "info" => Ok(LogLevel::Info),
//             "debug" => Ok(LogLevel::Debug),
//             "trace" => Ok(LogLevel::Trace),
//             s => Err(format!("{} is not a valid option for LogLevel", s)),
//         }
//     }
// }

impl Default for Cli {
    fn default() -> Cli {
        Cli::from_args()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_cli() {
        assert_eq!("1", "1")
    }
}
