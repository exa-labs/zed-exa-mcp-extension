The Exa API key is optional for basic search, but required for deep research tools.

1. Sign up for an [Exa API account](https://dashboard.exa.ai)
2. Generate your API key from [dashboard.exa.ai/api-keys](https://dashboard.exa.ai/api-keys)

To enable additional tools (like deep research), set the `tools` field to a comma-separated list. For example:

```
"tools": "web_search_exa,get_code_context_exa,deep_researcher_start,deep_researcher_check"
```

Available tools: `web_search_exa`, `web_search_advanced_exa`, `get_code_context_exa`, `crawling_exa`, `company_research_exa`, `people_search_exa`, `deep_researcher_start`, `deep_researcher_check`, `deep_search_exa`
