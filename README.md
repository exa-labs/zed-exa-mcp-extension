# Exa MCP Server for Zed

A Zed extension that connects Zed's Agent Panel to Exa's hosted MCP server (`https://mcp.exa.ai/mcp`) via `mcp-remote`, giving your agent web search, page fetching, and Exa Agent tools.

## Install

Open Zed's Extensions view (`zed: extensions`), search for **Exa**, and click Install.

## Tools

**Enabled by default:**

| Tool | Description |
| --- | --- |
| `web_search_exa` | Search the web for any topic and get clean, ready-to-use content |
| `web_fetch_exa` | Read a webpage's full content as clean markdown from one or more URLs |

**Available via the `tools` setting:**

| Tool | Description |
| --- | --- |
| `web_search_advanced_exa` | Advanced search with filters, domains, dates, highlights, summaries, and subpage crawling |
| `agent_run` | Run an [Exa Agent](https://docs.exa.ai/reference/agent-api-guide) for multi-step research, list-building, enrichment, and structured output (requires an API key) |

Note: setting `tools` replaces the defaults entirely — list every tool you want enabled.

## Configuration

Add an `exa_api_key` and/or `tools` under the server's `settings` in `settings.json`:

```json
{
    "context_servers": {
        "mcp-server-exa-search": {
            "settings": {
                "exa_api_key": "YOUR_API_KEY",
                "tools": "web_search_exa,web_fetch_exa,agent_run"
            }
        }
    }
}
```

## Authentication

The hosted server works anonymously with rate limits — no key required for `web_search_exa` and `web_fetch_exa`. An API key from [dashboard.exa.ai/api-keys](https://dashboard.exa.ai/api-keys) raises your rate limits and unlocks `agent_run` (Exa Agent). The key is passed to the server as `?exaApiKey=`.

## Without the extension

Zed natively supports remote MCP servers (with OAuth), so you can skip the extension entirely and add this to `settings.json`:

```json
{
    "context_servers": {
        "exa": {
            "url": "https://mcp.exa.ai/mcp"
        }
    }
}
```

## Using it in the Agent Panel

Open the Agent Panel settings and check the indicator dot next to the Exa server — green means it's running. Tools can be toggled per agent profile. See the [Zed MCP docs](https://zed.dev/docs/ai/mcp) for details.

Full tool docs: [docs.exa.ai/reference/exa-mcp](https://docs.exa.ai/reference/exa-mcp)
