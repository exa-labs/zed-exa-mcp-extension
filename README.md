# mcp-server-exa-search

Zed extension that wraps the `exa-mcp-server` package from [exa-labs/exa-mcp-server](https://github.com/exa-labs/exa-mcp-server). This extension supports both web search functionality and retrieval of web page contents.

## Features

The Exa MCP server provides several tools:

- **web_search**: Performs real-time web searches with optimized results and content extraction
- **research_paper_search**: Specialized search focused on academic papers and research content
- **twitter_search**: Dedicated Twitter/X.com search that finds tweets, profiles, and conversations
- **company_research**: Comprehensive company research tool that crawls company websites
- **crawling**: Extracts content from specific URLs, useful for reading articles, PDFs, or any web page
- **competitor_finder**: Identifies competitors of a company

## Configuration

This MCP server requires an API key.

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
