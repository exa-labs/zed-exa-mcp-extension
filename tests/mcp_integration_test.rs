use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

const TEST_TIMEOUT_SECS: u64 = 30;

struct McpClient {
    child: Child,
}

impl McpClient {
    fn new(mcp_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let child = Command::new("npx")
            .args(&["-y", "mcp-remote", mcp_url])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        Ok(McpClient { child })
    }

    fn send_request(&mut self, method: &str, params: Value) -> Result<Value, Box<dyn std::error::Error>> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        });

        let request_str = serde_json::to_string(&request)?;
        let content = format!("Content-Length: {}\r\n\r\n{}", request_str.len(), request_str);

        let stdin = self.child.stdin.as_mut().ok_or("Failed to get stdin")?;
        stdin.write_all(content.as_bytes())?;
        stdin.flush()?;

        let stdout = self.child.stdout.as_mut().ok_or("Failed to get stdout")?;
        let mut reader = BufReader::new(stdout);

        let mut headers = String::new();
        let mut content_length = 0;

        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            
            if line.trim().is_empty() {
                break;
            }
            
            if line.starts_with("Content-Length:") {
                content_length = line
                    .trim()
                    .strip_prefix("Content-Length:")
                    .ok_or("Invalid Content-Length header")?
                    .trim()
                    .parse()?;
            }
            
            headers.push_str(&line);
        }

        if content_length == 0 {
            return Err("No Content-Length header found".into());
        }

        let mut body = vec![0u8; content_length];
        reader.read_exact(&mut body)?;

        let response: Value = serde_json::from_slice(&body)?;
        Ok(response)
    }
}

impl Drop for McpClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
#[ignore] // Run with: cargo test --test mcp_integration_test -- --ignored
fn test_mcp_server_connectivity() {
    let mcp_url = std::env::var("EXA_MCP_URL").unwrap_or_else(|_| "https://mcp.exa.ai/mcp".to_string());
    
    println!("Testing MCP server connectivity to: {}", mcp_url);

    let mut client = McpClient::new(&mcp_url).expect("Failed to start MCP client");

    std::thread::sleep(Duration::from_secs(2));

    let response = client
        .send_request("initialize", json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "zed-exa-test",
                "version": "0.1.0"
            }
        }))
        .expect("Failed to send initialize request");

    println!("Initialize response: {}", serde_json::to_string_pretty(&response).unwrap());

    assert!(response.get("result").is_some(), "Initialize should return a result");
    
    let result = response.get("result").unwrap();
    assert!(result.get("protocolVersion").is_some(), "Response should include protocolVersion");
    assert!(result.get("serverInfo").is_some(), "Response should include serverInfo");
}

#[test]
#[ignore] // Run with: cargo test --test mcp_integration_test -- --ignored
fn test_mcp_server_list_tools() {
    let mcp_url = std::env::var("EXA_MCP_URL").unwrap_or_else(|_| "https://mcp.exa.ai/mcp".to_string());
    
    println!("Testing MCP server tools list from: {}", mcp_url);

    let mut client = McpClient::new(&mcp_url).expect("Failed to start MCP client");

    std::thread::sleep(Duration::from_secs(2));

    client
        .send_request("initialize", json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "zed-exa-test",
                "version": "0.1.0"
            }
        }))
        .expect("Failed to initialize");

    let response = client
        .send_request("tools/list", json!({}))
        .expect("Failed to list tools");

    println!("Tools list response: {}", serde_json::to_string_pretty(&response).unwrap());

    assert!(response.get("result").is_some(), "tools/list should return a result");
    
    let result = response.get("result").unwrap();
    let tools = result.get("tools").expect("Result should include tools array");
    let tools_array = tools.as_array().expect("Tools should be an array");

    assert!(!tools_array.is_empty(), "Tools array should not be empty");

    let tool_names: Vec<String> = tools_array
        .iter()
        .filter_map(|t| t.get("name").and_then(|n| n.as_str()).map(String::from))
        .collect();

    println!("Available tools: {:?}", tool_names);

    assert!(
        tool_names.iter().any(|name| name.contains("search") || name.contains("exa")),
        "Should have at least one search-related tool"
    );
}

#[test]
#[ignore] // Run with: cargo test --test mcp_integration_test -- --ignored
fn test_mcp_server_search_functionality() {
    let mcp_url = std::env::var("EXA_MCP_URL").unwrap_or_else(|_| "https://mcp.exa.ai/mcp".to_string());
    
    println!("Testing MCP server search functionality from: {}", mcp_url);

    let mut client = McpClient::new(&mcp_url).expect("Failed to start MCP client");

    std::thread::sleep(Duration::from_secs(2));

    client
        .send_request("initialize", json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "zed-exa-test",
                "version": "0.1.0"
            }
        }))
        .expect("Failed to initialize");

    let tools_response = client
        .send_request("tools/list", json!({}))
        .expect("Failed to list tools");

    let tools = tools_response
        .get("result")
        .and_then(|r| r.get("tools"))
        .and_then(|t| t.as_array())
        .expect("Failed to get tools array");

    let search_tool = tools
        .iter()
        .find(|t| {
            t.get("name")
                .and_then(|n| n.as_str())
                .map(|name| name.contains("search") || name == "web_search_exa")
                .unwrap_or(false)
        })
        .expect("Should find a search tool");

    let tool_name = search_tool
        .get("name")
        .and_then(|n| n.as_str())
        .expect("Tool should have a name");

    println!("Testing search tool: {}", tool_name);

    let search_response = client
        .send_request("tools/call", json!({
            "name": tool_name,
            "arguments": {
                "query": "rust programming language",
                "numResults": 3
            }
        }))
        .expect("Failed to call search tool");

    println!("Search response: {}", serde_json::to_string_pretty(&search_response).unwrap());

    assert!(search_response.get("result").is_some(), "Search should return a result");
    
    let result = search_response.get("result").unwrap();
    assert!(result.get("content").is_some(), "Result should include content");
}

#[test]
#[ignore] // Run with: cargo test --test mcp_integration_test -- --ignored
fn test_staging_environment() {
    let mcp_url = "https://mcp.exa.sh/mcp";
    
    println!("Testing staging MCP server connectivity to: {}", mcp_url);

    let mut client = McpClient::new(mcp_url).expect("Failed to start MCP client");

    std::thread::sleep(Duration::from_secs(2));

    let response = client
        .send_request("initialize", json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "zed-exa-test-staging",
                "version": "0.1.0"
            }
        }))
        .expect("Failed to send initialize request to staging");

    println!("Staging initialize response: {}", serde_json::to_string_pretty(&response).unwrap());

    assert!(response.get("result").is_some(), "Staging initialize should return a result");
}
