import { h, Fragment, FunctionComponent } from 'preact';
import { StateUpdater } from 'preact/hooks';
import { CropOption } from '../hooks/useSmartCrop';

type Props = {
  name: keyof CropOption;
  setValue: StateUpdater<CropOption['width'] | CropOption['height']>;
} & h.JSX.HTMLAttributes<HTMLInputElement>;

export const Slider: FunctionComponent<Props> = (props) => {
  const { name, children, setValue, ...rest } = props;
  return (
    <Fragment>
      <label for={name}>{children}</label>
      <input
        id={name}
        type="range"
        min="1"
        max="500"
        onChange={(e) => {
          setValue(parseInt((e.target as HTMLInputElement).value, 10));
        }}
        {...rest}
      />
      {props.value}
    </Fragment>
  );
};
