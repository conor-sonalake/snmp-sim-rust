use crate::cli::CommandHandler;
use crate::operations::agent::{create_agent, delete_agent, list_agents, update_agent};
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) enum AgentCommands {
    /// List SNMP Agents
    Ls,

    /// Create a new instance of SNMP Agent
    Add(CreateAgent),

    /// Update an existing instance of SNMP Agent
    Update(UpdateAgent),

    /// Remove one or more SNMP Agents
    Rm(DeleteAgent),
}

#[derive(Debug, Args)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct CreateAgent {
    // name of the agent to be created
    #[clap(short, long)]
    name: String,
}

#[derive(Debug, Args)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct UpdateAgent {
    // unique identifier of an existing agent
    #[clap(short, long)]
    id: String,

    // new name of an agent
    #[clap(short, long)]
    name: String,
}

#[derive(Debug, Args)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct DeleteAgent {
    // unique identifier of an existing agent
    #[clap(short, long)]
    id: String,
}

impl CommandHandler for AgentCommands {
    fn handle(self) -> Result<(), anyhow::Error> {
        match self {
            AgentCommands::Ls => list_agents(),
            AgentCommands::Add(args) => create_agent(args),
            AgentCommands::Update(args) => update_agent(args),
            AgentCommands::Rm(args) => delete_agent(args),
        }
    }
}