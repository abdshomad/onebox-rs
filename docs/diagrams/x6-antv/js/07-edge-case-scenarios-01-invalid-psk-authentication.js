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
            attrs: { body: { fill: '#ccc', stroke: '#ccc' } }
        },
        {
            id: 'server-life',
            shape: 'rect',
            x: 559,
            y: 100,
            width: 2,
            height: 400,
            attrs: { body: { fill: '#ccc', stroke: '#ccc' } }
        },
        // Note
        {
            id: 'note',
            shape: 'rect',
            x: 250,
            y: 350,
            width: 250,
            height: 50,
            label: 'Server rejects connection',
            attrs: {
                body: {
                    fill: '#fffbe6',
                    stroke: '#ffe58f',
                    rx: 5,
                    ry: 5,
                }
            }
        }
    ],
    edges: [
        {
            source: { cell: 'client-life', y: 150 },
            target: { cell: 'server-life', y: 150 },
            label: 'AuthRequest(Invalid PSK)',
        },
        {
            source: { cell: 'server-life', y: 200 },
            target: { cell: 'server-life', y: 230 },
            vertices: [{ x: 600, y: 200 }, { x: 600, y: 230 }],
            label: 'Verify PSK',
        },
        {
            source: { cell: 'server-life', y: 280 },
            target: { cell: 'client-life', y: 280 },
            label: 'AuthResponse(Failure)',
            attrs: {
                line: {
                    strokeDasharray: '5 5',
                }
            }
        },
    ],
};
