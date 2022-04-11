/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RequestAgent : An agent as a response body.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestAgent {
    /// The name of this agent.
    #[serde(rename = "name")]
    pub name: String,
}

impl RequestAgent {
    /// An agent as a response body.
    pub fn new(name: String) -> RequestAgent {
        RequestAgent {
            name,
        }
    }
}


