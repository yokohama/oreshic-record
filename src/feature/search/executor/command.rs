use anyhow::Result;

use crate::{
    context::Context,
    feature::{
        search::{
            table::{
                md_records,
                section_records,
            },
            executor::{
                common,
                flag,
                SearchExecutor,
            },
        },
    },
};

pub struct Command {
    pub number: Vec<usize>, 
    pub run: bool, 
    pub del: bool,
    pub open: bool,
    pub is_track: bool,
}

impl Command {
    pub fn new(
        number: Vec<usize>,
        run: bool,
        del: bool,
        open: bool,
        is_track: bool,
    ) -> Self {
        Self {
            number,
            run,
            del,
            open,
            is_track,
        }
    }
}

impl SearchExecutor for Command {
    fn run(&self, ctx: &Context) -> Result<()> {

        let target_dir = if self.is_track {
            &ctx.tracks_dir
        } else {
            &ctx.commands_dir
        };

        match self.number.as_slice() {
            [] => {
                let records = common::collect_records(&target_dir)?;
                md_records::print_md_table(&records)
            },
            [md] => {
                let md_index = *md;

                let records = common::collect_records(&target_dir)?;
                let md = &records[md_index];
                let sections = common::collect_sections(&md)?;

                if self.open {
                    flag::open::exe(&md.path)?;
                } else {
                    section_records::print_section_table(&sections)
                }
            },
            [md, section] => {
                let md_index = *md;
                let section_index = *section;

                let records = common::collect_records(&target_dir)?;
                let md = &records[md_index];
                let sections = common::collect_sections(&md)?;

                let section = sections
                    .get(section_index)
                    .ok_or_else(|| anyhow::anyhow!("invalid section index"))?;

                if self.run {
                    flag::run::exe(section)?;
                } else if self.del {

                    let start_line = section.start_line
                        .ok_or_else(|| anyhow::anyhow!("section has no start_line"))?;

                    let end_line = section.end_line
                        .ok_or_else(|| anyhow::anyhow!("section has no end_line"))?;


                    flag::del::exe(
                        &md.path,
                        start_line,
                        end_line
                    )?;
                } else {
                    common::print_section(section)?
                }
            },
            _ => {
                return Err(anyhow::anyhow!("number must be at most 2"));
            }
        }

        Ok(())
    }
}
