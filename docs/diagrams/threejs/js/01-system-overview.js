import * as THREE from 'three';
import { FontLoader } from 'three/addons/loaders/FontLoader.js';
import { TextGeometry } from 'three/addons/geometries/TextGeometry.js';

export function createSystemOverview(scene) {
    const group = new THREE.Group();
    scene.add(group);

    // 3. Create Nodes (3D Objects)
    const nodeMaterialClient = new THREE.MeshPhongMaterial({ color: 0xf900f9 });
    const nodeMaterialServer = new THREE.MeshPhongMaterial({ color: 0xccccff });
    const nodeMaterialWAN = new THREE.MeshPhongMaterial({ color: 0xeeeeee });

    const client = new THREE.Mesh(new THREE.BoxGeometry(2, 1, 1), nodeMaterialClient);
    client.position.x = -6;
    group.add(client);

    const server = new THREE.Mesh(new THREE.BoxGeometry(2, 1, 1), nodeMaterialServer);
    server.position.x = 6;
    group.add(server);

    const wan1 = new THREE.Mesh(new THREE.BoxGeometry(2, 0.5, 1), nodeMaterialWAN);
    wan1.position.y = 3;
    group.add(wan1);

    const wan2 = new THREE.Mesh(new THREE.BoxGeometry(2, 0.5, 1), nodeMaterialWAN);
    wan2.position.y = -3;
    group.add(wan2);

    // Add Text Labels
    const fontLoader = new FontLoader();
    fontLoader.load('https://unpkg.com/three@0.160.0/examples/fonts/helvetiker_regular.typeface.json', (font) => {
        const textMaterial = new THREE.MeshBasicMaterial({ color: 0xffffff });
        const createText = (text, position) => {
            const geometry = new TextGeometry(text, {
                font: font,
                size: 0.5,
                height: 0.1,
            });
            const mesh = new THREE.Mesh(geometry, textMaterial);
            mesh.position.copy(position);
            group.add(mesh);
        };
        createText('onebox-client', new THREE.Vector3(-7, 0.7, 0));
        createText('onebox-server', new THREE.Vector3(5, 0.7, 0));
        createText('WAN 1', new THREE.Vector3(-1, 3.5, 0));
        createText('WAN 2', new THREE.Vector3(-1, -2.5, 0));
    });


    // 4. Create Edges (Lines)
    const lineMaterial = new THREE.LineBasicMaterial({ color: 0xaaaaaa });

    function createLine(startVec, endVec) {
        const points = [startVec, endVec];
        const geometry = new THREE.BufferGeometry().setFromPoints(points);
        return new THREE.Line(geometry, lineMaterial);
    }

    group.add(createLine(client.position, wan1.position));
    group.add(createLine(client.position, wan2.position));
    group.add(createLine(wan1.position, server.position));
    group.add(createLine(wan2.position, server.position));

    // 5. Animation
    const packetMaterial = new THREE.MeshBasicMaterial({ color: 0x00aaff });
    const packetGeo = new THREE.SphereGeometry(0.15, 8, 8);

    const packets = [];

    function createPacket(startPos, endPos) {
        const packet = new THREE.Mesh(packetGeo, packetMaterial);
        packet.position.copy(startPos);
        packets.push({ mesh: packet, start: startPos, end: endPos, progress: 0 });
        group.add(packet);
    }

    const intervalId = setInterval(() => {
        createPacket(client.position, wan1.position);
        createPacket(client.position, wan2.position);
    }, 1000);

    const animate = (delta) => {
        // Animate packets
        for (let i = packets.length - 1; i >= 0; i--) {
            const p = packets[i];
            p.progress += delta * 0.5; // speed

            if (p.progress >= 1) {
                if (p.end === wan1.position || p.end === wan2.position) {
                     p.start = p.end;
                     p.end = server.position;
                     p.progress = 0;
                } else {
                    group.remove(p.mesh);
                    packets.splice(i, 1);
                }
            } else {
                p.mesh.position.lerpVectors(p.start, p.end, p.progress);
            }
        }

        // Rotate the whole group for a nice effect
        group.rotation.y += delta * 0.1;
    };

    const cleanup = () => {
        clearInterval(intervalId);
    };

    return { animate, cleanup };
}
