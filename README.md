# mcp-server-exa-search

Zed extension that connects to Exa's hosted MCP server via HTTP. This extension provides access to Exa's web search, code search, and crawling capabilities through the Model Context Protocol.

## Features

The Exa MCP server provides these tools for Zed:

- **get_code_context_exa**: Search and get relevant code snippets, examples, and documentation from open source libraries, GitHub repositories, and programming frameworks
- **web_search_exa**: Performs real-time web searches with optimized results and content extraction


## Configuration

### Basic Usage (No API Key Required)

The extension works out of the box without any configuration. Simply install it and start using Exa's MCP tools in Zed's agent mode.

### Optional: API Key Configuration

You can optionally provide an Exa API key:

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

### Agent Mode Configuration

If you're using Zed's agent mode, you need to enable this context server for your assistant:

1. Open Zed's assistant settings
2. Enable the Exa MCP tool in the tools panel
3. Enable the Exa MCP tool in the active assistant profile. In the chat section, click on the 'Write|Ask' button, then click on 'tools', then enable the Exa MCP tool

