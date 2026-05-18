use muddle_window::{run_muddle_window_hosts_from_env_args, MuddleWindowHostRegistration};
use tigris_sim::parliament_ai_muddle_host;

fn main() -> std::io::Result<()> {
    run_muddle_window_hosts_from_env_args(vec![MuddleWindowHostRegistration {
        name: "tigris-parliament-ai",
        description: "TIGRIS Parliament: product-owned MUDDLE window host with a deterministic AI opponent.",
        suggested_commands: "`go board`, `draft axis`, `place tiger`, `end turn`, `challenge ai`.",
        create: || Box::new(parliament_ai_muddle_host()),
    }])
}
