import sharp from "sharp";

if (process.argv.length !== 3) {
  console.error(
    `Usage: ${process.argv[0]} ${process.argv[1]} <path-to-source-icon>`
  );
  process.exit(1);
}

const assets = {
  "32x32.png": 32,
  "128x128.png": 128,
  "128x128@2x.png": 256,
  "icon.ico": 256,
  "Square30x30Logo.png": 30,
  "Square44x44Logo.png": 44,
  "Square71x71Logo.png": 71,
  "Square89x89Logo.png": 89,
  "Square107x107Logo.png": 107,
  "Square142x142Logo.png": 142,
  "Square150x150Logo.png": 150,
  "Square284x284Logo.png": 284,
  "Square310x310Logo.png": 310,
  "StoreLogo.png": 50,
};

for (const [filename, size] of Object.entries(assets)) {
  sharp(process.argv[2])
    .resize(size, size)
    .toFile(`src-tauri/icons/${filename}`)
    .then(() => console.log(`Generated ${filename}`))
    .catch((err) => console.error(err));
}
