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
            height: 300,
            attrs: { body: { fill: '#ccc', stroke: '#ccc' } }
        },
        {
            id: 'server-life',
            shape: 'rect',
            x: 559,
            y: 100,
            width: 2,
            height: 300,
            attrs: { body: { fill: '#ccc', stroke: '#ccc' } }
        },
        // Note
        {
            id: 'note',
            shape: 'rect',
            x: 485,
            y: 280,
            width: 150,
            height: 50,
            label: 'Silently drop packet',
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
            label: 'Malformed Packet',
        },
        {
            source: { cell: 'server-life', y: 200 },
            target: { cell: 'server-life', y: 230 },
            vertices: [{ x: 600, y: 200 }, { x: 600, y: 230 }],
            label: 'Attempt to Decrypt/Parse',
        },
    ],
};
