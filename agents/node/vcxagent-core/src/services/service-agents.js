const {
  Agent
} = require('@hyperledger/node-vcx-wrapper')

module.exports.createServiceAgents = function createServiceAgents ({ logger, saveAgent, loadAgent }) {
  async function publicAgentCreate (agentId, institutionDid) {
    logger.info(`Creating public agent with id ${agentId} for institution did ${institutionDid}`)
    const agent = await Agent.create(institutionDid)
    await saveAgent(agentId, agent)
    return agent
  }

  async function getPublicInvite (agentId, label) {
    logger.info(`Public agent with id ${agentId} is creating public invite with label ${label}`)
    const agent = await loadAgent(agentId)
    return agent.createPublicInvite(label)
  }

  return {
    publicAgentCreate,
    getPublicInvite
  }
}
