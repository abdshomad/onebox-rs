window.currentDiagram = {
    nodes: [
        {
            id: 'client',
            x: 50,
            y: 250,
            width: 150,
            height: 80,
            label: 'onebox-client',
            attrs: {
                body: {
                    fill: '#f9f',
                    stroke: '#333',
                    strokeWidth: 2,
                },
            },
        },
        {
            id: 'server',
            x: 600,
            y: 250,
            width: 150,
            height: 80,
            label: 'onebox-server',
            attrs: {
                body: {
                    fill: '#ccf',
                    stroke: '#333',
                    strokeWidth: 2,
                },
            },
        },
        {
            id: 'wan1',
            x: 325,
            y: 100,
            width: 150,
            height: 50,
            label: 'WAN 1 (Ethernet)',
        },
        {
            id: 'wan2',
            x: 325,
            y: 450,
            width: 150,
            height: 50,
            label: 'WAN 2 (Cellular)',
        },
    ],
    edges: [
        {
            source: 'client',
            target: 'wan1',
        },
        {
            source: 'client',
            target: 'wan2',
        },
        {
            source: 'wan1',
            target: 'server',
        },
        {
            source: 'wan2',
            target: 'server',
        },
    ],
};
