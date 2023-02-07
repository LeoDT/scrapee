import {useMemo} from 'react';

import {Tab, TabList, TabPanel, TabPanels, Tabs} from '@chakra-ui/react';
import {Atom, useAtomValue, useSetAtom} from 'jotai';

import {
  currentNavStateAtom,
  SiteNavState,
  updateCurrentNavStateAtom,
} from '../atoms/nav';
import {Page} from '../atoms/site';
import {PageDetail} from './PageDetail';

interface Props {
  pageAtom: Atom<Page | undefined>;
}

export function PageTabs({pageAtom}: Props): JSX.Element | null {
  const navState = useAtomValue(currentNavStateAtom) as SiteNavState;
  const updateNavState = useSetAtom(updateCurrentNavStateAtom);
  const page = useAtomValue(pageAtom);
  const tabIndex = useMemo(
    () => (navState.state?.mode === 'content' ? 1 : 0),
    [navState],
  );

  return page ? (
    <Tabs
      index={tabIndex}
      isLazy
      onChange={(i: number) => {
        updateNavState({pageId: page.id, mode: i === 1 ? 'content' : 'detail'});
      }}>
      <TabList>
        <Tab>Detail</Tab>
        <Tab>Content</Tab>
      </TabList>

      <TabPanels>
        <TabPanel>
          <PageDetail page={page} />
        </TabPanel>
        <TabPanel>
          <div>{page.name} content</div>
        </TabPanel>
      </TabPanels>
    </Tabs>
  ) : null;
}
