window.currentDiagram = {
    nodes: [
        {
            id: 'start',
            shape: 'circle',
            x: 350,
            y: 50,
            width: 20,
            height: 20,
            attrs: {
                body: {
                    fill: '#000',
                    stroke: '#000',
                }
            }
        },
        {
            id: 'unknown',
            shape: 'rect',
            x: 300,
            y: 150,
            width: 100,
            height: 50,
            label: 'Unknown',
            attrs: {
                body: {
                    rx: 10, // rounded corners
                    ry: 10,
                }
            }
        },
        {
            id: 'up',
            shape: 'rect',
            x: 150,
            y: 250,
            width: 100,
            height: 50,
            label: 'Up',
            attrs: {
                body: {
                    rx: 10,
                    ry: 10,
                    fill: '#d4edda' // green
                }
            }
        },
        {
            id: 'down',
            shape: 'rect',
            x: 450,
            y: 250,
            width: 100,
            height: 50,
            label: 'Down',
            attrs: {
                body: {
                    rx: 10,
                    ry: 10,
                    fill: '#f8d7da' // red
                }
            }
        },
    ],
    edges: [
        {
            source: 'start',
            target: 'unknown',
        },
        {
            source: 'unknown',
            target: 'up',
            label: 'Successful Probe',
        },
        {
            source: 'unknown',
            target: 'down',
            label: '4 Consecutive\nProbe Failures',
        },
        {
            source: 'up',
            target: 'down',
            label: '4 Consecutive\nProbe Failures',
            vertices: [{ x: 200, y: 350 }, { x: 500, y: 350 }],
        },
        {
            source: 'down',
            target: 'up',
            label: 'Successful Probe',
            vertices: [{ x: 500, y: 400 }, { x: 200, y: 400 }],
        },
    ],
};
