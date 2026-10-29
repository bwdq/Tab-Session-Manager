import imageCompression from 'browser-image-compression';

const wasmPath = "./wasm_png_compression/index_bg.wasm";

export const compressDataUrl = async dataUrl => {
  try {
    const file = await imageCompression.getFilefromDataUrl(dataUrl, "");
    // const compressedFile = await imageCompression(file, {
    //   maxWidthOrHeight: 32,
    //   initialQuality: 0.5,
    //   maxIteration: 1,
    //   useWebWorker: false
    // });
    //
    // const reader = new FileReader();
    // return new Promise(resolve => {
    //   reader.onload = e => resolve(e.target.result);
    //   reader.readAsDataURL(compressedFile);
    // });

  }
  catch (error) {
    console.error(error);
    return dataUrl;
  }
};