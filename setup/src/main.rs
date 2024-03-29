use std::{process::Command, path::PathBuf};
use anyhow::Result;

fn main() -> Result<()> {

    let home = std::env::var("HOME")?;

    let mut src = std::env::current_dir()?;
    src.push("../init.lua");
    let mut dest = PathBuf::from(home.clone());
    dest.push(".config/nvim"); 

    Command::new("ln")
        .arg("-sf")
        .arg(src)
        .arg(&dest)
        .output()?;

    let mut src = std::env::current_dir()?;
    src.push("../lua");

    Command::new("ln")
        .arg("-sf")
        .arg(src)
        .arg(&dest)
        .output()?;

    let mut src = std::env::current_dir()?;
    src.push("../.tmux.conf");
    let mut dest = PathBuf::from(home);

    Command::new("ln")
        .arg("-sf")
        .arg(src)
        .arg(&dest)
        .output()?;
        
    Ok(())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn files_exist() -> Result<()> {
        let home = std::env::var("HOME")?;

        let mut dest = PathBuf::from(home);
        dest.push(".config/nvim"); 
        eprintln!("{:?}", dest);
        assert!(&dest.exists());

        let mut src = std::env::current_dir()?;
        src.push("../init.lua");
        eprintln!("{:?}", src);
        assert!(&src.exists());

        let mut src = std::env::current_dir()?;
        src.push("../lua");
        eprintln!("{:?}", src);
        assert!(&src.exists());

        let mut src = std::env::current_dir()?;
        src.push("../.tmux.conf");
        eprintln!("{:?}", src);
        assert!(&src.exists());

        Ok(())
    }
}
