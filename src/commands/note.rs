use crate::{
    app_config::AppConfig, args::NoteCommand, formatters::NoteSearchFormatter, web_client,
};

pub async fn note_cmd(config: &AppConfig, subcommand: NoteCommand) -> Result<(), anyhow::Error> {
    let mut client = web_client::get_client(config);

    match subcommand {
        NoteCommand::Add(args) => {
            let note = client
                .create_note(args.content.join(" "), args.today)
                .await?;
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
