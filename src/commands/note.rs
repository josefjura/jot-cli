use chrono::Utc;

use crate::{
    args::{NoteCommand, NoteSearchArgs, OutputFormat},
    editor::Editor,
    formatters::NoteFormatter,
    web_client::Client,
};

const TEMPLATE: &str = r#"tags = ["work", "important"]
#tags = [""]
#date = "YYYY-MM-DD"
+++"#;

pub async fn note_cmd(
    mut client: Box<dyn Client>,
    subcommand: NoteCommand,
) -> Result<(), anyhow::Error> {
    match subcommand {
        NoteCommand::Add(args) => {
            let target_date = args.date.to_date(Utc::now().date_naive());

            if let Some(target_date) = target_date {
                let note = if args.edit {
                    let editor = Editor::new(TEMPLATE);
                    let result = editor.open(&args)?;

                    let tags = result.tags.iter().map(|t| t.to_string()).collect();

                    client
                        .create_note(result.content, tags, target_date)
                        .await?
                } else {
                    client
                        .create_note(args.content.join(" "), args.tag, target_date)
                        .await?
                };

                NoteFormatter::new(OutputFormat::Pretty).print_notes(&[note])?;
            } else {
                println!("Invalid date");
            }
        }
        NoteCommand::Search(args) => {
            let notes = client.search(&args).await?;
            let mut formatter = NoteFormatter::new(args.output);

            formatter
                .print_notes(&notes.notes)
                .map_err(|e| anyhow::anyhow!("Error while formatting notes: {}", e))?;

            if args.delete {
                println!("Do you want to delete these notes? [y/N]");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() == "y" {
                    let ids: Vec<i64> = notes
                        .notes
                        .into_iter()
                        .filter_map(|n| -> Option<i64> { n.id })
                        .collect();
                    client.delete(&ids).await?;
                }
            }
        }
        NoteCommand::Last(args) => {
            let args = NoteSearchArgs {
                term: args.term,
                tag: args.tag,
                date: None,
                lines: None,
                limit: Some(1),
                output: args.output,
                delete: false,
            };
            let notes = client.search(&args).await?;

            let mut formatter = NoteFormatter::new(args.output);

            formatter
                .print_notes(&notes.notes)
                .map_err(|e| anyhow::anyhow!("Error while formatting notes: {}", e))?;
        }
    };

    Ok(())
}
