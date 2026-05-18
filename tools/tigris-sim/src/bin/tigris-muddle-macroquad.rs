use macroquad::prelude::*;
use muddle_core::MuddleClientHostRegistration;
use muddle_macroquad::{
    apply_default_macroquad_paths, macroquad_usage, macroquad_window_conf,
    parse_macroquad_run_options, run_muddle_macroquad_hosts, MuddleMacroquadRunConfig,
};
use tigris_sim::parliament_ai_muddle_host;

const HOST_NAME: &str = "tigris-parliament-ai";

fn window_conf() -> Conf {
    macroquad_window_conf("TIGRIS Parliament AI")
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut options = match parse_macroquad_run_options(std::env::args().skip(1)) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("{error}");
            eprintln!("{}", macroquad_usage());
            return;
        }
    };
    if options.host_name.is_none() && !options.list_hosts && !options.show_help {
        options.host_name = Some(HOST_NAME.to_string());
    }
    apply_default_macroquad_paths(
        &mut options,
        "tigris-parliament.macroquad.muddle",
        "tigris-parliament.macroquad.txt",
        "tigris-parliament.import.muddle",
        "tigris-parliament.export.muddle",
    );

    let registrations = vec![MuddleClientHostRegistration {
        name: HOST_NAME,
        category: "Games",
        description: "TIGRIS Parliament: native AI-opponent amendment slice.",
        suggested_commands:
            "`choose persona`, `go board`, `draft axis`, `stake claim`, `reveal collision`, `place tiger`, `end turn`, `challenge ai`, `go score`, `score amendment`, `close parliament`.",
        create: || Box::new(parliament_ai_muddle_host()),
    }];

    if let Err(error) = run_muddle_macroquad_hosts(
        registrations,
        options,
        MuddleMacroquadRunConfig {
            screen_title: "TIGRIS Parliament AI".to_string(),
        },
    )
    .await
    {
        eprintln!("{error}");
    }
}
