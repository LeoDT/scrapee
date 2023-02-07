import {atom} from 'jotai';

import {atomWithStorage} from '../utils/atomWithStorage';

export enum NavType {
  Home,
  Site,
  Reader,
}

interface BaseNav {
  id: string;
  name: string;
  link?: string;
  type: NavType;
  removable: boolean;
}

export interface HomeNav extends BaseNav {
  type: NavType.Home;
  removable: false;
}

export interface SiteNav extends BaseNav {
  type: NavType.Site;
  siteId: number;
}

export interface ReaderNav extends BaseNav {
  type: NavType.Reader;
  readerId: number;
}

export type Nav = HomeNav | SiteNav | ReaderNav;

export const HOME_NAV: HomeNav = {
  id: 'HOME_NAV',
  name: 'Home',
  link: '/tab/HOME_NAV',
  type: NavType.Home,
  removable: false,
};

export const navsAtom = atomWithStorage<Nav[]>('navs', [HOME_NAV]);

export const addNavAtom = atom(null, (get, set, nav: Nav) => {
  set(navsAtom, [...get(navsAtom), nav]);
  set(addNavStateAtom, nav);
});

export const removeNavAtom = atom(null, (get, set, nav: Nav) => {
  if (nav.removable) {
    set(
      navsAtom,
      get(navsAtom).filter(n => n !== nav),
    );

    set(removeNavStateAtom, nav);
  }
});

export const currentNavIdAtom = atom<string>(HOME_NAV.id);
export const currentNavAtom = atom(get => {
  const navId = get(currentNavIdAtom);
  const nav = get(navsAtom).find(n => n.id === navId);

  if (nav) {
    return nav;
  }

  throw new Error(`no nav with id: ${navId}`);
});

interface BaseNavState<S> {
  navId: string;
  type: NavType;
  state: S;
}

export type HomeNavStateState = null;
export interface HomeNavState extends BaseNavState<HomeNavStateState> {
  type: NavType.Home;
}

export type TabSitePageMode = 'detail' | 'content';
export type SiteNavStateState = {
  pageId: number;
  mode: TabSitePageMode;
} | null;
export interface SiteNavState extends BaseNavState<SiteNavStateState> {
  type: NavType.Site;
}

export type ReaderNavStateState = null;
export interface ReaderNavState extends BaseNavState<ReaderNavStateState> {
  type: NavType.Reader;
}

export type NavStateState =
  | HomeNavStateState
  | SiteNavStateState
  | ReaderNavStateState;
export type NavState = HomeNavState | SiteNavState | ReaderNavState;

export const navStatesAtom = atomWithStorage<NavState[]>('navStates', [
  {navId: HOME_NAV.id, type: HOME_NAV.type, state: null},
]);
export const addNavStateAtom = atom(null, (get, set, nav: Nav) => {
  const state: NavState = {
    navId: nav.id,
    type: nav.type,
    state: null,
  };

  set(navStatesAtom, [...get(navStatesAtom), state]);
});
export const removeNavStateAtom = atom(null, (get, set, nav: Nav) => {
  const states = get(navStatesAtom);
  const index = states.findIndex(s => s.navId === nav.id);

  if (index >= 0) {
    set(navStatesAtom, [...states.slice(0, index), ...states.slice(index + 1)]);
  }
});

export const currentNavStateAtom = atom(get => {
  const navId = get(currentNavIdAtom);
  const states = get(navStatesAtom);
  const state = states.find(s => s.navId === navId);

  if (state) {
    return state;
  }

  throw new Error(`no nav state for nav with id: ${navId}`);
});

export const updateCurrentNavStateAtom = atom(
  null,
  (get, set, state: NavStateState) => {
    const navState = {
      ...get(currentNavStateAtom),
      state,
    };

    const states = get(navStatesAtom);
    const index = states.findIndex(s => s.navId === navState.navId);

    if (index >= 0) {
      set(navStatesAtom, [
        ...states.slice(0, index),
        navState,
        ...states.slice(index + 1),
      ]);
    }
  },
);
