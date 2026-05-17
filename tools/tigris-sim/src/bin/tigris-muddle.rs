use muddle_cli::{run_muddle_host_from_env_args, MuddleCliHostInfo};
use tigris_sim::parliament_ai_muddle_host;

fn main() -> std::io::Result<()> {
    let mut host = parliament_ai_muddle_host();
    run_muddle_host_from_env_args(
        &mut host,
        MuddleCliHostInfo {
            name: "tigris-parliament-ai",
            description:
                "TIGRIS Parliament: product-owned MUDDLE host with a deterministic AI opponent.",
            suggested_commands:
                "`go board`, `draft axis`, `place tiger`, `end turn`, `challenge ai`, `quit`.",
        },
    )
    .map(|_| ())
}
