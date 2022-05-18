/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RequestDevice : An agent as a response body.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestDevice {
    #[serde(rename = "agent_id")]
    pub agent_id: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of this agent.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "snmp_protocol_attributes")]
    pub snmp_protocol_attributes: Box<crate::models::RequestDeviceSnmpProtocolAttributes>,
}

impl RequestDevice {
    /// An agent as a response body.
    pub fn new(agent_id: String, name: String, snmp_protocol_attributes: crate::models::RequestDeviceSnmpProtocolAttributes) -> RequestDevice {
        RequestDevice {
            agent_id,
            description: None,
            name,
            snmp_protocol_attributes: Box::new(snmp_protocol_attributes),
        }
    }
}

