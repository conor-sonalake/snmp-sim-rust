/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResponseDevice : An managed device as a response body.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseDevice {
    #[serde(rename = "agent")]
    pub agent: Box<crate::models::ResponseDeviceAgent>,
    /// Device's optional description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The unique identifier of this managed device.
    #[serde(rename = "id")]
    pub id: String,
    /// Device's name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "snmp_protocol_attributes")]
    pub snmp_protocol_attributes: Box<crate::models::RequestDeviceSnmpProtocolAttributes>,
}

impl ResponseDevice {
    /// An managed device as a response body.
    pub fn new(agent: crate::models::ResponseDeviceAgent, id: String, name: String, snmp_protocol_attributes: crate::models::RequestDeviceSnmpProtocolAttributes) -> ResponseDevice {
        ResponseDevice {
            agent: Box::new(agent),
            description: None,
            id,
            name,
            snmp_protocol_attributes: Box::new(snmp_protocol_attributes),
        }
    }
}


