import * as THREE from 'three';
import { FontLoader } from 'three/addons/loaders/FontLoader.js';
import { TextGeometry } from 'three/addons/geometries/TextGeometry.js';

export function createPacketStructure(scene) {
    const group = new THREE.Group();
    scene.add(group);

    const fontLoader = new FontLoader();
    fontLoader.load('https://unpkg.com/three@0.160.0/examples/fonts/helvetiker_regular.typeface.json', (font) => {
        const textMaterial = new THREE.MeshBasicMaterial({ color: 0xffffff });

        const createText = (text, size, position) => {
            const geometry = new TextGeometry(text, { font, size, height: 0.1 });
            const mesh = new THREE.Mesh(geometry, textMaterial);
            mesh.position.copy(position);
            group.add(mesh);
        };

        // Create the layers
        const udpDatagramMat = new THREE.MeshPhongMaterial({ color: 0x00ffff, transparent: true, opacity: 0.2 });
        const udpDatagramGeo = new THREE.BoxGeometry(12, 4, 4);
        const udpDatagram = new THREE.Mesh(udpDatagramGeo, udpDatagramMat);
        group.add(udpDatagram);
        createText('UDP Datagram', 0.5, new THREE.Vector3(-5.5, 2.5, 0));

        const ipHeaderMat = new THREE.MeshPhongMaterial({ color: 0xff00ff, transparent: true, opacity: 0.3 });
        const ipHeaderGeo = new THREE.BoxGeometry(10, 1.5, 3);
        const ipHeader = new THREE.Mesh(ipHeaderGeo, ipHeaderMat);
        ipHeader.position.y = 1;
        group.add(ipHeader);
        createText('IP Header', 0.3, new THREE.Vector3(-4.5, 2, 2));

        const udpHeaderMat = new THREE.MeshPhongMaterial({ color: 0xffff00, transparent: true, opacity: 0.3 });
        const udpHeaderGeo = new THREE.BoxGeometry(10, 1.5, 3);
        const udpHeader = new THREE.Mesh(udpHeaderGeo, udpHeaderMat);
        udpHeader.position.y = -1;
        group.add(udpHeader);
        createText('UDP Header', 0.3, new THREE.Vector3(-4.5, 0, 2));

        const oneboxHeaderMat = new THREE.MeshPhongMaterial({ color: 0x00ff00, transparent: true, opacity: 0.3 });
        const oneboxHeaderGeo = new THREE.BoxGeometry(10, 1.5, 3);
        const oneboxHeader = new THREE.Mesh(oneboxHeaderGeo, oneboxHeaderMat);
        oneboxHeader.position.y = -3;
        group.add(oneboxHeader);
        createText('onebox Header', 0.3, new THREE.Vector3(-4.5, -2, 2));

        const payloadMat = new THREE.MeshPhongMaterial({ color: 0xffa500, transparent: true, opacity: 0.3 });
        const payloadGeo = new THREE.BoxGeometry(10, 1.5, 3);
        const payload = new THREE.Mesh(payloadGeo, payloadMat);
        payload.position.y = -5;
        group.add(payload);
        createText('Encrypted Payload', 0.3, new THREE.Vector3(-4.5, -4, 2));

        // Create the fields
        const fieldMat = new THREE.MeshPhongMaterial({ color: 0xffffff });
        const createField = (name, size, position, parent) => {
             const fieldGeo = new THREE.BoxGeometry(size.x, size.y, size.z);
             const field = new THREE.Mesh(fieldGeo, fieldMat);
             field.position.copy(position);
             parent.add(field);
             createText(name, 0.2, position.add(new THREE.Vector3(0, 0, 1)));
        }

        createField('Source IP', new THREE.Vector3(4, 0.5, 1), new THREE.Vector3(-2, 0, 0), ipHeader);
        createField('Dest IP', new THREE.Vector3(4, 0.5, 1), new THREE.Vector3(2, 0, 0), ipHeader);

        createField('Source Port', new THREE.Vector3(4, 0.5, 1), new THREE.Vector3(-2, 0, 0), udpHeader);
        createField('Dest Port', new THREE.Vector3(4, 0.5, 1), new THREE.Vector3(2, 0, 0), udpHeader);

        createField('Type', new THREE.Vector3(1, 0.5, 1), new THREE.Vector3(-4, 0, 0), oneboxHeader);
        createField('Client ID', new THREE.Vector3(2, 0.5, 1), new THREE.Vector3(-1.5, 0, 0), oneboxHeader);
        createField('Sequence', new THREE.Vector3(3, 0.5, 1), new THREE.Vector3(2.5, 0, 0), oneboxHeader);

        createField('Original IP Packet', new THREE.Vector3(4, 0.5, 1), new THREE.Vector3(-2, 0, 0), payload);
        createField('Auth Tag', new THREE.Vector3(4, 0.5, 1), new THREE.Vector3(2, 0, 0), payload);
    });

    const animate = (delta) => {
        group.rotation.y += delta * 0.2;
        group.rotation.x += delta * 0.1;
    };

    const cleanup = () => {
        // No intervals to clear
    };

    return { animate, cleanup };
}
