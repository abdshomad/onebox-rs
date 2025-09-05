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
        // Loop block
        {
            id: 'loop-block',
            shape: 'rect',
            x: 50,
            y: 150,
            width: 650,
            height: 200,
            label: 'loop',
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
        },
        {
            id: 'loop-text',
            shape: 'text',
            x: 375,
            y: 170,
            width: 150,
            height: 20,
            label: 'For each WAN link',
        }
    ],
    edges: [
        {
            source: { cell: 'client-life', y: 220 },
            target: { cell: 'server-life', y: 220 },
            label: 'Health Probe',
        },
        {
            source: { cell: 'server-life', y: 280 },
            target: { cell: 'client-life', y: 280 },
            label: 'Probe ACK',
            attrs: {
                line: {
                    strokeDasharray: '5 5',
                }
            }
        },
    ],
};
