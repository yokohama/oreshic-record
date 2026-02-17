use anyhow::Result;

use crate::{
    context::Context,
    feature::search::{
        executor::{
            common,
            flag,
            SearchExecutor,
        },
        table::md_records,
    },
};

pub struct Writeup {
    pub number: Vec<usize>, 
    pub open: bool,
}

impl Writeup {
    pub fn new(
        number: Vec<usize>,
        open: bool,
    ) -> Self {
        Self {
            number,
            open,
        }
    }
}

impl SearchExecutor for Writeup {
    fn run(&self, ctx: &Context) -> Result<()> {
        match self.number.as_slice() {
            [] => {
                let records = common::collect_records(&ctx.writeups_dir)?;
                md_records::print_md_table(&records)
            },
            [md] => {
                let md_index = *md;

                let records = common::collect_records(&ctx.writeups_dir)?;
                let md = records[md_index].clone();

                if self.open {
                    flag::open::exe(&md.path)?;
                } else {
                    common::print_md(&md.path)?;
                }
            },
            _ => {
                anyhow::bail!("Too many arguments")
            }
        }

        Ok(())
    }
}
