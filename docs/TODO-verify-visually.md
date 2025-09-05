# Visual Verification Test Plan

This document outlines the steps to visually verify that all documentation files can be rendered correctly.

## Prerequisites

Before running the verification script, you will need to have the following installed on your system:

1.  **Rust:** The verification script is written in Rust. You can find installation instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
2.  **Firefox:** The script uses Firefox to render the documentation files. You can download it from the [Firefox website](https://www.mozilla.org/en-US/firefox/new/).
3.  **geckodriver:** This is the WebDriver for Firefox. You can download the latest version from the [geckodriver releases page](https://github.com/mozilla/geckodriver/releases). Make sure to place the `geckodriver` executable in a directory that is in your system's `PATH`.
4.  **npm:** Some of the slide decks have `npm` dependencies. You can install `npm` by installing [Node.js](https://nodejs.org/en/download/).

## Running the Verification Script

1.  **Install npm dependencies:**
    Some of the slide decks have their own `npm` dependencies. Run the following command from the root of the repository to install them:
    ```bash
    find . -name "package.json" -exec sh -c 'cd "$(dirname "$0")" && npm install' {} \;
    ```

2.  **Run the verification script:**
    The verification script is located in the `visual_verifier` directory. To run it, execute the following command from the root of the repository:
    ```bash
    cargo run --manifest-path visual_verifier/Cargo.toml
    ```
    The script will:
    - Start a local web server.
    - Launch a headless Firefox browser.
    - Navigate to each documentation file and take a screenshot.
    - Save the screenshots in a new `screenshots` directory inside the `visual_verifier` directory.

3.  **Review the screenshots:**
    After the script has finished, you will find a `screenshots` directory in the `visual_verifier` directory. Please review the screenshots to ensure that all documentation files are rendering correctly.

## Checklist

The following is a list of directories that will be visually verified by the script.

- [ ] `diagrams/bpmn-io`
- [ ] `diagrams/blockdiag`
- [ ] `diagrams/jointjs`
- [ ] `diagrams/nomnoml`
- [ ] `diagrams/plantuml`
- [ ] `diagrams/threejs`
- [ ] `diagrams/x6-antv`
- [ ] `diagrams/graphviz`
- [ ] `slides/deckjs`
- [ ] `slides/flowtimejs`
- [ ] `slides/impressjs`
- [ ] `slides/marp`
- [ ] `slides/mdx-deck`
- [ ] `slides/remark`
- [ ] `slides/revealjs`
- [ ] `slides/slidev`
- [ ] `slides/spectacle`
- [ ] `slides/webslides`
