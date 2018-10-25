use errors::*;

use db;
use shell::Readline;
use structopt::StructOpt;
use models::*;
use term;


#[derive(Debug, StructOpt)]
pub enum Args {
    #[structopt(name="domains")]
    Domains(Filter),
    #[structopt(name="subdomains")]
    Subdomains(Filter),
    #[structopt(name="ipaddrs")]
    IpAddrs(Filter),
    #[structopt(name="urls")]
    Urls(Filter),
    #[structopt(name="emails")]
    Emails(Filter),
}

#[derive(Debug, StructOpt)]
pub struct Filter {
    args: Vec<String>,
}

impl Filter {
    pub fn parse(&self) -> Result<db::Filter> {
        db::Filter::parse(&self.args)
    }
}

pub fn run(rl: &mut Readline, args: &[String]) -> Result<()> {
    let args = Args::from_iter_safe(args)?;
    let rows = match args {
        Args::Domains(filter) => delete::<Domain>(rl, &filter),
        Args::Subdomains(filter) => delete::<Subdomain>(rl, &filter),
        Args::IpAddrs(filter) => delete::<IpAddr>(rl, &filter),
        Args::Urls(filter) => delete::<Url>(rl, &filter),
        Args::Emails(filter) => delete::<Email>(rl, &filter),
    }?;
    term::info(&format!("Deleted {} rows", rows));
    Ok(())
}

fn delete<T: Model + Detailed>(rl: &mut Readline, filter: &Filter) -> Result<usize> {
    rl.db().delete::<T>(&filter.parse()?)
}
