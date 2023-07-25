import sharp from "sharp";

if (process.argv.length !== 3) {
  console.error(
    `Usage: ${process.argv[0]} ${process.argv[1]} <path-to-source-icon>`,
  );
  process.exit(1);
}

const assets = {
  "tray-template.png": 64,
};

for (const [filename, size] of Object.entries(assets)) {
  sharp(process.argv[2])
    .resize(size, size)
    .toFile(`src-tauri/icons/${filename}`)
    .then(() => console.log(`Generated ${filename}`))
    .catch((err) => console.error(err));
}
