/*
 * Rust client test spec
 *
 * Special testing for the Rust client generator
 *
 * The version of the OpenAPI document: 1.0.7
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;

use super::{Error, configuration};

pub struct DefaultApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl DefaultApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> DefaultApiClient {
        DefaultApiClient {
            configuration,
        }
    }
}

pub trait DefaultApi {
    fn dummy_get(&self, ) -> Result<(), Error>;
}

impl DefaultApi for DefaultApiClient {
    fn dummy_get(&self, ) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/dummy", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

}
