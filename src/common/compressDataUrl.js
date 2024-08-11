import imageCompression from 'browser-image-compression';

import { ImageUtil } from 'wasm_png_compression';

export const compressDataUrl = async dataUrl => {
  try {
    const wasm_compressed = ImageUtil.compress_image(dataUrl);
    const wasm_blob = new Blob([wasm_compressed], { type: 'image/png' });

    // const file = await imageCompression.getFilefromDataUrl(dataUrl, "");
    // const compressedFile = await imageCompression(file, {
    //   maxWidthOrHeight: 32,
    //   initialQuality: 0.5,
    //   maxIteration: 1,
    //   useWebWorker: false
    // });
    const reader = new FileReader();
    return new Promise(resolve => {
      reader.onload = e => resolve(e.target.result);
      reader.readAsDataURL(wasm_blob);
    });
  }
  catch {
    return dataUrl;
  }
};
