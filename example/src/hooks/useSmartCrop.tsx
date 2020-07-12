import { h, FunctionComponent } from 'preact';
import { useState, useEffect, useMemo } from 'preact/hooks';

const smartcrop = {
  js: () => import('smartcrop'),
  wasm: () => import('../../../pkg/smartcrop_wasm'),
};

export type CropOption = {
  width: number;
  height: number;
  module: 'js' | 'wasm';
};

export type Crop = {
  x: number;
  y: number;
  width: number;
  height: number;
};

type CropInput = {
  opt: CropOption;
};

type State = {
  image?: HTMLImageElement;
  result?: Crop;
  time?: number;
};

export function useSmartCrop() {
  const [state, setState] = useState<State>({});

  const CropInput = useMemo(
    (): FunctionComponent<CropInput> => ({ opt }) => {
      const { width, height, module } = opt;

      const run = async (image: HTMLImageElement, opt: CropOption) => {
        if (opt.module === 'js') {
          const data = await runJS(image, opt);
          setState(data);
        }
        if (opt.module === 'wasm') {
          const data = await runWASM(image, opt);
          setState(data);
        }
      };

      useEffect(() => {
        if (state.image) {
          run(state.image, {
            width,
            height,
            module,
          });
        }
      }, [width, height, module]);

      return (
        <input
          type="file"
          accept="image/*"
          onChange={(e) => {
            const { files } = e.target as HTMLInputElement;
            const image = new Image();
            image.onload = async () => {
              await run(image, opt);
            };
            if (files?.[0] instanceof File) {
              image.src = URL.createObjectURL(files[0]);
            }
          }}
        />
      );
    },
    [state.image]
  );

  return {
    image: state.image,
    result: state.result,
    time: state.time,
    CropInput,
  };
}

async function runJS(img: HTMLImageElement, opt: CropOption) {
  const smartcropjs = await smartcrop.js();
  const start = performance.now();
  const result = await smartcropjs.crop(img, {
    width: opt.width,
    height: opt.height,
  });
  const end = performance.now();
  return {
    image: img,
    result: result.topCrop,
    time: end - start,
  };
}

async function runWASM(img: HTMLImageElement, opt: CropOption) {
  const smartcropwasm = await smartcrop.wasm();
  const data = await fetch(img.src)
    .then((res) => res.arrayBuffer())
    .then((res) => new Uint8Array(res));
  const start = performance.now();
  const crop = smartcropwasm.crop(data, opt.width, opt.height);
  const end = performance.now();
  const [x, y, width, height] = crop;
  return {
    image: img,
    result: {
      x,
      y,
      width,
      height,
    },
    time: end - start,
  };
}
