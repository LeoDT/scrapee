import {createElement, ComponentType, ChangeEvent} from 'react';

import {FormControl, FormHelperText, FormLabel, Input} from '@chakra-ui/react';
import {useAtomValue, useSetAtom} from 'jotai';

import {ArrayFieldAtoms} from '../../utils/form';

interface Props<VItem, TFields> {
  fieldAtom: ArrayFieldAtoms<VItem, TFields>;
  renderItem: () => void;
}

export function ArrayField<VItem, TFields>({
  fieldAtom,
}: Props<VItem, TFields>): JSX.Element {
  const {readFieldsAtom} = useAtomValue(fieldAtom);
  const fields = useAtomValue(readFieldsAtom);

  return <Box>{fields.map(field => {})}</Box>;
}
