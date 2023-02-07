import {useEffect, useMemo, startTransition} from 'react';

import {Box, Flex, Heading, Text} from '@chakra-ui/react';
import {atom, Atom, useAtomValue, useSetAtom} from 'jotai';

import {
  currentNavStateAtom,
  SiteNavState,
  updateCurrentNavStateAtom,
} from '../atoms/nav';
import {Page} from '../atoms/site';
import {PageTabs} from './PageTabs';

interface Props {
  pagesAtom: Atom<Page[]>;
  currentPageAtom: Atom<Page | undefined>;
}

export function PageList({pagesAtom, currentPageAtom}: Props): JSX.Element {
  const pages = useAtomValue(pagesAtom);
  const navState = useAtomValue(currentNavStateAtom) as SiteNavState;
  const updateNavState = useSetAtom(updateCurrentNavStateAtom);
  const currentPage = useAtomValue(currentPageAtom);

  useEffect(() => {
    if (!navState.state?.pageId) {
      updateNavState({pageId: pages[0].id, mode: 'detail'});
    }
  }, [pages, navState]);

  return (
    <Flex
      direction="row"
      justifyContent="flex-start"
      alignItems="stretch"
      h="100%">
      <Box w="20%" maxW="360px" flexShrink={0}>
        {pages.map(p => {
          const active = p === currentPage;

          return (
            <Box
              key={p.id}
              p="2"
              cursor="pointer"
              borderBottom="1px"
              borderColor="gray.300"
              bgColor={active ? 'gray.100' : 'white'}
              _hover={active ? {} : {bgColor: 'gray.50'}}
              _last={{borderBottom: 0}}
              onClick={() => {
                startTransition(() => {
                  updateNavState({
                    pageId: p.id,
                    mode: navState.state?.mode || 'detail',
                  });
                });
              }}>
              <Heading as="h4" size="md" m="0">
                {p.name}
              </Heading>
              <Text fontSize="sm" color="gray.500" wordBreak="break-all">
                {p.url ?? p.url_pattern}
              </Text>
            </Box>
          );
        })}
      </Box>
      <Box
        w="1px"
        flexGrow={0}
        borderLeft="1px"
        borderColor="gray.300"
        my="1"
      />
      <Box flexGrow={1}>
        <PageTabs pageAtom={currentPageAtom} />
      </Box>
    </Flex>
  );
}
