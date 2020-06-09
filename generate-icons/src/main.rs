use std::collections::HashSet;
use once_cell::sync::Lazy;
use regex::Regex;

use druid_icon_generator::library::IconLibrary;
use druid_icon_generator::generator::Generator;

const SOURCE: &str = "../material-design-icons";
const DESTINATION: &str = "../src/lib.rs";

static ICON_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^ic_(.*)_(\d+)px\.svg$").unwrap());


fn main() {
    let mut visited_icons = HashSet::<String>::new();

    let icons = IconLibrary::new(SOURCE).iter()
        .filter(|icon_file| {
            icon_file.module.ends_with("svg/production") &&
                icon_file.name.ends_with(".svg")
        })
        .filter_map(|icon_file| {
            ICON_REGEX.captures(&icon_file.name.clone()).and_then(|captures| {
                let name = captures.get(1)?.as_str();
                let size = captures.get(2)?.as_str().parse::<f64>().ok()?;

                if !visited_icons.contains(name) {
                    visited_icons.insert(name.to_string());

                    println!("+++ {}", icon_file.path.display());

                    let name = if name.starts_with("3") {
                        format!("three_{}", &name[1..])
                    } else {
                        name.to_string()
                    };

                    let module = icon_file.module.iter().next().unwrap()
                        .to_os_string().into_string().unwrap();

                    let icon_file = icon_file
                        .with_module(module)
                        .with_name(name);

                    let icon_data = icon_file.load().ok()?;

                    assert_eq!(icon_data.size, kurbo::Size::new(size, size));

                    Some((icon_file, icon_data))
                }
                else {
                    println!("--- {}", icon_file.path.display());
                    None
                }
            })
        });

    Generator::new(DESTINATION)
        .generate(icons).unwrap()
}
