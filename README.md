# mcp-server-exa-search

Zed extension that connects to Exa's hosted MCP server at [https://mcp.exa.ai/mcp](https://mcp.exa.ai/mcp). This extension supports both web search functionality and retrieval of web page contents, powered by the Exa Search API.

## Features

The Exa MCP server provides these tools for Zed:

- **web_search**: Performs real-time web searches with optimized results and content extraction
- **crawling**: Extracts content from specific URLs, useful for reading articles, PDFs, or any web page
- **get_code_context_exa**: Search and get relevant code snippets, examples, and documentation from open source libraries and GitHub repositories

## Configuration

This MCP server can be used with or without an API key.

**Without API key:** The server works via [Smithery](https://smithery.ai/server/exa) with rate limits. Simply install the extension and start using it.

**With API key (recommended):** For full access and higher rate limits:

1. Sign up for an [Exa API account](https://dashboard.exa.ai)
2. Generate your API key from [dashboard.exa.ai/api-keys](https://dashboard.exa.ai/api-keys)
3. Add the API key to your Zed settings:

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
2. Enable the exa MCP tool
3. Enable the exa MCP tool in the active assistant profile. In the chat section, click on the 'Write|Ask' button, then click on 'tools', then enable the exa MCP tool.

## Technical Details

This extension uses the HTTP-based MCP protocol to connect to Exa's hosted server at `https://mcp.exa.ai/mcp`. When an API key is provided, it's passed via the Authorization header as a Bearer token.
