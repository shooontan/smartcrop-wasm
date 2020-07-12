import { h, Fragment, FunctionComponent } from 'preact';
import { StateUpdater } from 'preact/hooks';
import { CropOption } from '../hooks/useSmartCrop';

type Props = {
  name: CropOption['module'];
  setModule: StateUpdater<CropOption['module']>;
} & h.JSX.HTMLAttributes<HTMLInputElement>;

export const Radio: FunctionComponent<Props> = (props) => {
  const { name, children, setModule, ...rest } = props;
  return (
    <Fragment>
      <label for={name}>
        <input type="radio" onClick={() => setModule(name)} {...rest} />
        {children}
      </label>
    </Fragment>
  );
};
