const fs = require('fs');

// Read the auto-generated index.js
let content = fs.readFileSync('index.js', 'utf8');

// Replace the simple exports with our wrapper functions
const wrapperCode = `
const { composeSystemPrompt: nativeComposeSystemPrompt, composeSystemPromptWithPromptsDir: nativeComposeSystemPromptWithPromptsDir, listAvailableDomains, listAvailableBehaviors, listAvailableDomainsWithPromptsDir, listAvailableBehaviorsWithPromptsDir, isAvailable: nativeIsAvailable, getStatus: nativeGetStatus } = nativeBinding

// Wrapper function to handle JSON conversion for composeSystemPrompt
async function composeSystemPrompt(request) {
  const path = require('path');
  const promptsDir = path.join(__dirname, 'prompts');
  
  if (typeof request === 'object') {
    // Convert object to JSON string
    const requestJson = JSON.stringify(request);
    const responseJson = nativeComposeSystemPromptWithPromptsDir(requestJson, promptsDir);
    const response = JSON.parse(responseJson);
    
    // Add metadata
    response.source = 'native';
    response.version = require('./package.json').version;
    
    return response;
  } else {
    // Already a string, pass through
    const responseJson = nativeComposeSystemPromptWithPromptsDir(request, promptsDir);
    const response = JSON.parse(responseJson);
    response.source = 'native';
    return response;
  }
}

// Wrapper for getStatus to parse JSON response and make async
async function getStatus() {
  try {
    const statusJson = nativeGetStatus();
    return JSON.parse(statusJson);
  } catch (error) {
    return {
      available: false,
      source: 'native',
      error: error.message,
      domains: [],
      behaviors: []
    };
  }
}

// Async wrapper for isAvailable (for compatibility)
async function isAvailable() {
  return nativeIsAvailable();
}

// Async wrapper for domain listing
async function listAvailableDomainsAsync() {
  const path = require('path');
  const promptsDir = path.join(__dirname, 'prompts');
  
  try {
    return listAvailableDomainsWithPromptsDir(promptsDir);
  } catch (error) {
    // Fallback to default if prompts dir function fails
    return listAvailableDomains();
  }
}

// Async wrapper for behavior listing  
async function listAvailableBehaviorsAsync() {
  const path = require('path');
  const promptsDir = path.join(__dirname, 'prompts');
  
  try {
    return listAvailableBehaviorsWithPromptsDir(promptsDir);
  } catch (error) {
    // Fallback to default if prompts dir function fails
    return listAvailableBehaviors();
  }
}

module.exports.composeSystemPrompt = composeSystemPrompt
module.exports.listAvailableDomains = listAvailableDomainsAsync
module.exports.listAvailableBehaviors = listAvailableBehaviorsAsync
module.exports.isAvailable = isAvailable
module.exports.getStatus = getStatus`;

// Replace the original exports
const originalExports = `const { composeSystemPrompt, composeSystemPromptWithPromptsDir, listAvailableDomains, listAvailableDomainsWithPromptsDir, listAvailableBehaviors, listAvailableBehaviorsWithPromptsDir, isAvailable, getStatus } = nativeBinding

module.exports.composeSystemPrompt = composeSystemPrompt
module.exports.composeSystemPromptWithPromptsDir = composeSystemPromptWithPromptsDir
module.exports.listAvailableDomains = listAvailableDomains
module.exports.listAvailableDomainsWithPromptsDir = listAvailableDomainsWithPromptsDir
module.exports.listAvailableBehaviors = listAvailableBehaviors
module.exports.listAvailableBehaviorsWithPromptsDir = listAvailableBehaviorsWithPromptsDir
module.exports.isAvailable = isAvailable
module.exports.getStatus = getStatus`;

content = content.replace(originalExports, wrapperCode);

// Write back the modified content
fs.writeFileSync('index.js', content);

console.log('✅ Added async wrapper functions to index.js');
