use std::io::{self, Write};

use crate::{
    app_config::AppConfig,
    args::NoteCommand,
    editor::{Editor, ParseTemplate},
    formatters::NoteSearchFormatter,
    web_client,
};

const template: &'static str = r#"#tags = ["work", "important"]
#tags = []
#today = true
date = "2024-01-01"
+++"#;

pub async fn note_cmd(config: &AppConfig, subcommand: NoteCommand) -> Result<(), anyhow::Error> {
    let mut client = web_client::get_client(config);

    match subcommand {
        NoteCommand::Add(args) => {
            let editor = Editor::new();

            let note = if args.edit {
                print!("\x1B[?1049h");
                io::stdout().flush()?;
                let content = editor.with_initial_content(&template)?.parse_template()?;
                let tags: Vec<String> = content.tags.into_iter().collect();
                let result = client
                    .create_note(content.content, tags, content.today)
                    .await?;

                // Restore state and ensure buffer is cleared properly
                print!("\x1B[?1049l\x1B[H\x1B[2J");
                io::stdout().flush()?; // Important to flush here too

                result
            } else {
                client
                    .create_note(args.content.join(" "), vec![], args.today)
                    .await?
            };

            println!("Note added successfully ({})", note.id);
        }
        NoteCommand::Search(args) => {
            let notes = client.search(&args).await?;
            let mut formatter = NoteSearchFormatter::new(args);

            formatter
                .print_notes(&notes.notes)
                .map_err(|e| anyhow::anyhow!("Error while formatting notes: {}", e))?;
        }
    };

    Ok(())
}
