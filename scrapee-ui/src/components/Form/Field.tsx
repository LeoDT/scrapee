import {createElement, ComponentType, ChangeEvent} from 'react';

import {FormControl, FormHelperText, FormLabel, Input} from '@chakra-ui/react';
import {useAtomValue, useSetAtom} from 'jotai';

import {NormalFieldAtoms} from '../../utils/form';

interface ComponentBasicProps<V> {
  value?: V;
  onChange: (e: V | ChangeEvent<HTMLInputElement>) => void;
}

interface Props<V, P extends ComponentBasicProps<V>> {
  fieldAtom: NormalFieldAtoms<V>;
  component?: ComponentType<P>;
  label?: React.ReactNode;
  helper?: React.ReactNode;
  componentProps?: Partial<Omit<P, 'value' | 'onChange'>>;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function isReactChangeEvent(e: any): e is ChangeEvent<HTMLInputElement> {
  return 'nativeEvent' in e;
}

export function Field<V, P extends ComponentBasicProps<V>>({
  fieldAtom,
  component,
  label,
  helper,
  componentProps,
}: Props<V, P>): JSX.Element {
  const {readAtom, writeAtom} = useAtomValue(fieldAtom);
  const field = useAtomValue(readAtom);
  const write = useSetAtom(writeAtom);

  return (
    <FormControl>
      {label ? <FormLabel>{label}</FormLabel> : null}

      {component
        ? createElement(component, {
            value: field.value,
            onChange: v => {
              if (isReactChangeEvent(v)) {
                write(v.target.value as V);
              } else {
                write(v as V);
              }
            },
            ...componentProps,
          } as P)
        : createElement(Input, {
            value: field.value?.toString() ?? '',
            onChange: e => {
              write(e.currentTarget.value as V);
            },
            ...componentProps,
          })}

      {helper ? <FormHelperText>{helper}</FormHelperText> : null}
    </FormControl>
  );
}
