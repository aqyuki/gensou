#[cfg(test)]
mod tests {
    use crate::manifest::{
        DotfileEntry, DotfileMethod, ExternalEntry, ExternalMethod, ExtractOptions,
    };

    use std::path::PathBuf;

    #[test]
    fn test_deserialize_manifest_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let manifest_path = PathBuf::from("tests/data/manifest.toml");
        let manifest = crate::manifest::load_manifest(manifest_path)?;

        // Assert dotfiles section
        assert!(manifest.dotfiles.is_some());
        let dotfiles = manifest.dotfiles.unwrap();
        assert_eq!(dotfiles.len(), 3);
        assert_eq!(
            dotfiles.get("bashrc").unwrap(),
            &DotfileEntry {
                source: String::from("~/.config/bash/bashrc"),
                target: String::from("~/.bashrc"),
                method: DotfileMethod::Symlink,
                backup: true
            }
        );
        assert_eq!(
            dotfiles.get("zshrc").unwrap(),
            &DotfileEntry {
                source: String::from("~/.config/zsh/zshrc"),
                target: String::from("~/.zshrc"),
                method: DotfileMethod::Symlink,
                backup: false
            }
        );
        assert_eq!(
            dotfiles.get("vimrc").unwrap(),
            &DotfileEntry {
                source: String::from("dotfiles/vimrc"),
                target: String::from("~/.vimrc"),
                method: DotfileMethod::Copy,
                backup: false
            }
        );

        // Assert external section
        assert!(manifest.external.is_some());
        let external = manifest.external.unwrap();
        assert_eq!(external.len(), 3);
        assert_eq!(
            external.get("fzf").unwrap(),
            &ExternalEntry {
                method: ExternalMethod::Git,
                source: String::from("https://github.com/junegunn/fzf.git"),
                target: String::from("~/.fzf"),
                extract: None,
                checksum: None,
                timeout: None,
                branch: None,
                depth: Some(1),
                recurse_submodules: None
            }
        );
        assert_eq!(
            external.get("ripgrep").unwrap(),
            &ExternalEntry {
                method: ExternalMethod::Download,
                source: String::from(
                    "https://github.com/BurntSushi/ripgrep/releases/download/13.0.0/ripgrep-x86_64-linux.tar.gz"
                ),
                target: String::from("~/.local/bin/ripgrep.tar.gz"),
                extract: Some(ExtractOptions {
                    format: String::from("tar.gz"),
                    strip_components: Some(1)
                }),
                checksum: Some(String::from("...")),
                timeout: Some(30),
                branch: None,
                depth: None,
                recurse_submodules: None
            }
        );
        assert_eq!(
            external.get("zsh_plugins").unwrap(),
            &ExternalEntry {
                method: ExternalMethod::Git,
                source: String::from("https://github.com/zsh-users/zsh-plugins.git"),
                target: String::from("~/.zsh/plugins"),
                extract: None,
                checksum: None,
                timeout: None,
                branch: Some(String::from("master")),
                depth: None,
                recurse_submodules: Some(true)
            }
        );

        // Assert packages section
        assert!(manifest.packages.is_some());
        let packages = manifest.packages.unwrap();
        assert_eq!(packages.len(), 3);
        assert_eq!(
            packages.get("pacman").unwrap(),
            &vec![String::from("fzf"), String::from("ripgrep")]
        );
        assert_eq!(packages.get("apt").unwrap(), &vec![String::from("zsh")]);
        assert_eq!(packages.get("brew").unwrap(), &Vec::<String>::new());

        // Assert options section
        assert!(manifest.options.is_some());
        let options = manifest.options.unwrap();
        assert!(!options.overwrite);
        assert!(options.backup);

        Ok(())
    }
}
