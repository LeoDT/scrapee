import {Box} from '@chakra-ui/react';
import {useAtomValue} from 'jotai';

import {Page} from '../atoms/site';
import {usePageFormAtom} from '../forms/page';
import {Field} from './Form/Field';

interface Props {
  page: Page;
}

export function PageDetail({page}: Props): JSX.Element {
  const [formAtom] = usePageFormAtom(page, [page.id]);
  const form = useAtomValue(formAtom);

  return (
    <Box>
      <Field fieldAtom={form.name} label="Name" />
      <Field fieldAtom={form.url} label="URL" />
      <Field fieldAtom={form.url_pattern} label="URL Pattern" />
    </Box>
  );
}
