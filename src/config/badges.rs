use std::collections::BTreeMap;

// https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata

use percent_encoding as pe;

const BADGE_BRANCH_DEFAULT: &str = "master";
const BADGE_SERVICE_DEFAULT: &str = "github";
const BADGE_WORKFLOW_DEFAULT: &str = "main";

type Attrs = BTreeMap<String, String>;

pub fn appveyor(attrs: Attrs) -> String {
    let repo = &attrs["repository"];
    let branch = attrs
        .get("branch")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_BRANCH_DEFAULT);
    let service = attrs
        .get("service")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_SERVICE_DEFAULT);

    format!(
        "[![Build Status](https://ci.appveyor.com/api/projects/status/{service}/{repo}?branch={branch}&svg=true)]\
        (https://ci.appveyor.com/project/{repo}/branch/{branch})",
        repo=repo, branch=branch, service=service
    )
}

pub fn circle_ci(attrs: Attrs) -> String {
    let repo = &attrs["repository"];
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

    format!(
        "[![Build Status](https://circleci.com/{service}/{repo}/tree/{branch}.svg?style=shield)]\
         (https://circleci.com/{service}/{repo}/cargo-readme/tree/{branch})",
        repo = repo,
        service = service,
        branch = percent_encode(branch)
    )
}

pub fn gitlab(attrs: Attrs) -> String {
    let repo = &attrs["repository"];
    let branch = attrs
        .get("branch")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_BRANCH_DEFAULT);

    format!(
        "[![Build Status](https://gitlab.com/{repo}/badges/{branch}/pipeline.svg)]\
         (https://gitlab.com/{repo}/commits/master)",
        repo = repo,
        branch = percent_encode(branch)
    )
}

pub fn travis_ci(attrs: Attrs) -> String {
    let repo = &attrs["repository"];
    let branch = attrs
        .get("branch")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_BRANCH_DEFAULT);

    format!(
        "[![Build Status](https://travis-ci.org/{repo}.svg?branch={branch})]\
         (https://travis-ci.org/{repo})",
        repo = repo,
        branch = percent_encode(branch)
    )
}

pub fn github(attrs: Attrs) -> String {
    let repo = &attrs["repository"];
    let workflow = attrs
        .get("workflow")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_WORKFLOW_DEFAULT);

    format!(
        "[![Workflow Status](https://github.com/{repo}/workflows/{workflow}/badge.svg)]\
         (https://github.com/{repo}/actions?query=workflow%3A%22{workflow_plus}%22)",
        repo = repo,
        workflow = percent_encode(workflow),
        workflow_plus = percent_encode(&str::replace(workflow, " ", "+"))
    )
}

pub fn codecov(attrs: Attrs) -> String {
    let repo = &attrs["repository"];
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

    format!(
        "[![Coverage Status](https://codecov.io/{service}/{repo}/branch/{branch}/graph/badge.svg)]\
         (https://codecov.io/{service}/{repo})",
        repo = repo,
        branch = percent_encode(branch),
        service = service
    )
}

pub fn coveralls(attrs: Attrs) -> String {
    let repo = &attrs["repository"];
    let branch = attrs
        .get("branch")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_BRANCH_DEFAULT);
    let service = attrs
        .get("service")
        .map(|i| i.as_ref())
        .unwrap_or(BADGE_SERVICE_DEFAULT);

    format!(
        "[![Coverage Status](https://coveralls.io/repos/{service}/{repo}/badge.svg?branch=branch)]\
         (https://coveralls.io/{service}/{repo}?branch={branch})",
        repo = repo,
        branch = percent_encode(branch),
        service = service
    )
}

pub fn is_it_maintained_issue_resolution(attrs: Attrs) -> String {
    let repo = &attrs["repository"];
    format!(
        "[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/{repo}.svg)]\
        (https://isitmaintained.com/project/{repo} \"Average time to resolve an issue\")",
        repo=repo
    )
}

pub fn is_it_maintained_open_issues(attrs: Attrs) -> String {
    let repo = &attrs["repository"];
    format!(
        "[![Percentage of issues still open](https://isitmaintained.com/badge/open/{repo}.svg)]\
         (https://isitmaintained.com/project/{repo} \"Percentage of issues still open\")",
        repo = repo
    )
}

pub fn maintenance(attrs: Attrs) -> String {
    let status = &attrs["status"];
    //example https://img.shields.io/badge/maintenance-experimental-blue.svg
    format!(
        "![Maintenance](https://img.shields.io/badge/maintenance-{status}-blue.svg)",
        status = status
    )
}

fn percent_encode(input: &str) -> pe::PercentEncode<pe::PATH_SEGMENT_ENCODE_SET> {
    pe::utf8_percent_encode(input, pe::PATH_SEGMENT_ENCODE_SET)
}

fn badge_service_short_name(service: &str) -> &'static str {
    match service {
        "github" => "gh",
        "bitbucket" => "bb",
        "gitlab" => "gl",
        _ => "gh",
    }
}
