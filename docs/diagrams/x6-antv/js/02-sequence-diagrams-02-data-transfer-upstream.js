window.currentDiagram = {
    nodes: [
        // Participants
        {
            id: 'lan',
            shape: 'rect',
            x: 50,
            y: 50,
            width: 120,
            height: 50,
            label: 'LAN',
        },
        {
            id: 'client',
            shape: 'rect',
            x: 250,
            y: 50,
            width: 120,
            height: 50,
            label: 'onebox-client',
        },
        {
            id: 'server',
            shape: 'rect',
            x: 450,
            y: 50,
            width: 120,
            height: 50,
            label: 'onebox-server',
        },
        {
            id: 'internet',
            shape: 'rect',
            x: 650,
            y: 50,
            width: 120,
            height: 50,
            label: 'Internet',
        },
        // Lifelines
        {
            id: 'lan-life',
            shape: 'rect',
            x: 109,
            y: 100,
            width: 2,
            height: 400,
            attrs: { body: { fill: '#ccc', stroke: '#ccc' } }
        },
        {
            id: 'client-life',
            shape: 'rect',
            x: 309,
            y: 100,
            width: 2,
            height: 400,
            attrs: { body: { fill: '#ccc', stroke: '#ccc' } }
        },
        {
            id: 'server-life',
            shape: 'rect',
            x: 509,
            y: 100,
            width: 2,
            height: 400,
            attrs: { body: { fill: '#ccc', stroke: '#ccc' } }
        },
        {
            id: 'internet-life',
            shape: 'rect',
            x: 709,
            y: 100,
            width: 2,
            height: 400,
            attrs: { body: { fill: '#ccc', stroke: '#ccc' } }
        },
    ],
    edges: [
        {
            source: { cell: 'lan-life', y: 150 },
            target: { cell: 'client-life', y: 150 },
            label: 'IP Packet',
        },
        {
            source: { cell: 'client-life', y: 200 },
            target: { cell: 'client-life', y: 230 },
            vertices: [{ x: 340, y: 200 }, { x: 340, y: 230 }],
            label: 'Encrypt & Encapsulate',
        },
        {
            source: { cell: 'client-life', y: 280 },
            target: { cell: 'server-life', y: 280 },
            label: 'Encrypted Packet',
        },
        {
            source: { cell: 'server-life', y: 330 },
            target: { cell: 'server-life', y: 360 },
            vertices: [{ x: 540, y: 330 }, { x: 540, y: 360 }],
            label: 'Decrypt & Reassemble',
        },
        {
            source: { cell: 'server-life', y: 410 },
            target: { cell: 'internet-life', y: 410 },
            label: 'Original IP Packet',
        },
    ],
};
