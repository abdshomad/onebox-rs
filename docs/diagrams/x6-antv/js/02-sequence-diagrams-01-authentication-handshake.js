window.currentDiagram = {
    nodes: [
        // Participants
        {
            id: 'client',
            shape: 'rect',
            x: 100,
            y: 50,
            width: 120,
            height: 50,
            label: 'onebox-client',
        },
        {
            id: 'server',
            shape: 'rect',
            x: 500,
            y: 50,
            width: 120,
            height: 50,
            label: 'onebox-server',
        },
        // Lifelines
        {
            id: 'client-life',
            shape: 'rect',
            x: 159,
            y: 100,
            width: 2,
            height: 400,
            attrs: {
                body: {
                    fill: '#ccc',
                    stroke: '#ccc',
                }
            }
        },
        {
            id: 'server-life',
            shape: 'rect',
            x: 559,
            y: 100,
            width: 2,
            height: 400,
            attrs: {
                body: {
                    fill: '#ccc',
                    stroke: '#ccc',
                }
            }
        },
        // Alt block
        {
            id: 'alt-block',
            shape: 'rect',
            x: 50,
            y: 250,
            width: 650,
            height: 200,
            label: 'alt',
            attrs: {
                body: {
                    fill: 'none',
                    stroke: '#000',
                    strokeWidth: 1,
                },
                label: {
                    refX: 5,
                    refY: 5,
                    textAnchor: 'start',
                    textVerticalAnchor: 'top',
                }
            }
        }
    ],
    edges: [
        // Messages
        {
            source: { cell: 'client-life', y: 150 },
            target: { cell: 'server-life', y: 150 },
            label: 'AuthRequest(PSK)',
        },
        {
            source: { cell: 'server-life', y: 200 },
            target: { cell: 'server-life', y: 230 },
            vertices: [{ x: 600, y: 200 }, { x: 600, y: 230 }],
            label: 'Verify PSK',
        },
        // Alt-Success
        {
            source: { cell: 'server-life', y: 300 },
            target: { cell: 'client-life', y: 300 },
            label: 'AuthResponse(Success)',
            attrs: {
                line: {
                    strokeDasharray: '5 5',
                }
            }
        },
        // Dashed line for else
        {
            source: { x: 50, y: 350 },
            target: { x: 700, y: 350 },
            attrs: {
                line: {
                    stroke: '#000',
                    strokeWidth: 1,
                    strokeDasharray: '5 5',
                }
            },
            label: 'else Auth Failed',
        },
        // Alt-Failure
        {
            source: { cell: 'server-life', y: 400 },
            target: { cell: 'client-life', y: 400 },
            label: 'AuthResponse(Failure)',
            attrs: {
                line: {
                    strokeDasharray: '5 5',
                }
            }
        },
    ],
};
