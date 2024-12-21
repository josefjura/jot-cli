use std::io::{self, Write};

use chrono::{Local, NaiveDate};

use crate::{
    app_config::AppConfig,
    args::{NoteCommand, NoteSearchArgs},
    editor::{Editor, ParseTemplate},
    formatters::NoteSearchFormatter,
    model::Note,
    web_client,
};

const TEMPLATE: &'static str = r#"#tags = ["work", "important"]
#tags = []
date = "today"
#date = "YYYY-MM-DD"
+++"#;

pub async fn note_cmd(config: &AppConfig, subcommand: NoteCommand) -> Result<(), anyhow::Error> {
    let mut client = web_client::get_client(config);

    match subcommand {
        NoteCommand::Add(args) => {
            let note = if args.edit {
                let editor = Editor::new(&TEMPLATE);
                let result = editor.open(&args)?;

                let tags = result.tags.iter().map(|t| t.to_string()).collect();

                client
                    .create_note(
                        result.content,
                        tags,
                        result.date.unwrap_or(Local::now().date_naive()),
                    )
                    .await?
            } else {
                client
                    .create_note(args.content.join(" "), args.tag, args.date.to_date())
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
        NoteCommand::Last(args) => {
            let args = NoteSearchArgs {
                term: args.term,
                tag: args.tag,
                date: None,
                lines: None,
                limit: Some(1),
                output: args.output,
            };
            let notes = client.search(&args).await?;

            let mut formatter = NoteSearchFormatter::new(args);

            formatter
                .print_notes(&notes.notes)
                .map_err(|e| anyhow::anyhow!("Error while formatting notes: {}", e))?;
        }
    };

    Ok(())
}
