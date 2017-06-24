use cargo_info::Cargo;

/// Renders the template
///
/// This is not a real template engine, it just processes a few substitutions.
pub fn render(
    template: Option<String>,
    mut readme: String,
    cargo: Cargo,
    add_title: bool,
    add_license: bool,
) -> Result<String, String> {
    let title = cargo.package.name.as_ref();
    let license = cargo.package.license.as_ref();

    match template {
        Some(template) => {
            if template.contains("{{license}}") && !add_license {
                return Err(
                    "`{{license}}` was found in template but should not be rendered".to_owned(),
                );
            }

            if template.contains("{{crate}}") && !add_title {
                return Err(
                    "`{{crate}}` was found in template but title should not be rendered"
                        .to_owned(),
                );
            }

            let title = if add_title { Some(title) } else { None };
            let license = if add_license {
                Some(license.unwrap().as_ref())
            } else {
                None
            };
            process_template(template, readme, title, license)
        }
        None => {
            if add_title {
                readme = prepend_title(readme, &title);
            }
            if add_license {
                readme = append_license(readme, &license.unwrap());
            }

            Ok(readme)
        }
    }
}

/// Process the substitutions of the template
///
/// Available variable:
/// - `{{readme}}` documentation extracted from the rust docs
/// - `{{crate}}` crate name defined in `Cargo.toml`
/// - `{{license}}` license defined in `Cargo.toml`
fn process_template(
    mut template: String,
    readme: String,
    title: Option<&str>,
    license: Option<&str>,
) -> Result<String, String> {

    template = template.trim_right_matches("\n").to_owned();

    if !template.contains("{{readme}}") {
        return Err("Missing `{{readme}}` in template".to_owned());
    }

    if template.contains("{{license}}") && license.is_none() {
        return Err(
            "`{{license}}` was found in template but no license was provided".to_owned(),
        );
    }

    if template.contains("{{crate}}") && title.is_none() {
        return Err(
            "`{{crate}}` was found in template but no crate name was provided".to_owned(),
        );
    }

    if let Some(title) = title {
        if template.contains("{{crate}}") {
            template = template.replace("{{crate}}", &title);
        }
    }

    if let Some(license) = license {
        if template.contains("{{license}}") {
            template = template.replace("{{license}}", &license);
        }
    }

    let result = template.replace("{{readme}}", &readme);
    Ok(result)
}

/// Prepend title (crate name) to output string
fn prepend_title(readme: String, crate_name: &str) -> String {
    let title = format!("# {}", crate_name);
    if !readme.trim().is_empty() {
        format!("{}\n\n{}", title, readme)
    } else {
        title
    }
}

/// Append license to output string
fn append_license(readme: String, license: &str) -> String {
    let license = format!("License: {}", license);
    if !readme.trim().is_empty() {
        format!("{}\n\n{}", readme, license)
    } else {
        license
    }
}

#[cfg(test)]
mod tests {
    const CRATE_NAME: &str = "my_crate";
    const LICENSE: &str = "MPL";

    const TEMPLATE_NO_CRATE_NO_LICENSE: &str = "{{readme}}";
    const TEMPLATE_CRATE_NO_LICENSE: &str = "# {{crate}}\n\n{{readme}}";
    const TEMPLATE_NO_CRATE_LICENSE: &str = "{{readme}}\n\nLicense: {{license}}";
    const TEMPLATE_CRATE_LICENSE: &str = "# {{crate}}\n\n{{readme}}\n\nLicense: {{license}}";

    macro_rules! test_process_template {
        ( $name:ident,
          $template:ident,
          input => $input:expr,
          with_title => $with_title:expr,
          with_license => $with_license:expr,
          expected => $expected:expr) =>
        {
            #[test]
            fn $name() {
                let input = $input;
                let title = if $with_title { Some(CRATE_NAME) } else { None };
                let license = if $with_license { Some(LICENSE) } else { None };

                let result = super::process_template(
                    $template.to_owned(), input.into(), title, license
                ).unwrap();

                assert_eq!($expected, result);
            }
        };

        ( $name:ident,
          $template:ident,
          input => $input:expr,
          with_title => $with_title:expr,
          with_license => $with_license:expr,
          panic => $panic:expr) =>
        {
            #[test]
            #[should_panic(expected = $panic)]
            fn $name() {
                let input = $input;
                let title = if $with_title { Some(CRATE_NAME) } else { None };
                let license = if $with_license { Some(LICENSE) } else { None };

                super::process_template(
                    $template.to_owned(), input.into(), title, license
                ).unwrap();
            }
        }
    }

    // TEMPLATE_NO_CRATE_NO_LICENSE

    // with title with license
    test_process_template!(
        process_template_no_crate_no_license_with_title_with_license,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        with_title => true,
        with_license => true,
        expected => "# documentation"
    );

    // with title without license
    test_process_template!(
        process_template_no_crate_no_license_with_title_without_license,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        with_title => true,
        with_license => false,
        expected => "# documentation"
    );

    // without title with license
    test_process_template!(
        process_template_no_crate_no_license_without_title_with_license,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        with_title => false,
        with_license => true,
        expected => "# documentation"
    );

    // without title without license
    test_process_template!(
        process_template_no_crate_no_license_without_title_without_license,
        TEMPLATE_NO_CRATE_NO_LICENSE,
        input => "# documentation",
        with_title => false,
        with_license => false,
        expected => "# documentation"
    );

    // TEMPLATE_CRATE_NO_LICENSE

    // with title with license
    test_process_template!(
        process_template_crate_no_license_with_title_with_license,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        with_title => true,
        with_license => true,
        expected => "# my_crate\n\n# documentation"
    );

    // with title without license
    test_process_template!(
        process_template_crate_no_license_with_title_without_license,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        with_title => true,
        with_license => false,
        expected => "# my_crate\n\n# documentation"
    );

    // without title with license
    test_process_template!(
        process_template_crate_no_license_without_title_with_license,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        with_title => false,
        with_license => true,
        panic => "`{{crate}}` was found in template but no crate name was provided"
    );

    // without title without license
    test_process_template!(
        process_template_crate_no_license_without_title_without_license,
        TEMPLATE_CRATE_NO_LICENSE,
        input => "# documentation",
        with_title => false,
        with_license => false,
        panic => "`{{crate}}` was found in template but no crate name was provided"
    );

    // TEMPLATE_NO_CRATE_LICENSE

    // with title with license
    test_process_template!(
        process_template_no_crate_license_with_title_with_license,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        with_title => true,
        with_license => true,
        expected => "# documentation\n\nLicense: MPL"
    );

    // with title without license
    test_process_template!(
        process_template_no_crate_license_with_title_without_license,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        with_title => true,
        with_license => false,
        panic => "`{{license}}` was found in template but no license was provided"
    );

    // without title with license
    test_process_template!(
        process_template_no_crate_license_without_title_with_license,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        with_title => false,
        with_license => true,
        expected => "# documentation\n\nLicense: MPL"
    );

    // without title without license
    test_process_template!(
        process_template_no_crate_license_without_title_without_license,
        TEMPLATE_NO_CRATE_LICENSE,
        input => "# documentation",
        with_title => false,
        with_license => false,
        panic => "`{{license}}` was found in template but no license was provided"
    );

    // TEMPLATE_CRATE_LICENSE

    // with title with license
    test_process_template!(
        process_template_crate_license_with_title_with_license,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        with_title => true,
        with_license => true,
        expected => "# my_crate\n\n# documentation\n\nLicense: MPL"
    );

    // with title without license
    test_process_template!(
        process_template_crate_license_with_title_without_license,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        with_title => true,
        with_license => false,
        panic => "`{{license}}` was found in template but no license was provided"
    );

    // without title with license
    test_process_template!(
        process_template_crate_license_without_title_with_license,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        with_title => false,
        with_license => true,
        panic => "`{{crate}}` was found in template but no crate name was provided"
    );

    // without title with license
    test_process_template!(
        process_template_crate_license_without_title_witout_license,
        TEMPLATE_CRATE_LICENSE,
        input => "# documentation",
        with_title => false,
        with_license => false,
        panic => "`{{license}}` was found in template but no license was provided"
    );
}
