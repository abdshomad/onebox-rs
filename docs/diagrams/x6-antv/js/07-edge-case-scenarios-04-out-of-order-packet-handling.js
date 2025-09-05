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
    ],
    edges: [
        {
            source: { cell: 'client-life', y: 150 },
            target: { cell: 'server-life', y: 150 },
            label: 'Packet (Seq=3)',
        },
        {
            source: { cell: 'client-life', y: 200 },
            target: { cell: 'server-life', y: 200 },
            label: 'Packet (Seq=1)',
        },
        {
            source: { cell: 'client-life', y: 250 },
            target: { cell: 'server-life', y: 250 },
            label: 'Packet (Seq=2)',
        },
        {
            source: { cell: 'server-life', y: 300 },
            target: { cell: 'server-life', y: 330 },
            vertices: [{ x: 600, y: 300 }, { x: 600, y: 330 }],
            label: 'Add packets to Jitter Buffer',
        },
        {
            source: { cell: 'server-life', y: 360 },
            target: { cell: 'server-life', y: 390 },
            vertices: [{ x: 600, y: 360 }, { x: 600, y: 390 }],
            label: 'Reorder packets',
        },
        {
            source: { cell: 'server-life', y: 420 },
            target: { cell: 'server-life', y: 450 },
            vertices: [{ x: 600, y: 420 }, { x: 600, y: 450 }],
            label: 'Process Packet (Seq=1)',
        },
        {
            source: { cell: 'server-life', y: 480 },
            target: { cell: 'server-life', y: 510 },
            vertices: [{ x: 600, y: 480 }, { x: 600, y: 510 }],
            label: 'Process Packet (Seq=2)',
        },
        {
            source: { cell: 'server-life', y: 540 },
            target: { cell: 'server-life', y: 570 },
            vertices: [{ x: 600, y: 540 }, { x: 600, y: 570 }],
            label: 'Process Packet (Seq=3)',
        },
    ],
};
