mod find;

use heck::ShoutySnakeCase;
use std::{fs, io::Write as IoWrite, path::PathBuf};

use crate::find::find_icons;
use generate_icons::{Icon, IconSize, Implement};
use kurbo::Size;

fn main() {
    let mut out = fs::File::create("icons.rs").unwrap();
    for icon in find_icons() {
        writeln!(out, "{}", Implement::new(&icon)).unwrap();
    }
}

#[derive(Debug)]
struct MaterialIcon {
    category: String,
    prefix: String,
    size: kurbo::Size,
}

impl Icon for MaterialIcon {
    fn path(&self) -> PathBuf {
        format!(
            "../material-design-icons/{}/svg/production/ic_{}_{}px.svg",
            self.category,
            self.prefix,
            IconSize::new(self.size)
        )
        .into()
    }

    fn const_name(&self) -> String {
        let name = self.prefix.to_shouty_snake_case();
        if name.starts_with("3") {
            format!("THREE_{}", &name[1..])
        } else {
            name
        }
    }

    fn size(&self) -> Size {
        self.size
    }
}
