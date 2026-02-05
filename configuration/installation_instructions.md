**Recommended**: Use Zed's native remote MCP support instead of this extension. Add to your `settings.json`:

```json
{
    "context_servers": {
        "exa-mcp": {
            "url": "https://mcp.exa.ai/mcp"
        }
    }
}
```

No Node.js or extension required. Optionally append `?exaApiKey=YOUR_KEY` to the URL.

If using this extension instead, the Exa API key is optional:

1. Sign up for an [Exa API account](https://dashboard.exa.ai)
2. Generate your API key from [dashboard.exa.ai/api-keys](https://dashboard.exa.ai/api-keys)
