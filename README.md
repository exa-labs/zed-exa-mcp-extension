# mcp-server-exa-search

Zed extension that connects to Exa's hosted MCP server for web search, code search, and crawling capabilities through the Model Context Protocol.

## Setup (Recommended: Native URL)

Zed supports remote MCP servers natively via URL. Add this to your Zed `settings.json`:

```json
{
    "context_servers": {
        "exa-mcp": {
            "url": "https://mcp.exa.ai/mcp"
        }
    }
}
```

With an API key:

```json
{
    "context_servers": {
        "exa-mcp": {
            "url": "https://mcp.exa.ai/mcp?exaApiKey=YOUR_API_KEY"
        }
    }
}
```

No extension installation or Node.js required.

## Setup (Alternative: Extension)

If you prefer the extension approach, install this extension from Zed's extension marketplace. It uses the `mcp-remote` npm package under the hood, which requires Node.js.

### Optional: API Key Configuration

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

## Features

- **web_search_exa**: Real-time web searches with optimized results and content extraction
- **get_code_context_exa**: Search and get relevant code snippets, examples, and documentation from open source libraries, GitHub repositories, and programming frameworks

## Agent Mode

1. Open Zed's assistant settings
2. Enable the Exa MCP tool in the tools panel
3. In the chat section, click on the 'Write|Ask' button, then click on 'tools', then enable the Exa MCP tool

