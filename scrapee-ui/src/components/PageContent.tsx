import {Box, HStack} from '@chakra-ui/react';
import {Atom, useAtomValue} from 'jotai';

import {Page} from '../atoms/site';

interface Props {
  pageAtom: Atom<Page | undefined>;
}

export function PageContent({pageAtom}: Props): JSX.Element {
  const page = useAtomValue(pageAtom);

  return page ? (
    <Box>
      <HStack>
        <Box>Name</Box>
        <Box>{page.name}</Box>
      </HStack>
      <HStack>
        <Box>URL</Box>
        <Box>{page.url}</Box>
      </HStack>
      <HStack>
        <Box>URL Pattern</Box>
        <Box>{page.url_pattern}</Box>
      </HStack>
    </Box>
  ) : null;
}
