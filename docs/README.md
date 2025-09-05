# Documentation Viewer

This directory contains a simple web UI to view all the documentation files.

## How to use

1.  **Navigate to the `docs` directory:**
    ```bash
    cd docs
    ```

2.  **Start a simple web server:**
    If you have Python 3, run:
    ```bash
    python3 -m http.server
    ```
    If you have Python 2, run:
    ```bash
    python -m SimpleHTTPServer
    ```

3.  **Open the viewer in your browser:**
    Open your web browser and navigate to [http://localhost:8000/viewer.html](http://localhost:8000/viewer.html).

You should see a list of all the files and folders in the `docs` directory. You can click on folders to expand them and click on files to view their content in the right-hand pane.

## Supported File Types

The viewer supports rendering for the following file types:
*   Markdown (`.md`)
*   PlantUML (`.puml`)
*   BPMN (`.bpmn`)
*   HTML (`.html`)

### Unsupported File Types
*   **blockdiag (`.diag`):** These files are not rendered in the viewer. `blockdiag` is a Python-based tool and there is no available client-side library to render these diagrams in the browser. The files will be displayed as plain text.
*   **MDX (`.mdx`):** These files are rendered as plain Markdown. JSX components are not supported and will be displayed as plain text.
