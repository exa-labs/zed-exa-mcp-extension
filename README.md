# mcp-server-exa-search

Zed extension that connects to Exa's hosted MCP server at [https://mcp.exa.ai/mcp](https://mcp.exa.ai/mcp) via HTTP transport using [mcp-remote](https://github.com/modelcontextprotocol/mcp-remote). This extension supports web search functionality and retrieval of web page contents.

## Features

The Exa MCP server provides these tools for Zed:

- **web_search_exa**: Performs real-time web searches with optimized results and content extraction
- **crawling**: Extracts content from specific URLs, useful for reading articles, PDFs, or any web page
- **get_code_context_exa**: Retrieves code context from repositories

## Configuration

The API key is **optional** - the server works without an API key via Smithery (with rate limits).

For full access without rate limits:

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

If you don't provide an API key, the extension will still work with rate-limited access via Smithery.

### Agent Mode Configuration

If you're using Zed's agent mode, you need to enable this context server for your assistant:

1. Open Zed's assistant settings
2. Enable the exa MCP tool. If you see that the status of the tool is a red dot, make sure you added your exa api key in settings
3. Enable the exa MCP tool in the active assistant profile. In the chat section, click on the 'Write|Ask' button, then click on 'tools', then enable the exa MCP tool.
