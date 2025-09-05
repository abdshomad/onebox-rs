window.currentDiagram = {
    nodes: [
        {
            id: 'Config',
            shape: 'html',
            x: 300,
            y: 50,
            width: 200,
            height: 100,
            html: `
                <div class="class-node">
                    <div class="class-name">Config</div>
                    <div class="class-attrs">
                        +String log_level<br/>
                        +String preshared_key
                    </div>
                </div>
            `,
        },
        {
            id: 'ClientConfig',
            shape: 'html',
            x: 50,
            y: 250,
            width: 220,
            height: 150,
            html: `
                <div class="class-node">
                    <div class="class-name">ClientConfig</div>
                    <div class="class-attrs">
                        +String server_address<br/>
                        +int server_port<br/>
                        +String tun_name<br/>
                        +String tun_ip<br/>
                        +String tun_netmask
                    </div>
                </div>
            `,
        },
        {
            id: 'ServerConfig',
            shape: 'html',
            x: 550,
            y: 250,
            width: 200,
            height: 100,
            html: `
                <div class="class-node">
                    <div class="class-name">ServerConfig</div>
                    <div class="class-attrs">
                        +String listen_address<br/>
                        +int listen_port
                    </div>
                </div>
            `,
        },
    ],
    edges: [
        {
            source: 'Config',
            target: 'ClientConfig',
            labels: ['1', '1'],
        },
        {
            source: 'Config',
            target: 'ServerConfig',
            labels: ['1', '1'],
        },
    ],
};

// We need to add some CSS to style the HTML nodes
const style = document.createElement('style');
style.innerHTML = `
    .class-node {
        border: 1px solid #000;
        background-color: #fff;
        font-family: monospace;
        width: 100%;
        height: 100%;
    }
    .class-name {
        font-weight: bold;
        text-align: center;
        padding: 5px;
        border-bottom: 1px solid #000;
    }
    .class-attrs {
        padding: 5px;
        text-align: left;
    }
`;
document.head.appendChild(style);
