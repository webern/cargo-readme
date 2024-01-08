// https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata

use percent_encoding as pe;

const BADGE_SERVICE_DEFAULT: &str = "github";

pub fn appveyor(badge: cargo_toml::Badge) -> String {
    let repo = badge.repository;
    let branch = badge.branch;
    let service = badge
        .service
        .unwrap_or_else(|| BADGE_SERVICE_DEFAULT.to_owned());

    format!(
        "[![Build Status](https://ci.appveyor.com/api/projects/status/{service}/{repo}?branch={branch}&svg=true)]\
        (https://ci.appveyor.com/project/{repo}/branch/{branch})",
        repo=repo, branch=branch, service=service
    )
}

pub fn circle_ci(badge: cargo_toml::Badge) -> String {
    let repo = badge.repository;
    let branch = badge.branch;
    let service = badge_service_short_name(
        &badge
            .service
            .unwrap_or_else(|| BADGE_SERVICE_DEFAULT.to_owned()),
    );

    format!(
        "[![Build Status](https://circleci.com/{service}/{repo}/tree/{branch}.svg?style=shield)]\
         (https://circleci.com/{service}/{repo}/tree/{branch})",
        repo = repo,
        service = service,
        branch = percent_encode(&branch)
    )
}

pub fn gitlab(badge: cargo_toml::Badge) -> String {
    let repo = badge.repository;
    let branch = badge.branch;

    format!(
        "[![Build Status](https://gitlab.com/{repo}/badges/{branch}/pipeline.svg)]\
         (https://gitlab.com/{repo}/commits/master)",
        repo = repo,
        branch = percent_encode(&branch)
    )
}

pub fn travis_ci(badge: cargo_toml::Badge) -> String {
    let repo = badge.repository;
    let branch = badge.branch;

    format!(
        "[![Build Status](https://travis-ci.org/{repo}.svg?branch={branch})]\
         (https://travis-ci.org/{repo})",
        repo = repo,
        branch = percent_encode(&branch)
    )
}

#[allow(unused)]
pub fn github(badge: cargo_toml::Badge) -> String {
    let repo = badge.repository;
    // TODO: support workflow option.
    let workflow = "main"; // BADGE_WORKFLOW_DEFAULT

    format!(
        "[![Workflow Status](https://github.com/{repo}/workflows/{workflow}/badge.svg)]\
         (https://github.com/{repo}/actions?query=workflow%3A%22{workflow_plus}%22)",
        repo = repo,
        workflow = percent_encode(workflow),
        workflow_plus = percent_encode(&str::replace(workflow, " ", "+"))
    )
}

pub fn codecov(badge: cargo_toml::Badge) -> String {
    let repo = badge.repository;
    let branch = badge.branch;
    let service = badge_service_short_name(
        &badge
            .service
            .unwrap_or_else(|| BADGE_SERVICE_DEFAULT.to_owned()),
    );

    format!(
        "[![Coverage Status](https://codecov.io/{service}/{repo}/branch/{branch}/graph/badge.svg)]\
         (https://codecov.io/{service}/{repo})",
        repo = repo,
        branch = percent_encode(&branch),
        service = service
    )
}

pub fn coveralls(badge: cargo_toml::Badge) -> String {
    let repo = badge.repository;
    let branch = badge.branch;
    let service = badge
        .service
        .unwrap_or_else(|| BADGE_SERVICE_DEFAULT.to_owned());

    format!(
        "[![Coverage Status](https://coveralls.io/repos/{service}/{repo}/badge.svg?branch=branch)]\
         (https://coveralls.io/{service}/{repo}?branch={branch})",
        repo = repo,
        branch = percent_encode(&branch),
        service = service
    )
}

pub fn is_it_maintained_issue_resolution(badge: cargo_toml::Badge) -> String {
    let repo = badge.repository;
    format!(
        "[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/{repo}.svg)]\
        (https://isitmaintained.com/project/{repo} \"Average time to resolve an issue\")",
        repo=repo
    )
}

pub fn is_it_maintained_open_issues(badge: cargo_toml::Badge) -> String {
    let repo = badge.repository;
    format!(
        "[![Percentage of issues still open](https://isitmaintained.com/badge/open/{repo}.svg)]\
         (https://isitmaintained.com/project/{repo} \"Percentage of issues still open\")",
        repo = repo
    )
}

#[allow(unused)]
pub fn maintenance(maintenance: cargo_toml::Maintenance) -> Option<String> {
    let status = maintenance.status;

    // https://github.com/rust-lang/crates.io/blob/5a08887d4b531e034d01386d3e5997514f3c8ee5/src/models/badge.rs#L82
    match status {
        cargo_toml::MaintenanceStatus::ActivelyDeveloped => Some("activly--developed-brightgreen"),
        cargo_toml::MaintenanceStatus::PassivelyMaintained => {
            Some("passively--maintained-yellowgreen")
        }
        cargo_toml::MaintenanceStatus::AsIs => Some("as--is-yellow"),
        cargo_toml::MaintenanceStatus::Experimental => Some("experimental-blue"),
        cargo_toml::MaintenanceStatus::LookingForMaintainer => {
            Some("looking--for--maintainer-darkblue")
        } // color is a guess
        cargo_toml::MaintenanceStatus::Deprecated => Some("deprecated-red"),
        cargo_toml::MaintenanceStatus::None => None,
    }
    .map(|status| format!("![Maintenance](https://img.shields.io/badge/maintenance-{status}.svg)",))
}

fn percent_encode(input: &str) -> pe::PercentEncode {
    pe::utf8_percent_encode(input, pe::NON_ALPHANUMERIC)
}

fn badge_service_short_name(service: &str) -> &'static str {
    match service {
        "github" => "gh",
        "bitbucket" => "bb",
        "gitlab" => "gl",
        _ => "gh",
    }
}
