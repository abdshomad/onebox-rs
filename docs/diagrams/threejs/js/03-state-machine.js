import * as THREE from 'three';
import { FontLoader } from 'three/addons/loaders/FontLoader.js';
import { TextGeometry } from 'three/addons/geometries/TextGeometry.js';
import { TubeGeometry } from 'three/addons/geometries/TubeGeometry.js';

export function createStateMachine(scene) {
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

        // States
        const stateMaterialUp = new THREE.MeshPhongMaterial({ color: 0x00ff00 });
        const stateMaterialDown = new THREE.MeshPhongMaterial({ color: 0xff0000 });
        const stateMaterialUnknown = new THREE.MeshPhongMaterial({ color: 0x888888 });

        const startGeo = new THREE.SphereGeometry(0.2, 16, 16);
        const start = new THREE.Mesh(startGeo, new THREE.MeshBasicMaterial({color: 0x000000}));
        start.position.set(0, 5, 0);
        group.add(start);

        const unknownGeo = new THREE.SphereGeometry(1, 16, 16);
        const unknown = new THREE.Mesh(unknownGeo, stateMaterialUnknown);
        unknown.position.set(0, 0, 0);
        group.add(unknown);
        createText('Unknown', 0.5, new THREE.Vector3(-0.5, 0, 1.2));

        const upGeo = new THREE.SphereGeometry(1, 16, 16);
        const up = new THREE.Mesh(upGeo, stateMaterialUp);
        up.position.set(-5, -5, 0);
        group.add(up);
        createText('Up', 0.5, new THREE.Vector3(-5.5, -5, 1.2));

        const downGeo = new THREE.SphereGeometry(1, 16, 16);
        const down = new THREE.Mesh(downGeo, stateMaterialDown);
        down.position.set(5, -5, 0);
        group.add(down);
        createText('Down', 0.5, new THREE.Vector3(4.5, -5, 1.2));

        // Transitions
        const transitionMaterial = new THREE.MeshPhongMaterial({ color: 0xaaaaaa });
        const createTransition = (startPos, endPos, label, labelPos) => {
            const path = new THREE.LineCurve3(startPos, endPos);
            const geometry = new TubeGeometry(path, 20, 0.1, 8, false);
            const mesh = new THREE.Mesh(geometry, transitionMaterial);
            group.add(mesh);
            createText(label, 0.3, labelPos);
        };

        createTransition(start.position, unknown.position, '', new THREE.Vector3());
        createTransition(unknown.position, up.position, 'Successful Probe', new THREE.Vector3(-3, -2, 2));
        createTransition(unknown.position, down.position, '4 Fails', new THREE.Vector3(2, -2, 2));

        const downToUpPath = new THREE.CatmullRomCurve3([
            new THREE.Vector3(5, -5, 0),
            new THREE.Vector3(0, -10, 0),
            new THREE.Vector3(-5, -5, 0),
        ]);
        const downToUpGeo = new TubeGeometry(downToUpPath, 20, 0.1, 8, false);
        const downToUp = new THREE.Mesh(downToUpGeo, transitionMaterial);
        group.add(downToUp);
        createText('Successful Probe', 0.3, new THREE.Vector3(-1, -9, 0));

        const upToDownPath = new THREE.CatmullRomCurve3([
            new THREE.Vector3(-5, -5, 0),
            new THREE.Vector3(0, -8, 0),
            new THREE.Vector3(5, -5, 0),
        ]);
        const upToDownGeo = new TubeGeometry(upToDownPath, 20, 0.1, 8, false);
        const upToDown = new THREE.Mesh(upToDownGeo, transitionMaterial);
        group.add(upToDown);
        createText('4 Fails', 0.3, new THREE.Vector3(-0.5, -7, 0));
    });

    const animate = (delta) => {
        group.rotation.y += delta * 0.2;
    };

    const cleanup = () => {};

    return { animate, cleanup };
}
