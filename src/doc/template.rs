use std::io::Read;

/// Renders the template
///
/// This is not a real template engine, it just processes a few substitutions.
pub fn process_template<T: Read>(template: &mut T,
                             mut readme: String,
                             title: Option<&str>,
                             license: Option<&str>)
                             -> Result<String, String> {

    let mut template = try!(get_template(template));
    template = template.trim_right_matches("\n").to_owned();

    if !template.contains("{{readme}}") {
        return Err("Missing `{{readme}}` in template".to_owned());
    }

    if template.contains("{{license}}") && license.is_none() {
        return Err("`{{license}}` was found in template but no license was provided".to_owned());
    }

    if let Some(title) = title {
        if template.contains("{{crate}}") {
            template = template.replace("{{crate}}", &title);
        } else {
            readme = prepend_title(readme, &title);
        }
    }

    if let Some(license) = license {
        if template.contains("{{license}}") {
            template = template.replace("{{license}}", &license);
        } else {
            readme = append_license(readme, &license);
        }
    }

    let result = template.replace("{{readme}}", &readme);
    Ok(result)
}

fn get_template<T: Read>(template: &mut T) -> Result<String, String> {
    let mut template_string = String::new();
    match template.read_to_string(&mut template_string) {
        Err(e) => return Err(format!("Error: {}", e)),
        _ => {}
    }

    Ok(template_string)
}

pub fn prepend_title(readme: String, crate_name: &str) -> String {
    format!("# {}\n\n", crate_name) + readme.as_ref()
}

pub fn append_license(readme: String, license: &str) -> String {
    readme + &format!("\n\nLicense: {}", &license)
}