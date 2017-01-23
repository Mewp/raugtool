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
                let dest = match before {
                    true => matches[0].clone(),
                    false => matches.last().unwrap().clone()
                };
                // First, store all the values
                let mut values: Vec<_> = aug.matches(&from)?.iter().map(|m| aug.get(m)).collect::<Result<_,_>>()?;
                // Next, replace all of them with a marker value,
                // because setm is the only thing we can do on multiple
                // nodes at once
                aug.setm(&from, None, "raugtool-marker-value")?;
                // Now for each marked node...
                for val in values {
                    // ...we mark it with a different value,
                    let marker_path = match before {
                        true => "//*[. = \"raugtool-marker-value\"][1]",
                        false => "//*[. = \"raugtool-marker-value\"][last()]"
                    };
                    aug.set(marker_path, "raugtool-marker-value-current")?;
                    // move it to the right place,
                    let src = "//*[. = \"raugtool-marker-value-current\"][1]";
                    let mut marker = dest.clone();
                    marker.push_str("/../augtool-marker-node");
                    let label = aug.label(src)?.unwrap();
                    aug.insert(&dest, "augtool-marker-node", before)?;
                    aug.mv(src, &marker)?;
                    // and finally, restore the old value (and name).
                    aug.set(&marker, val.as_ref().map(|v| v.as_str()))?;
                    aug.rename(&marker, &label)?;
                    // This way, the first node with the marker value is again the first one to
                    // move, and so we can move around things without worrying about stale matches.
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
