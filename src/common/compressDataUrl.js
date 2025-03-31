let wasmInstance = null;

const loadWasmModule = async () => {
  const response = await fetch('src/wasm_png_compression/pkg/wasm_png_compression_bg.wasm');
  const bytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, {});
  return instance;
};

export const compressDataUrl = async dataUrl => {
  try {
    if (!wasmInstance) {
      wasmInstance = await loadWasmModule();
    }
    const { ImageUtil } = wasmInstance.exports;

    const wasm_compressed = ImageUtil.compress_image(dataUrl);
    const wasm_blob = new Blob([wasm_compressed], { type: 'image/png' });

    const reader = new FileReader();
    return new Promise(resolve => {
      reader.onload = e => resolve(e.target.result);
      reader.readAsDataURL(wasm_blob);
    });
  } catch {
    return dataUrl;
  }
};