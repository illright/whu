import sharp from "sharp";
import { IconIco, IconIcns } from "@shockpkg/icon-encoder";
import { readFileSync, unlinkSync, writeFileSync } from "fs";

if (process.argv.length !== 3) {
  console.error(
    `Usage: ${process.argv[0]} ${process.argv[1]} <path-to-source-icon>`,
  );
  process.exit(1);
}

const assets = {
  "32x32.png": 32,
  "128x128.png": 128,
  "128x128@2x.png": 256,
  "icon.png": 512,
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

const assetsForIco = {
  "16x16.png": 16,
  "24x24.png": 24,
  "48x48.png": 48,
  "64x64.png": 64,
  "256x256.png": 256,
};

const ico = new IconIco();
ico.addFromPng(readFileSync("src-tauri/icons/32x32.png"), null, false);

const icoPromises: Promise<void>[] = [];
for (const [filename, size] of Object.entries(assetsForIco)) {
  icoPromises.push(
    sharp(process.argv[2])
      .resize(size, size)
      .toFile(`src-tauri/icons/${filename}`)
      .then(() => {
        ico.addFromPng(
          readFileSync(`src-tauri/icons/${filename}`),
          null,
          false,
        );
        unlinkSync(`src-tauri/icons/${filename}`);
      })
      .catch((err) => console.error(err)),
  );
}

Promise.all(icoPromises).then(() => {
  writeFileSync("src-tauri/icons/icon.ico", ico.encode());
  console.log("Generated icon.ico");
});

const icns = new IconIcns();
icns.toc = true;

const assetsForIcns = (await fetch(
  "https://raw.githubusercontent.com/tauri-apps/tauri/dev/tooling/cli/src/helpers/icns.json",
).then((res) => res.json())) as Record<
  string,
  { name: string; size: number; ostype: string }
>;

const icnsPromises: Promise<void>[] = [];
for (const [_name, { name: filename, size, ostype }] of Object.entries(
  assetsForIcns,
)) {
  icnsPromises.push(
    sharp(process.argv[2])
      .resize(size, size)
      .toFile(`src-tauri/icons/${filename}`)
      .then(() => {
        icns.addFromPng(
          readFileSync(`src-tauri/icons/${filename}`),
          [ostype],
          false,
        );
        unlinkSync(`src-tauri/icons/${filename}`);
      })
      .catch((err) => console.error(err)),
  );
}

Promise.all(icnsPromises).then(() => {
  writeFileSync("src-tauri/icons/icon.icns", icns.encode());
  console.log("Generated icon.icns");
});
