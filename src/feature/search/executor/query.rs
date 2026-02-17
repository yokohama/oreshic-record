use anyhow::Result;

use crate::{
    context::Context,
    feature::search::{
        executor::{
            common,
            flag,
            SearchExecutor,
        },
        table::section_records,
    },
    ors_type::RecordType,
};

pub struct Query {
    pub word: String,
    pub number: Option<usize>, 
    pub run: bool, 
    pub del: bool,
    pub open: bool,
}

impl Query {
    pub fn new(
        word: String,
        number: Option<usize>,
        run: bool,
        del: bool,
        open: bool,
    ) -> Self {
        Self {
            word,
            number,
            run,
            del,
            open,
        }
    }
}

impl SearchExecutor for Query {
    fn run(&self, ctx: &Context) -> Result<()> {

        let word = &self.word.to_lowercase();

        let mut records = common::collect_records(&ctx.commands_dir)?;

        let mut track_records = common::collect_records(&ctx.tracks_dir)?;
        records.append(&mut track_records);
        
        let mut writeup_records = common::collect_records(&ctx.writeups_dir)?;
        records.append(&mut writeup_records);
        
        let mut sections = Vec::new();

        for record in records {
            match record.record_type {
                RecordType::Command | RecordType::Track => {
                    let mut s = common::collect_sections(&record)?;
                    sections.append(&mut s);
                },
                RecordType::Writeup => {
                    let wr = common::collect_writeup(&record)?;
                    sections.append(&mut vec![wr]);
                }
            }
        }

        sections.retain(|section| {
            let mut title_hit = false;
            let mut command_hit = false;

            match section.record_type {
                RecordType::Command | RecordType::Track => {
                    title_hit = section
                        .title
                        .as_ref()
                        .map(|s| s.to_lowercase().contains(word))
                        .unwrap_or(false);
        
                    command_hit = section
                        .command
                        .as_ref()
                        .map(|s| s.to_lowercase().contains(word))
                        .unwrap_or(false);
                },
                _ => (),
            };

            let tag_hit = section
                .tags
                .iter()
                .any(|t| t.to_lowercase().contains(word));
        
            title_hit || command_hit || tag_hit
        });

        for (i, section) in sections.iter_mut().enumerate() {
            section.index = i;
        }

        if let Some(n) = self.number {
            let section = sections
                .get(n)
                .ok_or_else(|| anyhow::anyhow!("invalid section index"))?;
        
            if self.run {
                flag::run::exe(section)?;
            } else if self.del {
                match section.record_type {
                    RecordType::Command | RecordType::Track => {
                        let start_line = section.start_line
                            .ok_or_else(|| anyhow::anyhow!("section has no start_line"))?;
                        let end_line = section.end_line
                            .ok_or_else(|| anyhow::anyhow!("section has no end_line"))?;

                        flag::del::exe(
                            &section.path,
                            start_line,
                            end_line
                        )?;
                    },
                    RecordType::Writeup => {
                        flag::del::writeup(&section.path)?;
                    }
                }
            } else if self.open {
                flag::open::exe(&section.path)?;
            } else {
                match section.record_type {
                    RecordType::Writeup => {
                        common::print_md(&section.path)?;
                    },
                    _ => {
                        common::print_section(section)?;
                    }
                }
            }
        
        } else {
            section_records::print_section_table(&sections);
        }

        Ok(())
    }
}
