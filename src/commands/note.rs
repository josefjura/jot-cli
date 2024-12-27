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
            let note = if args.edit {
                let editor = Editor::new(TEMPLATE);
                let result = editor.open(&args)?;

                let tags = result.tags.iter().map(|t| t.to_string()).collect();

                client
                    .create_note(result.content, tags, result.date.to_date())
                    .await?
            } else {
                client
                    .create_note(args.content.join(" "), args.tag, args.date.to_date())
                    .await?
            };

            NoteFormatter::new(OutputFormat::Pretty).print_notes(&[note])?;
        }
        NoteCommand::Search(args) => {
            let notes = client.search(&args).await?;
            let mut formatter = NoteFormatter::new(args.output);

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

            let mut formatter = NoteFormatter::new(args.output);

            formatter
                .print_notes(&notes.notes)
                .map_err(|e| anyhow::anyhow!("Error while formatting notes: {}", e))?;
        }
    };

    Ok(())
}
