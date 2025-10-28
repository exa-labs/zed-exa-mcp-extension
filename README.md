# mcp-server-exa-search

Zed extension that connects to Exa's hosted MCP server via HTTP. This extension provides access to Exa's web search, code search, and crawling capabilities through the Model Context Protocol.

## Features

The Exa MCP server provides these tools for Zed:

- **get_code_context_exa**: Search and get relevant code snippets, examples, and documentation from open source libraries, GitHub repositories, and programming frameworks
- **web_search_exa**: Performs real-time web searches with optimized results and content extraction
- **crawling**: Extracts content from specific URLs, useful for reading articles, PDFs, or any web page
- **company_research**: Comprehensive company research tool that crawls company websites
- **deep_researcher_start/check**: Start and check AI-powered deep research tasks
- **linkedin_search**: Search LinkedIn for companies and people

## Architecture

This extension uses Exa's hosted HTTP MCP server at `https://mcp.exa.ai/mcp` via the `mcp-remote` CLI tool. This approach:

- Requires no local installation of the exa-mcp-server package
- Connects directly to Exa's hosted infrastructure
- Works without an API key for basic functionality
- Supports both production (`mcp.exa.ai`) and staging (`mcp.exa.sh`) environments

## Configuration

### Basic Usage (No API Key Required)

The extension works out of the box without any configuration. Simply install it and start using Exa's MCP tools in Zed's agent mode.

### Optional: API Key Configuration

For enhanced rate limits or premium features, you can optionally provide an Exa API key:

1. Sign up for an [Exa API account](https://dashboard.exa.ai)
2. Generate your API key from [dashboard.exa.ai/api-keys](https://dashboard.exa.ai/api-keys)

In your Zed settings:
```json
{
    "context_servers": {
        "mcp-server-exa-search": {
          "settings": {
              "exa_api_key": "YOUR_API_KEY"
          }
        }
    }
}
```

### Advanced: Custom MCP Server URL

To use a different MCP server URL (e.g., staging environment):

```json
{
    "context_servers": {
        "mcp-server-exa-search": {
          "settings": {
              "mcp_url": "https://mcp.exa.sh/mcp"
          }
        }
    }
}
```

Alternatively, set the `EXA_MCP_URL` environment variable:
```bash
export EXA_MCP_URL=https://mcp.exa.sh/mcp
```

### Agent Mode Configuration

If you're using Zed's agent mode, you need to enable this context server for your assistant:

1. Open Zed's assistant settings
2. Enable the Exa MCP tool in the tools panel
3. Enable the Exa MCP tool in the active assistant profile. In the chat section, click on the 'Write|Ask' button, then click on 'tools', then enable the Exa MCP tool

## Testing

The extension includes integration tests to verify connectivity and functionality:

```bash
# Run all integration tests (requires npx and internet connection)
cargo test --test mcp_integration_test -- --ignored

# Test specific functionality
cargo test --test mcp_integration_test test_mcp_server_connectivity -- --ignored
cargo test --test mcp_integration_test test_mcp_server_search_functionality -- --ignored

# Test staging environment
cargo test --test mcp_integration_test test_staging_environment -- --ignored
```

## Requirements

- Node.js and npm/npx (for running mcp-remote)
- Internet connection (to reach Exa's hosted MCP server)

## Troubleshooting

### Connection Issues

If you experience connection issues:

1. Verify you have Node.js installed: `node --version`
2. Test the mcp-remote tool directly: `npx -y mcp-remote https://mcp.exa.ai/mcp`
3. Check your internet connection
4. Try the staging environment: Set `mcp_url` to `https://mcp.exa.sh/mcp`

### Tool Not Appearing

If the Exa tools don't appear in Zed:

1. Restart Zed completely
2. Check the extension is installed and enabled
3. Verify the context server is enabled in your assistant settings
