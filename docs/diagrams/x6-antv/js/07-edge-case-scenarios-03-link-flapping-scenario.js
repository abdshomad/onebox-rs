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
            height: 500,
            attrs: { body: { fill: '#ccc', stroke: '#ccc' } }
        },
        {
            id: 'server-life',
            shape: 'rect',
            x: 559,
            y: 100,
            width: 2,
            height: 500,
            attrs: { body: { fill: '#ccc', stroke: '#ccc' } }
        },
        // Loop block
        {
            id: 'loop-block',
            shape: 'rect',
            x: 50,
            y: 150,
            width: 650,
            height: 400,
            label: 'loop',
            attrs: {
                body: { fill: 'none', stroke: '#000', strokeWidth: 1 },
                label: { refX: 5, refY: 5, textAnchor: 'start', textVerticalAnchor: 'top' }
            }
        },
        {
            id: 'loop-text',
            shape: 'text',
            x: 375,
            y: 170,
            width: 150,
            height: 20,
            label: 'Link is flapping',
        },
        // Notes
        {
            id: 'note1',
            shape: 'rect',
            x: 180,
            y: 280,
            width: 120,
            height: 40,
            label: 'Mark Link Up',
            attrs: { body: { fill: '#fffbe6', stroke: '#ffe58f', rx: 5, ry: 5 } }
        },
        {
            id: 'note2',
            shape: 'rect',
            x: 180,
            y: 430,
            width: 120,
            height: 40,
            label: 'Probe Timeout',
            attrs: { body: { fill: '#fffbe6', stroke: '#ffe58f', rx: 5, ry: 5 } }
        },
        {
            id: 'note3',
            shape: 'rect',
            x: 180,
            y: 490,
            width: 120,
            height: 40,
            label: 'Mark Link Down',
            attrs: { body: { fill: '#fffbe6', stroke: '#ffe58f', rx: 5, ry: 5 } }
        },
    ],
    edges: [
        {
            source: { cell: 'client-life', y: 200 },
            target: { cell: 'server-life', y: 200 },
            label: 'Health Probe',
        },
        {
            source: { cell: 'server-life', y: 250 },
            target: { cell: 'client-life', y: 250 },
            label: 'Probe ACK',
            attrs: { line: { strokeDasharray: '5 5' } }
        },
        {
            source: { cell: 'client-life', y: 350 },
            target: { cell: 'server-life', y: 350 },
            label: 'Health Probe',
        },
    ],
};
