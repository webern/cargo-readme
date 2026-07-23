use std::collections::BTreeMap;

// https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata

use percent_encoding as pe;

const BADGE_BRANCH_DEFAULT: &str = "master";
const BADGE_SERVICE_DEFAULT: &str = "github";
const BADGE_WORKFLOW_DEFAULT: &str = "main";

type Attrs = BTreeMap<String, String>;

/// A badge key cargo-readme understands, plus which attributes it reads.
///
/// This is the single source of truth behind `cargo readme --list-badges`. The
/// `key`s here must stay in sync with the match arms in [`super::manifest::process_badges`];
/// a unit test in that module enforces it.
pub struct BadgeInfo {
    /// The `[badges]` table key, e.g. `"coveralls"`.
    pub key: &'static str,
    /// Whether crates.io documents this badge in the manifest reference.
    pub official: bool,
    /// Attributes that must be present, or badge rendering fails.
    pub required: &'static [&'static str],
    /// Attributes that are read when present, otherwise defaulted.
    pub optional: &'static [&'static str],
}

/// Every badge cargo-readme can render, in the order they appear in the output.
pub const SUPPORTED_BADGES: &[BadgeInfo] = &[
    BadgeInfo {
        key: "crates-io",
        official: false,
        required: &[],
        optional: &["crate"],
    },
    BadgeInfo {
        key: "appveyor",
        official: true,
        required: &["repository"],
        optional: &["branch", "service"],
    },
    BadgeInfo {
        key: "circle-ci",
        official: true,
        required: &["repository"],
        optional: &["branch", "service"],
    },
    BadgeInfo {
        key: "gitlab",
        official: true,
        required: &["repository"],
        optional: &["branch"],
    },
    BadgeInfo {
        key: "travis-ci",
        official: true,
        required: &["repository"],
        optional: &["branch"],
    },
    BadgeInfo {
        key: "github",
        official: false,
        required: &["repository"],
        optional: &["workflow"],
    },
    BadgeInfo {
        key: "codecov",
        official: true,
        required: &["repository"],
        optional: &["branch", "service"],
    },
    BadgeInfo {
        key: "coveralls",
        official: true,
        required: &["repository"],
        optional: &["branch", "service"],
    },
    BadgeInfo {
        key: "is-it-maintained-issue-resolution",
        official: true,
        required: &["repository"],
        optional: &[],
    },
    BadgeInfo {
        key: "is-it-maintained-open-issues",
        official: true,
        required: &["repository"],
        optional: &[],
    },
    BadgeInfo {
        key: "maintenance",
        official: true,
        required: &["status"],
        optional: &[],
    },
];

pub fn appveyor(attrs: Attrs) -> Result<String, String> {
    let repo = required(&attrs, "appveyor", "repository")?;
    let branch = attrs
        .get("branch")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_BRANCH_DEFAULT);
    let service = attrs
        .get("service")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_SERVICE_DEFAULT);

    Ok(format!(
        "[![Build Status](https://ci.appveyor.com/api/projects/status/{service}/{repo}?branch={branch}&svg=true)]\
        (https://ci.appveyor.com/project/{repo}/branch/{branch})",
        repo=repo, branch=branch, service=service
    ))
}

pub fn circle_ci(attrs: Attrs) -> Result<String, String> {
    let repo = required(&attrs, "circle-ci", "repository")?;
    let branch = attrs
        .get("branch")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_BRANCH_DEFAULT);
    let service = badge_service_short_name(
        attrs
            .get("service")
            .map(|i| i.as_ref())
            .unwrap_or(BADGE_SERVICE_DEFAULT),
    );

    Ok(format!(
        "[![Build Status](https://circleci.com/{service}/{repo}/tree/{branch}.svg?style=shield)]\
         (https://circleci.com/{service}/{repo}/tree/{branch})",
        repo = repo,
        service = service,
        branch = percent_encode(branch)
    ))
}

pub fn gitlab(attrs: Attrs) -> Result<String, String> {
    let repo = required(&attrs, "gitlab", "repository")?;
    let branch = attrs
        .get("branch")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_BRANCH_DEFAULT);

    Ok(format!(
        "[![Build Status](https://gitlab.com/{repo}/badges/{branch}/pipeline.svg)]\
         (https://gitlab.com/{repo}/commits/master)",
        repo = repo,
        branch = percent_encode(branch)
    ))
}

pub fn travis_ci(attrs: Attrs) -> Result<String, String> {
    let repo = required(&attrs, "travis-ci", "repository")?;
    let branch = attrs
        .get("branch")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_BRANCH_DEFAULT);

    Ok(format!(
        "[![Build Status](https://travis-ci.org/{repo}.svg?branch={branch})]\
         (https://travis-ci.org/{repo})",
        repo = repo,
        branch = percent_encode(branch)
    ))
}

pub fn github(attrs: Attrs) -> Result<String, String> {
    let repo = required(&attrs, "github", "repository")?;
    let workflow = attrs
        .get("workflow")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_WORKFLOW_DEFAULT);

    Ok(format!(
        "[![Workflow Status](https://github.com/{repo}/workflows/{workflow}/badge.svg)]\
         (https://github.com/{repo}/actions?query=workflow%3A%22{workflow_plus}%22)",
        repo = repo,
        workflow = percent_encode(workflow),
        workflow_plus = percent_encode(&str::replace(workflow, " ", "+"))
    ))
}

pub fn codecov(attrs: Attrs) -> Result<String, String> {
    let repo = required(&attrs, "codecov", "repository")?;
    let branch = attrs
        .get("branch")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_BRANCH_DEFAULT);
    let service = badge_service_short_name(
        attrs
            .get("service")
            .map(|i| i.as_ref())
            .unwrap_or(BADGE_SERVICE_DEFAULT),
    );

    Ok(format!(
        "[![Coverage Status](https://codecov.io/{service}/{repo}/branch/{branch}/graph/badge.svg)]\
         (https://codecov.io/{service}/{repo})",
        repo = repo,
        branch = percent_encode(branch),
        service = service
    ))
}

pub fn coveralls(attrs: Attrs) -> Result<String, String> {
    let repo = required(&attrs, "coveralls", "repository")?;
    let branch = attrs
        .get("branch")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_BRANCH_DEFAULT);
    let service = attrs
        .get("service")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_SERVICE_DEFAULT);

    Ok(format!(
        "[![Coverage Status](https://coveralls.io/repos/{service}/{repo}/badge.svg?branch={branch})]\
         (https://coveralls.io/{service}/{repo}?branch={branch})",
        repo = repo,
        branch = percent_encode(branch),
        service = service
    ))
}

pub fn is_it_maintained_issue_resolution(attrs: Attrs) -> Result<String, String> {
    let repo = required(&attrs, "is-it-maintained-issue-resolution", "repository")?;
    Ok(format!(
        "[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/{repo}.svg)]\
        (https://isitmaintained.com/project/{repo} \"Average time to resolve an issue\")",
        repo=repo
    ))
}

pub fn is_it_maintained_open_issues(attrs: Attrs) -> Result<String, String> {
    let repo = required(&attrs, "is-it-maintained-open-issues", "repository")?;
    Ok(format!(
        "[![Percentage of issues still open](https://isitmaintained.com/badge/open/{repo}.svg)]\
         (https://isitmaintained.com/project/{repo} \"Percentage of issues still open\")",
        repo = repo
    ))
}

pub fn maintenance(attrs: Attrs) -> Result<String, String> {
    let status = required(&attrs, "maintenance", "status")?;

    // https://github.com/rust-lang/crates.io/blob/5a08887d4b531e034d01386d3e5997514f3c8ee5/src/models/badge.rs#L82
    let status_with_color = match status {
        "actively-developed" => "actively--developed-brightgreen",
        "passively-maintained" => "passively--maintained-yellowgreen",
        "as-is" => "as--is-yellow",
        "none" => "maintenance-none-lightgrey", // color is a guess
        "experimental" => "experimental-blue",
        "looking-for-maintainer" => "looking--for--maintainer-darkblue", // color is a guess
        "deprecated" => "deprecated-red",
        _ => "unknown-black",
    };

    //example https://img.shields.io/badge/maintenance-experimental-blue.svg
    Ok(format!(
        "![Maintenance](https://img.shields.io/badge/maintenance-{status}.svg)",
        status = status_with_color
    ))
}

pub fn crates_io(attrs: Attrs, crate_name: &str) -> Result<String, String> {
    // Not part of the official `[badges]` set, but the crate name is already in
    // `Cargo.toml`, so `crate` defaults to the package name when omitted.
    let name = attrs.get("crate").map(|s| s.as_ref()).unwrap_or(crate_name);
    Ok(format!(
        "[![Crates.io](https://img.shields.io/crates/v/{name}.svg)](https://crates.io/crates/{name})",
        name = name
    ))
}

/// Look up an attribute that a badge cannot render without.
fn required<'a>(attrs: &'a Attrs, badge: &str, key: &str) -> Result<&'a str, String> {
    attrs
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("badge `{badge}` is missing required attribute `{key}`"))
}

fn percent_encode(input: &str) -> pe::PercentEncode<'_> {
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
