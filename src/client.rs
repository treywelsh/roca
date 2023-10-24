//! The client module acts as a wrapper of XML-RPC client to add OpenNebula related helpers

use serde_xmlrpc::Value;

use crate::common::Errors;
use crate::controller::RPCCaller;

/// The Roca XML-RPC basic client
#[derive(Debug)]
pub struct ClientXMLRPC {
    auth: String,
    endpoint: String,
    // TODO: add http client here and use a trait to abstract the HTTP client
}

impl ClientXMLRPC {
    // TODO, defines method for reading oen_auth

    pub fn new(auth: String, endpoint: String) -> ClientXMLRPC {
        ClientXMLRPC { auth, endpoint }
    }
}

impl RPCCaller for ClientXMLRPC {
    //Try to import https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
    // if works open a PR

    fn call(&self, name: &str, args: Vec<Value>) -> Result<String, Errors> {
        // TODO: remove this http client creation from here
        let client = reqwest::blocking::Client::new();

        let mut full_args = vec![Value::String(self.auth.clone())];
        full_args.extend(args);

        // TODO: remove unwrap
        let body = serde_xmlrpc::request_to_string(name, full_args).unwrap();

        let resp = match client.post(&self.endpoint).body(body).send() {
            Ok(r) => r,
            Err(e) => return Err(Errors::HTTPReq(e.to_string())),
        };

        let text = match resp.text() {
            Ok(t) => t,
            Err(e) => return Err(Errors::HTTPRespHandling(e.to_string())),
        };

        Ok(text)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn one_client() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:pDi4mFBHue"),
            String::from("http://192.168.33.10:2633/RPC2"),
        );

        let resp_txt = client.call("one.vn.info", vec![0.into()]).unwrap();
        let result = serde_xmlrpc::response_from_str::<(bool, String)>(&resp_txt);

        assert_eq!(result.unwrap().0, true);
    }

    #[test]
    fn one_rc() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:pDi4mFBHue"),
            String::from("http://192.168.33.10:2633/RPC2"),
        );

        let resp_txt = client.call("one.user.info", vec![0.into()]).unwrap();
        let result = serde_xmlrpc::response_from_str::<(bool, String)>(&resp_txt);

        assert_eq!(result.unwrap().0, true);
    }
}
