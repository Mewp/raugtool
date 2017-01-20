extern crate augeas;
extern crate rustyline;

use augeas::{Augeas, AugFlag};
mod augcmd {
    use augeas::{Augeas};
    use augeas::error::Error;

    #[derive(Debug, Clone)]
    pub enum Command {
        Foreign(String),
        Reorder(String, bool, String),
    }
    include!(concat!(env!("OUT_DIR"), "/augcmd.rs"));

    pub fn run_cmd(aug: &mut Augeas, cmd: &Command) -> Result<String, Error> {
        match *cmd {
            Command::Foreign(ref cmd) => aug.run(&cmd),
            Command::Reorder(ref from, before, ref to) => {
                let matches = aug.matches(&to)?;
                if matches.len() == 0 {
                    return Ok("".into())
                }
                let dest = if before {
                    matches[0].clone()
                } else {
                    matches.last().unwrap().clone()
                };
                for src in aug.matches(&from).unwrap() {
                    let mut marker = dest.clone();
                    marker.push_str("/../augtool-marker-node");
                    let label = aug.label(&src)?.unwrap();
                    aug.insert(&dest, "augtool-marker-node", before)?;
                    aug.mv(&src, &marker)?;
                    aug.rename(&marker, &label)?;
                }
                Ok("".into())
            }
        }
    }
}


fn main() {
    let mut aug = Augeas::new("/", "", AugFlag::None).unwrap();

    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(line) = rl.readline("augtool> ") {
        let cmdline = line.trim();
        if cmdline.len() == 0 { continue }
        rl.add_history_entry(cmdline);
        let cmd = augcmd::cmd(cmdline).unwrap();
        match augcmd::run_cmd(&mut aug, &cmd) {
            Ok(s) => { print!("{}", s); },
            Err(e) => { print!("{}", e); },
        }
    }
}
