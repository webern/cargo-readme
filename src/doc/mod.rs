use std::io::Read;
use std::path::Path;

mod extractor;
mod modifier;
mod cargo_info;
mod template;

use self::extractor::DocExtractor;
use self::modifier::DocModifier;
pub use self::cargo_info::get_cargo_info;

/// Generates readme data from `source` file
pub fn generate_readme<T: Read>(project_root: &Path,
                                source: &mut T,
                                template: &mut Option<T>,
                                add_title: bool,
                                add_license: bool,
                                indent_headings: bool)
    -> Result<String, String> {

    let doc_iter = DocModifier::new(DocExtractor::new(source), indent_headings);
    let mut doc_data = Vec::new();
    for line in doc_iter {
        let line = line.map_err(|e| format!("{}", e))?;
        doc_data.push(line);
    }
    
    let mut readme = fold_doc_data(doc_data);

    let cargo = cargo_info::get_cargo_info(project_root)?;
    if add_license && cargo.package.license.is_none() {
        return Err("There is no license in Cargo.toml".to_owned());
    }

    let title = cargo.package.name.as_ref();
    let license = cargo.package.license.as_ref();

    match template.as_mut() {
        Some(template) => {
            let title = if add_title { Some(title) } else { None };
            let license = if add_license { Some(license.unwrap().as_ref()) } else { None };
            template::process_template(template, readme, title, license)
        }
        None => {
            if add_title {
                readme = template::prepend_title(readme, &title);
            }
            if add_license {
                readme = template::append_license(readme, &license.unwrap());
            }

            Ok(readme)
        }
    }
}

/// Transforms the Vec of lines into a single String
fn fold_doc_data(data: Vec<String>) -> String {
    if data.len() < 1 {
        String::new()
    } else if data.len() < 2 {
        data[0].to_owned()
    } else {
        data[1..].into_iter().fold(data[0].to_owned(), |acc, line| format!("{}\n{}", acc, line))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    const TEMPLATE_NO_CRATE_NO_LICENSE: &'static str = "{{readme}}";
    const TEMPLATE_CRATE_NO_LICENSE: &'static str = "# {{crate}}\n\n{{readme}}";
    const TEMPLATE_NO_CRATE_LICENSE: &'static str = "{{readme}}\n\nLicense: {{license}}";
    const TEMPLATE_CRATE_LICENSE: &'static str = "# {{crate}}\n\n{{readme}}\n\nLicense: {{license}}";

    #[test]
    fn fold_data_empty_input() {
        let input: Vec<String> = vec![];

        let result = super::fold_data(input);

        assert!(result.is_empty());
    }

    #[test]
    fn fold_data_single_line() {
        let line = "# single line";
        let input: Vec<String> = vec![line.to_owned()];

        let result = super::fold_data(input);

        assert_eq!(line, result);
    }

    #[test]
    fn fold_data_multiple_lines() {
        let input: Vec<String> = vec![
            "# first line".to_owned(),
            "second line".to_owned(),
            "third line".to_owned(),
        ];

        let result = super::fold_data(input);

        assert_eq!("# first line\nsecond line\nthird line", result);
    }

    macro_rules! test_process_template {
        ( $name:ident,
          $template:ident,
          input => $input:expr,
          license => $license:expr,
          add_crate_name => $with_crate:expr,
          add_license => $with_license:expr,
          expected => $expected:expr) =>
        {
            #[test]
            fn $name() {
                let input = $input;
                let mut template = Cursor::new($template.as_bytes());

                let crate_info = super::cargo_info::Cargo {
                    package: super::cargo_info::CargoPackage {
                        name: "my_crate".into(),
                        license: $license,
                    },
                    lib: None,
                    bin: None,
                };

                let result = super::template::process_template(&mut template,
                                            input.into(),
                                            $with_crate,
                                            $with_license).unwrap();
                assert_eq!($expected, result);
            }
        };

        ( $name:ident,
          $template:ident,
          input => $input:expr,
          license => $license:expr,
          add_crate_name => $with_crate:expr,
          add_license => $with_license:expr,
          panic => $panic:expr) =>
        {
            #[test]
            #[should_panic(expected = $panic)]
            fn $name() {
                let input = $input;
                let mut template = Cursor::new($template.as_bytes());

                let crate_info = super::cargo_info::Cargo {
                    package: super::cargo_info::CargoPackage {
                        name: "my_crate".into(),
                        license: $license,
                    },
                    lib: None,
                    bin: None
                };

                super::process_template(&mut template,
                                        input.into(),
                                        crate_info.clone(),
                                        $with_crate,
                                        $with_license).unwrap();
            }
        }
    }

    // TEMPLATE_NO_CRATE_NO_LICENSE
    test_process_template!(
        process_template_no_crate_no_license_with_license_prepend_crate_append_license,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => true,
        add_license => true,
        expected => "# my_crate\n\n# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_no_crate_no_license_without_license_prepend_crate_append_license,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => true,
        add_license => true,
        panic => "There is no license in Cargo.toml"
    );

    test_process_template!(
        process_template_no_crate_no_license_with_license_prepend_crate,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => true,
        add_license => false,
        expected => "# my_crate\n\n# documentation"
    );

    test_process_template!(
        process_template_no_crate_no_license_without_license_prepend_crate,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => true,
        add_license => false,
        expected => "# my_crate\n\n# documentation"
    );

    test_process_template!(
        process_template_no_crate_no_license_with_license_append_license,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => false,
        add_license => true,
        expected => "# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_no_crate_no_license_without_license_append_license,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => false,
        add_license => true,
        panic => "There is no license in Cargo.toml"
    );

    test_process_template!(
        process_template_no_crate_no_license_with_license,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => false,
        add_license => false,
        expected => "# documentation"
    );

    test_process_template!(
        process_template_no_crate_no_license_without_license,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => false,
        add_license => false,
        expected => "# documentation"
    );

    // TEMPLATE_CRATE_NO_LICENSE
    test_process_template!(
        process_template_crate_no_license_with_license_prepend_crate_append_license,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => true,
        add_license => true,
        expected => "# my_crate\n\n# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_crate_no_license_without_license_prepend_crate_append_license,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => true,
        add_license => true,
        panic => "There is no license in Cargo.toml"
    );

    test_process_template!(
        process_template_crate_no_license_with_license_prepend_crate,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => true,
        add_license => false,
        expected => "# my_crate\n\n# documentation"
    );

    test_process_template!(
        process_template_crate_no_license_without_license_prepend_crate,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => true,
        add_license => false,
        expected => "# my_crate\n\n# documentation"
    );

    test_process_template!(
        process_template_crate_no_license_with_license_append_license,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => false,
        add_license => true,
        expected => "# my_crate\n\n# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_crate_no_license_without_license_append_license,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => false,
        add_license => true,
        panic => "There is no license in Cargo.toml"
    );

    test_process_template!(
        process_template_crate_no_license_with_license,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => false,
        add_license => false,
        expected => "# my_crate\n\n# documentation"
    );

    test_process_template!(
        process_template_crate_no_license_without_license,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => false,
        add_license => false,
        expected => "# my_crate\n\n# documentation"
    );

    // TEMPLATE_NO_CRATE_LICENSE
    test_process_template!(
        process_template_no_crate_license_with_license_prepend_crate_append_license,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => true,
        add_license => true,
        expected => "# my_crate\n\n# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_no_crate_license_without_license_prepend_crate_append_license,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => true,
        add_license => true,
        panic => "`{{license}}` found in template but there is no license in Cargo.toml"
    );

    test_process_template!(
        process_template_no_crate_license_with_license_prepend_crate,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => true,
        add_license => false,
        expected => "# my_crate\n\n# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_no_crate_license_without_license_prepend_crate,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => true,
        add_license => false,
        panic => "`{{license}}` found in template but there is no license in Cargo.toml"
    );

    test_process_template!(
        process_template_no_crate_license_with_license_append_license,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => false,
        add_license => true,
        expected => "# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_no_crate_license_without_license_append_license,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => true,
        add_license => false,
        panic => "`{{license}}` found in template but there is no license in Cargo.toml"
    );

    test_process_template!(
        process_template_no_crate_license_with_license,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => false,
        add_license => false,
        expected => "# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_no_crate_license_without_license,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => false,
        add_license => false,
        panic => "`{{license}}` found in template but there is no license in Cargo.toml"
    );

    // TEMPLATE_CRATE_LICENSE
    test_process_template!(
        process_template_crate_license_with_license_prepend_crate_append_license,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => true,
        add_license => true,
        expected => "# my_crate\n\n# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_crate_license_without_license_prepend_crate_append_license,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => true,
        add_license => true,
        panic => "`{{license}}` found in template but there is no license in Cargo.toml"
    );

    test_process_template!(
        process_template_crate_license_with_license_prepend_crate,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => true,
        add_license => false,
        expected => "# my_crate\n\n# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_crate_license_without_license_prepend_crate,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => true,
        add_license => false,
        panic => "`{{license}}` found in template but there is no license in Cargo.toml"
    );

    test_process_template!(
        process_template_crate_license_with_license_append_license,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => false,
        add_license => true,
        expected => "# my_crate\n\n# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_crate_license_without_license_append_license,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => false,
        add_license => true,
        panic => "`{{license}}` found in template but there is no license in Cargo.toml"
    );

    test_process_template!(
        process_template_crate_license_with_license,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        license => Some("MIT".to_owned()),
        add_crate_name => false,
        add_license => false,
        expected => "# my_crate\n\n# documentation\n\nLicense: MIT"
    );

    test_process_template!(
        process_template_crate_license_without_license,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        license => None,
        add_crate_name => false,
        add_license => false,
        panic => "`{{license}}` found in template but there is no license in Cargo.toml"
    );
}
