window.currentDiagram = {
    nodes: [
        // Title
        {
            id: 'title',
            shape: 'text',
            x: 400,
            y: 20,
            width: 400,
            height: 30,
            label: 'System Context diagram for onebox-rs',
            attrs: {
                text: {
                    fontSize: 20,
                    fontWeight: 'bold',
                }
            }
        },
        // Enterprise Boundary
        {
            id: 'boundary',
            shape: 'rect',
            x: 200,
            y: 150,
            width: 400,
            height: 300,
            label: 'onebox-rs System',
            attrs: {
                body: {
                    fill: '#f0f0f0',
                    stroke: '#ccc',
                    strokeDasharray: '5 5',
                },
                label: {
                    refY: 5,
                    textAnchor: 'center',
                }
            }
        },
        // Systems
        {
            id: 'client',
            shape: 'html',
            x: 250,
            y: 200,
            width: 150,
            height: 100,
            parent: 'boundary',
            html: `
                <div class="c4-node c4-system">
                    <div class="c4-name">onebox-client</div>
                    <div class="c4-desc">Intercepts traffic and distributes it across multiple WAN links</div>
                </div>
            `
        },
        {
            id: 'server',
            shape: 'html',
            x: 400,
            y: 300,
            width: 150,
            height: 100,
            parent: 'boundary',
            html: `
                <div class="c4-node c4-system">
                    <div class="c4-name">onebox-server</div>
                    <div class="c4-desc">Receives traffic, reassembles it, and forwards it to the internet</div>
                </div>
            `
        },
        // External Systems
        {
            id: 'user',
            shape: 'html',
            x: 50,
            y: 200,
            width: 150,
            height: 100,
            html: `
                <div class="c4-node c4-external">
                    <div class="c4-name">User</div>
                    <div class="c4-desc">A user of the onebox-rs system</div>
                </div>
            `
        },
        {
            id: 'internet',
            shape: 'html',
            x: 600,
            y: 300,
            width: 150,
            height: 100,
            html: `
                <div class="c4-node c4-external">
                    <div class="c4-name">Internet</div>
                    <div class="c4-desc">The public internet</div>
                </div>
            `
        },
    ],
    edges: [
        {
            source: 'client',
            target: 'server',
            label: 'Sends encrypted packets over UDP',
        },
        {
            source: 'user',
            target: 'client',
            label: 'Uses the bonded internet connection',
        },
        {
            source: 'server',
            target: 'internet',
            label: 'Forwards traffic to',
        },
    ],
};

// Add CSS for C4 nodes
const style = document.createElement('style');
style.innerHTML = `
    .c4-node {
        border: 1px solid #000;
        padding: 5px;
        text-align: center;
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }
    .c4-system {
        background-color: #e6f7ff;
        border-color: #91d5ff;
    }
    .c4-external {
        background-color: #fafafa;
        border-color: #d9d9d9;
    }
    .c4-name {
        font-weight: bold;
        margin-bottom: 5px;
    }
    .c4-desc {
        font-size: 12px;
        color: #666;
    }
`;
document.head.appendChild(style);
