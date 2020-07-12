import { h, Fragment } from 'preact';
import { useState } from 'preact/hooks';
import { Canvas } from './Canvas';
import { Radio } from './Radio';
import { Slider } from './Slider';
import { useSmartCrop, CropOption } from '../hooks/useSmartCrop';

export const App = () => {
  const [width, setWidth] = useState<CropOption['width']>(100);
  const [height, setHeight] = useState<CropOption['height']>(100);
  const [module, setModule] = useState<CropOption['module']>('wasm');
  const { image, result, time, CropInput } = useSmartCrop();

  return (
    <Fragment>
      <div>
        <CropInput
          opt={{
            width,
            height,
            module,
          }}
        />
      </div>
      <Canvas image={image} crop={result} />
      <div>
        <Radio name="js" checked={module === 'js'} setModule={setModule}>
          JavaScrip
        </Radio>
        <Radio name="wasm" checked={module === 'wasm'} setModule={setModule}>
          WebAssembly
        </Radio>
      </div>
      <div>
        <p>
          <Slider name="width" value={width} setValue={setWidth}>
            Width
          </Slider>
        </p>
        <p>
          <Slider name="height" value={height} setValue={setHeight}>
            Height
          </Slider>
        </p>
      </div>
      <p>{time ? time.toFixed(2) : 0} ms</p>
      <pre>
        <code>{JSON.stringify(result, null, '\t')}</code>
      </pre>
    </Fragment>
  );
};
