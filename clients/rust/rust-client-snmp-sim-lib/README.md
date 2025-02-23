# Rust API client for rust-client-snmp-sim-lib

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 
- Package version: 0.4.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `rust-client-snmp-sim-lib` and add the following to `Cargo.toml` under `[dependencies]`:

```
rust-client-snmp-sim-lib = { path = "./rust-client-snmp-sim-lib" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AgentsApi* | [**agents_get**](docs/AgentsApi.md#agents_get) | **GET** /agents | List agents
*AgentsApi* | [**agents_id_delete**](docs/AgentsApi.md#agents_id_delete) | **DELETE** /agents/{id} | Delete agent by ID
*AgentsApi* | [**agents_id_get**](docs/AgentsApi.md#agents_id_get) | **GET** /agents/{id} | Get agent by ID
*AgentsApi* | [**agents_id_put**](docs/AgentsApi.md#agents_id_put) | **PUT** /agents/{id} | Update agent
*AgentsApi* | [**agents_post**](docs/AgentsApi.md#agents_post) | **POST** /agents | Create a new agent
*DevicesApi* | [**devices_get**](docs/DevicesApi.md#devices_get) | **GET** /devices | List managed devices
*DevicesApi* | [**devices_id_delete**](docs/DevicesApi.md#devices_id_delete) | **DELETE** /devices/{id} | Delete managed device by ID
*DevicesApi* | [**devices_id_get**](docs/DevicesApi.md#devices_id_get) | **GET** /devices/{id} | Get managed device by ID
*DevicesApi* | [**devices_id_put**](docs/DevicesApi.md#devices_id_put) | **PUT** /devices/{id} | Update managed device
*DevicesApi* | [**devices_id_start_put**](docs/DevicesApi.md#devices_id_start_put) | **PUT** /devices/{id}/start | Start an existing managed device
*DevicesApi* | [**devices_id_stop_put**](docs/DevicesApi.md#devices_id_stop_put) | **PUT** /devices/{id}/stop | Stop an existing managed device
*DevicesApi* | [**devices_post**](docs/DevicesApi.md#devices_post) | **POST** /devices | Create a new managed device


## Documentation For Models

 - [RequestAgent](docs/RequestAgent.md)
 - [RequestDevice](docs/RequestDevice.md)
 - [RequestDeviceSnmpProtocolAttributes](docs/RequestDeviceSnmpProtocolAttributes.md)
 - [RequestDeviceSnmpProtocolAttributesSnmpV1](docs/RequestDeviceSnmpProtocolAttributesSnmpV1.md)
 - [RequestDeviceSnmpProtocolAttributesSnmpV3](docs/RequestDeviceSnmpProtocolAttributesSnmpV3.md)
 - [ResponseAgent](docs/ResponseAgent.md)
 - [ResponseDevice](docs/ResponseDevice.md)
 - [ResponseDeviceAgent](docs/ResponseDeviceAgent.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



