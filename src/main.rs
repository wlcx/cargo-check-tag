use cargo_metadata::MetadataCommand;
use std::env;

#[derive(Debug)]
enum Error {
    CantFindTag,
    CurrentRefNotTag(String),
    DoesntMatch {
        expected: String,
        from: String,
        got: String,
    },
    NoRootPackage,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::CantFindTag => "Can't find git tag from environment".into(),
            Self::CurrentRefNotTag(r) => format!("The current ref is not a tag: {r}"),
            Self::DoesntMatch {
                expected,
                from,
                got,
            } => format!(
                "The current tag `{got}` (via {from}) doesn't match the version in Cargo.toml `{expected}`"
            ),
            Self::NoRootPackage => "No root cargo package found".into(),
        };
        write!(f, "{msg}")
    }
}

fn main() -> Result<(), Error> {
    let metadata = MetadataCommand::new().exec().unwrap();
    let metaver = metadata
        .root_package()
        .ok_or(Error::NoRootPackage)?
        .version
        .to_string();
    let _ref = env::var("GITHUB_REF").map_err(|_| Error::CantFindTag)?;
    let tag = match _ref.strip_prefix("refs/tags/") {
        Some(t) => t,
        None => return Err(Error::CurrentRefNotTag(_ref)),
    };
    // Remove leading v if one exists
    let tag = tag.strip_prefix("v").unwrap_or(tag);
    if tag != metaver {
        Err(Error::DoesntMatch {
            expected: tag.into(),
            from: "GITHUB_REF".into(),
            got: metaver,
        })
    } else {
        Ok(())
    }
}
