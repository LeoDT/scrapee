import {atom} from 'jotai/vanilla';

export enum NavType {
  Home,
  SiteView,
}

interface Nav {
  id: string;
  link?: string;
  type: NavType;
  removable: boolean;
}

export const navsAtom = atom<Nav[]>([
  {id: 'home', link: '/', type: NavType.Home, removable: false},
]);

export const addNavAtom = atom(null, (get, set, nav: Nav) => {
  set(navsAtom, [...get(navsAtom), nav]);
});

export const removeNavAtom = atom(null, (get, set, nav: Nav) => {
  if (nav.removable) {
    set(
      navsAtom,
      get(navsAtom).filter(n => n !== nav),
    );
  }
});
