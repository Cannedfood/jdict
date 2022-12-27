import sharp from 'sharp';
import fs from 'fs/promises';

const input_dir = './web/images';
const output_dir = './web/public/images';

const sizes = [
    [640,  360,   '-360p.webp', 80],
    [1280, 720,   '-720p.webp', 90],
    [1920, 1080, '-1080p.webp', 90],
    [2560, 1440, '-1440p.webp', 90],
    [3840, 2160, '-2160p.webp', 90],
];

// Delete old files / create output directory
await fs.rm(output_dir, { recursive: true, force: true });
fs.mkdir(output_dir, { recursive: true });

// Generate new files
for(const jpg of await fs.readdir('./images')) {
    const path = `${input_dir}/${jpg}`;
    console.log(`${path}`)

    for(const [w, h, suffix, quality] of sizes) {
        const output_path = `${output_dir}/${jpg.replace('.jpg', suffix)}`;
        console.log(`\t${output_path} ${w}x${h}`);
        await (
            sharp(path)
            .resize(w, h)
            .webp({ quality })
            .toFile(output_path)
        );
    }
}