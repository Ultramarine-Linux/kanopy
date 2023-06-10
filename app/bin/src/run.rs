use color_eyre::eyre::Result;
use std::process::Command;

pub fn start_install() -> Result<()> {
    let mut binding = Command::new("kubeadm").arg("init").spawn()?;
    binding.wait()?;

    Ok(())
}
