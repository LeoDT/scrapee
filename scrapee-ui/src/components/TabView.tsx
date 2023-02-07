import {Suspense, useEffect} from 'react';

import {useAtomValue, useSetAtom} from 'jotai';
import {useParams} from 'react-router-dom';

import {currentNavIdAtom, currentNavAtom, NavType} from '../atoms/nav';
import {Home} from './Home';
import {TabReader} from './TabReader';
import {TabSite} from './TabSite';

export function TabView(): JSX.Element {
  const params = useParams();
  const setNavId = useSetAtom(currentNavIdAtom);
  const currentNav = useAtomValue(currentNavAtom);

  useEffect(() => {
    if (params.navId) {
      setNavId(params.navId);
    }
  }, [params.navId, setNavId]);

  switch (currentNav.type) {
    case NavType.Home:
      return <Home />;
    case NavType.Reader:
      return <TabReader readerId={currentNav.readerId} />;
    case NavType.Site:
      return <TabSite siteId={currentNav.siteId} />;
  }
}
