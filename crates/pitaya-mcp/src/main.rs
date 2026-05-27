//! External MCP agent entrypoint. Connects to the app Unix socket in P5.

fn main() {
    println!("pitaya-mcp {}", env!("CARGO_PKG_VERSION"));
}
