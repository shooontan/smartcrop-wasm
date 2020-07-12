import { h, FunctionComponent } from 'preact';
import { useRef, useEffect } from 'preact/hooks';
import { Crop } from '../hooks/useSmartCrop';

type Props = {
  image?: HTMLImageElement;
  crop?: Crop;
};

export const Canvas: FunctionComponent<Props> = ({ image, crop }) => {
  const canvas = useRef<HTMLCanvasElement>();

  useEffect(() => {
    if (image && crop && canvas.current) {
      canvas.current.width = image.width;
      canvas.current.height = image.height;
      const ctx = canvas.current.getContext('2d');
      if (ctx) {
        ctx.fillStyle = 'rgba(0, 0, 0, 0.8)';
        ctx.fillRect(0, 0, canvas.current.width, canvas.current.height);

        ctx.fillStyle = 'rgb(255, 255, 255)';
        ctx.globalCompositeOperation = 'destination-out';
        ctx.fillRect(crop.x, crop.y, crop.width, crop.height);

        ctx.globalCompositeOperation = 'destination-over';
        ctx.drawImage(image, 0, 0);
      }
    }
  }, [image, crop]);

  return <canvas ref={canvas} />;
};
