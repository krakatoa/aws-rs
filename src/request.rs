use hyper::Client;
use hyper::client::Response;
use hyper::Result as HyperResult;
use signers::sigv4::SigV4;
use credentials::Credentials;

#[derive(Debug)]
pub struct ApiClient {
    signer: SigV4,
    service: String,
    endpoint: String
}

impl ApiClient {
    pub fn new(creds: Credentials, region: &str, service: &str) -> ApiClient{
        let sig = SigV4::new();
        let sig = sig.credentials(creds);
        let sig = sig.region(region);
        let sig = sig.service(service);

        let host = format!("{}.{}.amazonaws.com", service, region);
        let sig = sig.header(("Host", &host));

        let endpoint = match service {
          "glacier" => format!("https://{}/-/", host),
          _ => format!("https://{}/", host)
        };

        ApiClient {
            signer: sig,
            service: format!("{}", service),
            endpoint: endpoint
        }
    }

    pub fn get(self, action: &str) -> HyperResult<Response>{
        let sig = self.signer.clone();
        let sig = sig.method("GET");

        let query: String;
        let mut url: String;
        let path;

        let headers;

        if self.service == "glacier".to_string() {
          path = "/-/vaults";
          let sig = sig.path(path);
          url = format!("{}/{}", self.endpoint, action);

          headers = sig.as_headers();
        } else {
          path = "/";
          let sig = sig.path(path);

          let query = format!("Action={}&Version=2015-04-15", action);
          let sig = sig.query(&query);
          url = format!("{}?{}", self.endpoint, query);

          headers = sig.as_headers()
        };
        debug!("{}", url);

        let mut client = Client::new();
        let res = client.get(&url).headers(headers).send();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::ApiClient;
    use credentials::Credentials;

    #[test]
    fn test_new_apiclient() {
        let cred = Credentials::new().path("fixtures/credentials.ini").load();
        let region = "eu-west-1";
        let service = "ec2";

        let client = ApiClient::new(cred, region, service);
        assert_eq!(client.endpoint, "https://ec2.eu-west-1.amazonaws.com/")
    }
}
