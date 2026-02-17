use std::io::Write;
use std::fs::OpenOptions;

use anyhow::Result;
use minijinja::{Environment, context};

const TEMPLATE_PATH: &str = include_str!("../../templates/record.md");

use crate::context::Context;

pub struct Md {
    pub title: Option<String>,
    pub message: Option<String>,
    pub command: Vec<String>,
    pub mode: String,
    pub result: Option<String>,
    pub tag: Option<String>,
    pub track_name: Option<String>,
}

impl Md {
    pub fn new(
        title: Option<String>, 
        message: Option<String>,
        command: Vec<String>,
        mode: String,
        result: Option<String>,
        tag: Option<String>,
        track_name: Option<String>,
    ) -> Self {

        Self {
            title,
            message,
            command,
            mode,
            result,
            tag,
            track_name,
        }
    }

    pub fn write(&self, ctx: &Context) -> Result<()> {

        // trackがセットされているか？なければコマンド名
        let md_file_path = match &self.track_name {
            Some(tn) => {
                ctx.tracks_dir.join(format!("{}.md", tn))
            },
            None => {
                let file_name = &self
                    .command
                    .first()
                    .ok_or_else(|| anyhow::anyhow!("command is enmpty"))?;
                ctx.commands_dir.join(format!("{}.md", file_name))
            }
        };

        let mut env = Environment::new();
        env.add_template("record", TEMPLATE_PATH)?;

        let tmpl = env.get_template("record")?;

        let content = tmpl.render(context! {
           title => self.title.as_deref().unwrap_or("{none}"),
           message => self.message.as_deref().unwrap_or("{none}"),
           command => self.command.join(" "),
           mode => self.mode,
           result => self.result.as_deref().unwrap_or("{none}"),
           tag => self.tag.as_deref().unwrap_or("{none}"),
        })?;


        let mut md_file = OpenOptions::new()
           .create(true)
           .append(true)
           .open(md_file_path)?;
    
        writeln!(md_file, "{}", content)?;
    
        Ok(())
    }
}
