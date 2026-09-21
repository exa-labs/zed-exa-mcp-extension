The Exa API key is optional — the server works anonymously with rate limits. A key raises your limits and is required for `agent_run` (Exa Agent).

1. Sign up for an [Exa API account](https://dashboard.exa.ai)
2. Generate your API key from [dashboard.exa.ai/api-keys](https://dashboard.exa.ai/api-keys)

By default `web_search_exa` and `web_fetch_exa` are enabled. To enable more tools, set `tools` to a comma-separated list — note it **replaces** the defaults, so list everything you want. For example:

```
"tools": "web_search_exa,web_fetch_exa,agent_run"
```

Available tools: `web_search_exa`, `web_fetch_exa`, `web_search_advanced_exa`, `agent_run` (requires an API key)
