//! External MCP agent entrypoint. Connects to the app Unix socket when implemented.

fn main() {
    println!("pitaya-mcp {}", env!("CARGO_PKG_VERSION"));
}
