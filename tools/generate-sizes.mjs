#!/usr/bin/env node

import sharp from 'sharp';
import fs from 'fs/promises';

const input_dir = './res/images';
const output_dir = './code/web/public/images';

const sizes = [
    [640,  360,   '-360p.webp', 80],
    [1280, 720,   '-720p.webp', 90],
    [1920, 1080, '-1080p.webp', 90],
    [2560, 1440, '-1440p.webp', 90],
    [3840, 2160, '-2160p.webp', 90],
    [ 360,  640,  '-360p-vertical.webp', 80],
    [ 720, 1280,  '-720p-vertical.webp', 90],
    [1080, 1920, '-1080p-vertical.webp', 90],
    [1440, 2560, '-1440p-vertical.webp', 90],
    [2160, 3840, '-2160p-vertical.webp', 90],
];

// Delete old files / create output directory
await fs.rm(output_dir, { recursive: true, force: true });
fs.mkdir(output_dir, { recursive: true });

// Generate new files
let tasks = [];
for(const jpg of await fs.readdir(input_dir)) {
    const path = `${input_dir}/${jpg}`;
    // console.log(`${path}`)

    for(const [w, h, suffix, quality] of sizes) {
        const output_path = `${output_dir}/${jpg.replace('.jpg', suffix)}`;
        // console.log(`\t${output_path} ${w}x${h}`);
        tasks.push(
            sharp(path)
            .resize(w, h)
            .webp({ quality })
            .toFile(output_path)
        );
    }
}

process.stdout.write("Rendering images: 0/0")
for(const [task, i] of tasks.map((task, i) => [task, i+1])) {
    await task;
    process.stdout.write(`\rRendering images: ${i}/${tasks.length} (${Math.floor(i/tasks.length*100)}%)`)
}
process.stdout.write(`\rRendering images: Rendered ${tasks.length} images\n`)