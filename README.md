# mcp-server-exa-search

Zed extension that connects to Exa's hosted MCP server via HTTP. This extension provides access to Exa's web search, code search, deep research, and more through the Model Context Protocol.

## Features

**Enabled by default:**
- **web_search_exa**: Real-time web searches with optimized results and content extraction
- **get_code_context_exa**: Search and get relevant code snippets, examples, and documentation

**Available via `tools` setting:**
- **web_search_advanced_exa**: Advanced web search with full control over filters, domains, dates, and content options
- **crawling_exa**: Get the full content of a specific webpage from a known URL
- **company_research_exa**: Research any company to get business information, news, and insights
- **people_search_exa**: Find people and their professional profiles
- **deep_researcher_start**: Start an AI research agent that searches, reads, and writes a detailed report
- **deep_researcher_check**: Check status and get results from a deep research task
- **deep_search_exa**: Deep search with query expansion and synthesized answers (requires API key)

## Configuration

### Basic Usage (No API Key Required)

The extension works out of the box without any configuration. Simply install it and start using Exa's MCP tools in Zed's agent mode.

### Enabling Additional Tools

To enable tools beyond the defaults (e.g. deep research), add the `tools` setting with a comma-separated list:

```json
{
    "context_servers": {
        "mcp-server-exa-search": {
          "settings": {
              "exa_api_key": "YOUR_API_KEY",
              "tools": "web_search_exa,get_code_context_exa,deep_researcher_start,deep_researcher_check"
          }
        }
    }
}
```

### API Key Configuration

An API key is optional for basic search, but required for deep research tools and to avoid rate limits.

1. Sign up for an [Exa API account](https://dashboard.exa.ai)
2. Generate your API key from [dashboard.exa.ai/api-keys](https://dashboard.exa.ai/api-keys)

### Agent Mode Configuration

If you're using Zed's agent mode, you need to enable this context server for your assistant:

1. Open Zed's assistant settings
2. Enable the Exa MCP tool in the tools panel
3. Enable the Exa MCP tool in the active assistant profile. In the chat section, click on the 'Write|Ask' button, then click on 'tools', then enable the Exa MCP tool

