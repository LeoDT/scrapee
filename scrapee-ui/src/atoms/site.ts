import {Atom, atom} from 'jotai';

import {
  PagesResponse,
  Site,
  SiteResponse,
  SitesResponse,
  Page as ServerPage,
  Field,
} from '../server-types';
import {createEntitiesAtom, EntityCache} from '../utils/atomCachedEntity';
import {http} from '../utils/fetch';

export interface Page extends ServerPage {
  fields: Field[];
}

const siteCache = new EntityCache<Site, number>();

export async function fetchSites(): Promise<Site[]> {
  const res = await http.get<SitesResponse>('/sites');

  const sites: Site[] = [];

  res.data.sites.forEach(site => {
    siteCache.set(site.id, site);

    sites.push(site);
  });

  return sites;
}

export async function fetchSite(id: number): Promise<Site> {
  if (siteCache.has(id)) return siteCache.get(id) as Site;

  const res = await http.get<SiteResponse>(`/site/${id}`);

  siteCache.set(res.data.site.id, res.data.site);

  return res.data.site;
}

export async function createSite(name: string): Promise<Site> {
  const res = await http.post<SiteResponse>(`/site`, {
    name,
  });

  siteCache.set(res.data.site.id, res.data.site);

  return res.data.site;
}

const pageCache = new EntityCache<Page, number>();

export async function fetchSitePages(siteId: number): Promise<Page[]> {
  const res = await http.get<PagesResponse>(`/site/${siteId}/pages`);

  const pages: Page[] = [];

  res.data.pages.forEach(({page, fields}) => {
    const result = {...page, fields};

    pages.push(result);

    pageCache.set(result.id, result);
  });

  return pages;
}

export const sitesAtom = createEntitiesAtom(siteCache);
export const pagesAtom = createEntitiesAtom(pageCache);

export function makeSiteAndPageAtom(
  siteId: number,
): [Atom<Site | undefined>, Atom<Page[]>] {
  const siteAtom = atom(get => {
    return get(sitesAtom).find(s => s.id === siteId);
  });

  const sitePagesAtom = atom(get => {
    return get(pagesAtom).filter(p => p.site_id === siteId);
  });

  return [siteAtom, sitePagesAtom];
}
