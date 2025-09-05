window.currentDiagram = {
    nodes: [
        {
            id: 'A',
            shape: 'rect',
            x: 325,
            y: 40,
            width: 150,
            height: 40,
            label: 'Start onebox-server',
        },
        {
            id: 'B',
            shape: 'polygon',
            x: 325,
            y: 120,
            width: 150,
            height: 60,
            label: 'Load config.toml',
            attrs: {
                body: {
                    refPoints: '50,0 100,50 50,100 0,50'
                }
            }
        },
        {
            id: 'C',
            shape: 'polygon',
            x: 100,
            y: 220,
            width: 200,
            height: 50,
            label: 'Create TUN Device\n& Setup NAT',
            attrs: {
                body: {
                    refPoints: '50,0 100,50 50,100 0,50'
                }
            }
        },
        {
            id: 'D',
            shape: 'rect',
            x: 550,
            y: 120,
            width: 150,
            height: 40,
            label: 'Log Config Error',
        },
        {
            id: 'E',
            shape: 'rect',
            x: 550,
            y: 460,
            width: 150,
            height: 40,
            label: 'Stop',
            attrs: {
                body: { fill: '#ff4d4f' },
                label: { fill: '#fff' },
            }
        },
        {
            id: 'F',
            shape: 'polygon',
            x: 125,
            y: 310,
            width: 150,
            height: 60,
            label: 'Bind Public\nUDP Socket',
            attrs: {
                body: {
                    refPoints: '50,0 100,50 50,100 0,50'
                }
            }
        },
        {
            id: 'G',
            shape: 'rect',
            x: 350,
            y: 225,
            width: 150,
            height: 40,
            label: 'Log TUN/NAT Error',
        },
        {
            id: 'H',
            shape: 'rect',
            x: 125,
            y: 410,
            width: 150,
            height: 40,
            label: 'Enter Main Loop',
        },
        {
            id: 'I',
            shape: 'rect',
            x: 350,
            y: 320,
            width: 150,
            height: 40,
            label: 'Log Socket Bind Error',
        },
        {
            id: 'J',
            shape: 'rect',
            x: 125,
            y: 490,
            width: 150,
            height: 40,
            label: 'Listen for Packets',
        },
        {
            id: 'K',
            shape: 'polygon',
            x: 125,
            y: 570,
            width: 150,
            height: 60,
            label: 'Packet Received?',
            attrs: {
                body: {
                    refPoints: '50,0 100,50 50,100 0,50'
                }
            }
        },
        {
            id: 'L',
            shape: 'rect',
            x: 125,
            y: 670,
            width: 150,
            height: 40,
            label: 'Process Packet',
        },
    ],
    edges: [
        { source: 'A', target: 'B' },
        { source: 'B', target: 'C', label: 'Success' },
        { source: 'B', target: 'D', label: 'Failure' },
        { source: 'D', target: 'E' },
        { source: 'C', target: 'F', label: 'Success' },
        { source: 'C', target: 'G', label: 'Failure' },
        { source: 'G', target: 'E' },
        { source: 'F', target: 'H', label: 'Success' },
        { source: 'F', target: 'I', label: 'Failure' },
        { source: 'I', target: 'E' },
        { source: 'H', target: 'J' },
        { source: 'J', target: 'K' },
        { source: 'K', target: 'L', label: 'Yes' },
        {
            source: 'L',
            target: 'J',
            vertices: [ {x: 200, y: 740}, {x: 200, y: 550} ],
        },
        {
            source: 'K',
            target: 'J',
            label: 'No',
            vertices: [ {x: 50, y: 600}, {x: 50, y: 510} ],
        },
    ],
};
