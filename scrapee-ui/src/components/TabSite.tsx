import {Suspense, useEffect, useMemo, useState} from 'react';

import {Box, Flex, Heading} from '@chakra-ui/react';
import {atom} from 'jotai';

import {currentNavStateAtom} from '../atoms/nav';
import {fetchSite, fetchSitePages, makeSiteAndPageAtom} from '../atoms/site';
import {PageList} from './PageList';

interface Props {
  siteId: number;
}

export function TabSite({siteId}: Props): JSX.Element {
  const [atoms, setAtoms] = useState(() => {
    return makeSiteAndPageAtom(siteId);
  });
  const [, pagesAtom] = atoms;
  const currentPageAtom = useMemo(() => {
    return atom(get =>
      get(pagesAtom).find(p => p.id === get(currentNavStateAtom).state?.pageId),
    );
  }, []);

  useEffect(() => {
    setAtoms(makeSiteAndPageAtom(siteId));

    fetchSite(siteId);
    fetchSitePages(siteId);
  }, [siteId]);

  return (
    <Box flexGrow={1} h="100%">
      <Suspense fallback="loading pages...">
        <PageList pagesAtom={pagesAtom} currentPageAtom={currentPageAtom} />
      </Suspense>
    </Box>
  );
}
