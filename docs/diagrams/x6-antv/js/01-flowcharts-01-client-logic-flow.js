window.currentDiagram = {
    nodes: [
        {
            id: 'A',
            shape: 'rect',
            x: 325,
            y: 40,
            width: 150,
            height: 40,
            label: 'Start onebox-client',
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
            shape: 'rect',
            x: 100,
            y: 220,
            width: 200,
            height: 50,
            label: 'Discover WAN Interfaces\n& Bind Sockets',
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
            y: 500,
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
            label: 'Create Virtual\nTUN Device',
            attrs: {
                body: {
                    refPoints: '50,0 100,50 50,100 0,50'
                }
            }
        },
        {
            id: 'G',
            shape: 'rect',
            x: 100,
            y: 410,
            width: 200,
            height: 50,
            label: 'Set System Default Route\nto TUN',
        },
        {
            id: 'H',
            shape: 'rect',
            x: 350,
            y: 320,
            width: 150,
            height: 40,
            label: 'Log TUN Device Error',
        },
        {
            id: 'I',
            shape: 'polygon',
            x: 125,
            y: 500,
            width: 150,
            height: 60,
            label: 'Perform Handshake',
            attrs: {
                body: {
                    refPoints: '50,0 100,50 50,100 0,50'
                }
            }
        },
        {
            id: 'J',
            shape: 'rect',
            x: 125,
            y: 600,
            width: 150,
            height: 40,
            label: 'Enter Main Loop',
        },
        {
            id: 'K',
            shape: 'rect',
            x: 350,
            y: 540,
            width: 150,
            height: 40,
            label: 'Log Handshake Error',
        },
        {
            id: 'L',
            shape: 'rect',
            x: 125,
            y: 680,
            width: 150,
            height: 50,
            label: 'Read IP Packet\nfrom TUN',
        },
        {
            id: 'M',
            shape: 'rect',
            x: 325,
            y: 680,
            width: 150,
            height: 50,
            label: 'Encrypt &\nEncapsulate',
        },
        {
            id: 'N',
            shape: 'rect',
            x: 525,
            y: 680,
            width: 150,
            height: 50,
            label: 'Select WAN link',
        },
        {
            id: 'O',
            shape: 'rect',
            x: 725,
            y: 680,
            width: 150,
            height: 50,
            label: 'Send Packet',
        },
    ],
    edges: [
        { source: 'A', target: 'B' },
        { source: 'B', target: 'C', label: 'Success' },
        { source: 'B', target: 'D', label: 'Failure' },
        { source: 'D', target: 'E' },
        { source: 'C', target: 'F' },
        { source: 'F', target: 'G', label: 'Success' },
        { source: 'F', target: 'H', label: 'Failure' },
        { source: 'H', target: 'E' },
        { source: 'G', target: 'I' },
        { source: 'I', target: 'J', label: 'Success' },
        { source: 'I', target: 'K', label: 'Failure' },
        { source: 'K', target: 'E' },
        { source: 'J', target: 'L' },
        { source: 'L', target: 'M' },
        { source: 'M', target: 'N' },
        { source: 'N', target: 'O' },
        {
            source: 'O',
            target: 'L',
            vertices: [ {x: 800, y: 750}, {x: 200, y: 750} ],
        },
    ],
};
