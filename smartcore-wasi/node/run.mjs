import fs from 'fs';
import wasiImport from 'wasi'
const { WASI } = wasiImport;

const getStringPointer = (data, instance) => {
  // prepared_string = bytes(string, 'utf-8')
  //   length_of_string = len(prepared_string) + 1
  //   string_ptr = instance.exports.allocate(length_of_string)

  //   memory = instance.exports.memory.uint8_view(string_ptr)
  //   memory[0:length_of_string] = prepared_string
  //   memory[length_of_string] = 0  # C-string terminates by NULL.
  //   return (string_ptr, length_of_string)

  // the `alloc` function returns an offset in
  // the module's memory to the start of the block
  var enc = new TextEncoder(); // always utf-8
  var length = data.length + 1;
  const targetMem = new Uint8Array(length);
  enc.encodeInto(data, targetMem);
  targetMem[length-1] = 0;
  
  var ptr = instance.exports.allocate(length);
  // create a typed `ArrayBuffer` at `ptr` of proper size
  var mem = new Uint8Array(instance.exports.memory.buffer, ptr, length);
  // copy the content of `data` into the memory buffer
  mem.set(targetMem);
  // return the pointer
  return { ptr, len: length };
}

const wasi = new WASI({
  args: [],
  env: process.env,
  preopens: {
    '.': '.'
  }
});

const importObject = { wasi_snapshot_preview1: wasi.wasiImport };

const wasm = await WebAssembly.compile(fs.readFileSync('smartcore_wasi_lib.wasm'));
const instance = await WebAssembly.instantiate(wasm, importObject);
wasi.initialize(instance);
let { ptr: fileToCopyPtr, len: len1 } = getStringPointer("./test1.txt", instance);
let { ptr: targetPtr, len: len2 } = getStringPointer("./test2.txt", instance);

instance.exports.load_model(fileToCopyPtr, targetPtr);
instance.exports.deallocate(fileToCopyPtr, len1);
instance.exports.deallocate(targetPtr, len2);
